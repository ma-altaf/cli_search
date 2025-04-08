use std::sync::mpsc::channel;

use threadpool::ThreadPool;
use threadpool_scope::scope_with;

use super::{engine::{HistoryNode, PathNode}, SearchEngine, TrieNode};

// TODO: use threadpool
fn traverse<'a>(node: &'a TrieNode, target: char, path: &mut Vec<PathNode>) -> Vec<HistoryNode<'a>> {
  let mut options: Vec<HistoryNode> = Vec::new();

  for next in &node.val {
    if next.0 == &target {
      path.push(PathNode { val: *next.0, in_query: true });
      options.push(HistoryNode { node: node.val.get(&target).unwrap(), path: path.to_vec() });
    } else {
      path.push(PathNode { val: *next.0, in_query: false });
      options.append(&mut traverse(next.1, target, path));
    }
    path.pop();
  }

  return options
}

// TODO: use threadpool
fn build_path(path: &Vec<PathNode>) -> String {
  path.iter().fold(String::new(), |mut acc, PathNode { val, in_query }| {
    let next_char = match in_query {
      true => &format!("-{}-", val),
      false => &format!("{}", val),
    };
    acc.push_str(next_char);
    acc
  })
}

// TODO: use threadpool
fn expand(node: &TrieNode) -> Vec<String> {
  let mut res = Vec::new();

  for (c, node) in &node.val {
    let path = c.to_string();
    let expansion = expand(node);
    
    if expansion.len() > 0 {
      expansion.iter().for_each(|v| {
        res.push(format!("{}{}", path, v));
      });
    } else {
      res.push(path);
    }
  }

  res
}

pub struct TPEngine<'a> {
  // store the different current Trie_nodes reached and PathNodes to build the line with history for backspace
  history: Vec<Vec<HistoryNode<'a>>>,
  threads: ThreadPool
}

impl<'a> TPEngine<'a> {
  pub fn new(root: &'a TrieNode, thread_count: usize) -> Self {
    Self { 
      history:  vec![vec![HistoryNode { node: root, path: Vec::new() }]],
      threads: ThreadPool::new(thread_count)
    }
  }
  
}

impl<'a> SearchEngine for TPEngine<'a> {  
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

    scope_with(&self.threads, |s| {
      let mut threads = Vec::new();
      for HistoryNode {node, mut path } in curr {
        let tx_c = tx.clone();
        threads.push( s.execute(move || {
          tx_c.send(traverse(node, input, &mut path)).unwrap();
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
      scope_with(&self.threads ,|s| {
        let mut threads = Vec::new();
        let (tx, tr) = channel();

        for HistoryNode { node, path } in nodes {
          let tx_c = tx.clone();
          threads.push(
            s.execute(move || {
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