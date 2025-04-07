use std::{collections::HashMap, sync::mpsc::channel, thread::scope, time::Instant};

use threadpool::ThreadPool;
use threadpool_scope::scope_with;

#[derive(Debug)]
pub struct TrieNode {
  ref_count: u32,
  pub val: HashMap<char, Self>
}

impl TrieNode {
  pub fn new() -> Self {
    Self {
      ref_count: 0,
      val: HashMap::new()
    }
  }
}

#[derive(Debug)]
pub struct SearchTrie {
  root: TrieNode
}

fn list_solver(curr: &TrieNode, path: &mut String, res: &mut Vec<String>) {
  if curr.val.is_empty() {
    return res.push(path.to_owned());
  }

  for (c, next) in &curr.val {
    path.push_str(&c.to_string());
    list_solver(&next, path, res);
    path.pop();
  }
}

impl SearchTrie {
  pub fn new() -> Self {
    Self { root: TrieNode::new() }
  }

  pub fn list(&self) -> Vec<String> {
    let timer = Instant::now();
    let mut res: Vec<String> = Vec::new();

    list_solver(&self.root,&mut String::new(), &mut res);

    println!("time elapsed: {}", timer.elapsed().as_millis());
    return res;
  }

  pub fn insert(&mut self, line: &str) {
    let mut pointer = &mut self.root;
    for c in line.chars() {
      if !pointer.val.contains_key(&c) {
        pointer.val.insert(c, TrieNode::new());
      }
      
      pointer = pointer.val.get_mut(&c).unwrap();
      pointer.ref_count += 1;
    }
  }

  pub fn remove(&mut self, line: &str) {
    let mut pointer = &mut self.root;
    for c in line.chars() {
      if let Some(node) = pointer.val.get_mut(&c) {
        if node.ref_count == 1 {
          pointer.val.remove(&c);
        } else {
          pointer = pointer.val.get_mut(&c).unwrap();
          pointer.ref_count -= 1;
        }
      }
    }
  }

  pub fn engine(&self, thread_count: usize) -> Engine {
    Engine::new(&self.root, thread_count)
  }
}

#[derive(Debug)]
pub struct PathNode {
  val: char,
  in_query: bool
}

impl Clone for PathNode {
    fn clone(&self) -> Self {
        Self { val: self.val.clone(), in_query: self.in_query.clone() }
    }
}

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

#[derive(Debug)]
struct HistoryNode<'a> {
  node: &'a TrieNode,
  path: Vec<PathNode>
}

impl<'a> Clone for HistoryNode<'a> {
    fn clone(&self) -> Self {
        Self { node: self.node, path: self.path.clone() }
    }
}

pub struct Engine<'a> {
  // store the different current Trie_nodes reached and PathNodes to build the line with history for backspace
  history: Vec<Vec<HistoryNode<'a>>>,
  threads: ThreadPool
}

impl<'a> Engine<'a> {
  pub fn new(root: &'a TrieNode, thread_count: usize) -> Self {
    Self { 
      history:  vec![vec![HistoryNode { node: root, path: Vec::new() }]],
      threads: ThreadPool::new(thread_count)
    }
  }

  pub fn query(&mut self, input: char) {
    if input.eq(&'*') {
      if self.history.len() > 1 {
        self.history.pop();
      }
      return;
    }

    let curr = self.history.last().unwrap().to_vec();
    let mut next: Vec<HistoryNode> = Vec::new();

    for HistoryNode {node, mut path } in curr {
      next.append(&mut traverse(node, input, &mut path));
    }
    
    self.history.push(next);
  }

  pub fn t_query(&mut self, input: char) {
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
      for HistoryNode {node, mut path } in curr {
        let tx_c = tx.clone();
        threads.push( s.spawn(move || {
          tx_c.send(traverse(node, input, &mut path)).unwrap();
        }));
      }

      for _ in threads {
        next.append(&mut tr.recv().unwrap());
      }
    });
    
    self.history.push(next);
  }

  pub fn tp_query(&mut self, input: char) {
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

  pub fn options(&self) -> Vec<String> {
    let mut options_list = Vec::new();
    if let Some(nodes) = self.history.last() {
      for HistoryNode { node, path } in nodes {
        let path1 = build_path(path);
        let path2_list = expand(node);
        if path2_list.len() == 0 {
          options_list.push(path1);
        } else {
          path2_list.iter().for_each(|path2| {
            options_list.push(format!("{}{}", path1, path2));
          });
        }
      }
    }

    options_list
  }

  pub fn t_options(&self) -> Vec<String> {
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

  pub fn tp_options(&self) -> Vec<String> {
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