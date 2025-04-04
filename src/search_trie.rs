use std::{collections::HashMap, time::Instant};

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

  pub fn engine(&self) -> Engine {
    Engine::new(&self.root)
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

fn traverse<'a>(node: &'a TrieNode, target: char, path: &mut Vec<PathNode>) -> Vec<(&'a TrieNode, Vec<PathNode>)> {
  let mut options: Vec<(&'a TrieNode, Vec<PathNode>)> = Vec::new();

  for next in &node.val {
    if next.0 == &target {
      path.push(PathNode { val: *next.0, in_query: true });
      options.push((node.val.get(&target).unwrap(), path.to_vec()));
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
    let fut = expand(node);
    
    if fut.len() > 0 {
      fut.iter().for_each(|v| {
        res.push(format!("{}{}", path, v));
      });
    } else {
      res.push(path);
    }
  }

  res
}

pub struct Engine<'a> {
  // store the different current Trie_nodes reached and PathNodes to build the line with history for backspace
  history: Vec<Vec<(&'a TrieNode, Vec<PathNode>)>>
}

impl<'a> Engine<'a> {
  pub fn new(root: &'a TrieNode) -> Self {
    Self { history:  vec![vec![(root, Vec::new())]]}
  }

  pub fn query(&mut self, input: char) {
    let curr = self.history.pop().unwrap();
    let mut next: Vec<(&'a TrieNode, Vec<PathNode>)> = Vec::new();

    for (node, mut path) in curr {
      next.append(&mut traverse(node, input, &mut path));
    }

    self.history.push(next);
  }

  pub fn options(&self) -> Vec<String> {
    let mut options_list = Vec::new();
    if let Some(nodes) = self.history.last() {
      for (node, path) in nodes {
        let path1 = build_path(path);
        expand(node).iter().for_each(|path2| {
          options_list.push(format!("{}{}", path1, path2));
        });
      }
    }

    options_list
  }
}