use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchTrie {
  pub val: HashMap<char, SearchTrie>
}

impl SearchTrie {
  pub fn new() -> Self {
    Self {
      val: HashMap::new()
    }
  }

  pub fn insert(&mut self, line: &str) {
    let mut pointer = self;
    for c in line.chars() {
      if !pointer.val.contains_key(&c) {
        pointer.val.insert(c, SearchTrie::new());
      }
      
      pointer = pointer.val.get_mut(&c).unwrap();
    }
  }
}