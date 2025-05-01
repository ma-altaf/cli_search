use super::TrieNode;

pub trait SearchEngine {
  fn query(&mut self, input: char) -> ();
  fn options(&self) -> Vec<String>;
}

#[derive(Debug)]
pub struct PathNode {
  pub val: char,
  pub in_query: bool
}

impl Clone for PathNode {
    fn clone(&self) -> Self {
        Self { val: self.val.clone(), in_query: self.in_query.clone() }
    }
}

pub(super) fn traverse<'a>(node: &'a TrieNode, target: char, source: Option<&PathNode>) -> Vec<HistoryNode<'a>> {
  let mut paths: Vec<HistoryNode<'a>> = Vec::new();

  for next in &node.val {
    if next.0 == &target {
      paths.push(HistoryNode { 
        node: node.val.get(&target).unwrap(),
        path: if let Some(s) = source { vec![s.clone(), PathNode { val: *next.0, in_query: true }] } 
          else { vec![PathNode { val: *next.0, in_query: true }] }
      });
    } else {
      let res = traverse(next.1, target, Some(&PathNode { val: *next.0, in_query: false }));
      
      for mut r in res {
        let mut p = if let Some(s) = source {vec![s.clone()]} else { Vec::new() };
        p.append(&mut r.path);
        paths.push(HistoryNode { node: r.node, path: p });
      }
    }
  }

  return paths
}

pub(super) fn build_path(path: &Vec<PathNode>) -> String {
  path.iter().fold(String::new(), |mut acc, PathNode { val, in_query }| {
    let next_char = match in_query {
      true => &format!("-{}-", val),
      false => &format!("{}", val),
    };
    acc.push_str(next_char);
    acc
  })
}

pub(super) fn expand(node: &TrieNode) -> Vec<String> {
  let mut res = Vec::new();

  for (c, node) in &node.val {
    let path = c.to_string();
    let expansion = expand(node);
    
    if node.end {
      res.push(path.clone());
    }
    
    if expansion.len() > 0 {
      expansion.iter().for_each(|v| {
        res.push(format!("{}{}", path, v));
      });
    }
  }

  res
}

#[derive(Debug)]
pub(super) struct HistoryNode<'a> {
  pub node: &'a TrieNode,
  pub path: Vec<PathNode>
}

impl<'a> Clone for HistoryNode<'a> {
    fn clone(&self) -> Self {
        Self { node: self.node, path: self.path.clone() }
    }
}

pub struct Engine<'a> {
  // store the different current Trie_nodes reached and PathNodes to build the line with history for backspace
  history: Vec<Vec<HistoryNode<'a>>>,
}

impl<'a> Engine<'a> {
  pub fn new(root: &'a TrieNode) -> Self {
    Self { 
      history:  vec![vec![HistoryNode { node: root, path: Vec::new() }]],
    }
  }  
}

impl<'a> SearchEngine for Engine<'a> {
  fn query(&mut self, input: char) {
    if input.eq(&'*') {
      if self.history.len() > 1 {
        self.history.pop();
      }
      return;
    }

    let curr = self.history.last().unwrap().to_vec();
    let mut next: Vec<HistoryNode> = Vec::new();

    for HistoryNode {node, path } in curr {
      next.append(&mut traverse(node, input, None).iter_mut().map(|hp| {
        let mut t_path = path.to_vec();
        t_path.append(&mut hp.path);
        HistoryNode {node: hp.node, path: t_path }
      }).collect());
    }
    
    self.history.push(next);
  }

  fn options(&self) -> Vec<String> {
    let mut options_list = Vec::new();
    if let Some(nodes) = self.history.last() {
      for HistoryNode { node, path } in nodes {
        let path1 = build_path(path);
        let path2_list = expand(node);
        
        if node.end {
          options_list.push(path1.clone());
        }

        path2_list.iter().for_each(|path2| {
          options_list.push(format!("{}{}", path1, path2));
        });
      }
    }

    options_list
  }
}