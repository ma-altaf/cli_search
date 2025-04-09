use std::{sync::mpsc::channel, thread::scope};

use super::engine::{build_path, expand, traverse};
use super::SearchEngine;
use super::{engine::HistoryNode, TrieNode};

pub struct TEngine<'a> {
  // store the different current Trie_nodes reached and PathNodes to build the line with history for backspace
  history: Vec<Vec<HistoryNode<'a>>>,
}

impl<'a> TEngine<'a> {
  pub fn new(root: &'a TrieNode) -> Self {
    Self { 
      history:  vec![vec![HistoryNode { node: root, path: Vec::new() }]],
    }
  }
}

impl<'a> SearchEngine for TEngine<'a> {
    
  fn query(&mut self, input: char) {
    if input.eq(&'*') {
      if self.history.len() > 1 {
        self.history.pop();
      }
      return;
    }

    let curr = self.history.last().unwrap().to_vec();
    let mut next = Vec::new();
    let (tx, tr) = channel();

    scope(|s| {
      let mut threads = Vec::new();
      for HistoryNode {node, path } in curr {
        let tx_c = tx.clone();
        threads.push( s.spawn(move || {
          tx_c.send(
            traverse(node, input, None).iter_mut().map(|hp| {
              let mut t_path = path.to_vec();
              t_path.append(&mut hp.path);
              HistoryNode {node: hp.node, path: t_path }
            }).collect()
          ).unwrap();
        }));
      }

      for _ in threads {
        next.append(&mut tr.recv().unwrap());
      }
    });
    
    self.history.push(next);
  }

  fn options(&self) -> Vec<String> {
    let mut options_list = Vec::new();

    if let Some(nodes) = self.history.last() {
      scope(|s| {
        let mut threads = Vec::new();
        let (tx, tr) = channel();

        for HistoryNode { node, path } in nodes {
          let tx_c = tx.clone();
          threads.push(
            s.spawn(move || {
              let path1 = build_path(path);
              let mut local_opts = Vec::new();
  
              let path2_list = expand(node);
              if path2_list.len() == 0 {
                local_opts.push(path1);
              } else {
                path2_list.iter().for_each(|path2| {
                  local_opts.push(format!("{}{}", path1, path2));
                });
              }
  
              tx_c.send(local_opts).unwrap();
            })
          );

        }

        for _ in threads {
          options_list.append(&mut tr.recv().unwrap());
        }
      });

    }

    options_list
  }
}