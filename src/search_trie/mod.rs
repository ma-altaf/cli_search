use std::{collections::HashMap, time::Instant};

mod engine;
mod t_engine;
mod tp_engine;

use engine::Engine;
use t_engine::TEngine;
use tp_engine::TPEngine;
pub use engine::SearchEngine;

#[derive(Debug)]
pub struct TrieNode {
  ref_count: u32,
  pub val: HashMap<char, Self>,
  pub end: bool
}

impl TrieNode {
  pub fn new() -> Self {
    Self {
      ref_count: 0,
      val: HashMap::new(),
      end: false
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
    pointer.end = true;
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

  pub fn engine(&self) -> Engine {
    Engine::new(&self.root)
  }

  pub fn t_engine(&self) -> TEngine {
    TEngine::new(&self.root)
  }

  pub fn tp_engine(&self, thread_count: usize) -> TPEngine {
    TPEngine::new(&self.root, thread_count)
  }
}
