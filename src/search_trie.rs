use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchTrie {
  pub ref_count: u32,
  pub val: HashMap<char, Self>
}

impl SearchTrie {
  pub fn new() -> Self {
    Self {
      ref_count: 0,
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
      pointer.ref_count += 1;
    }
  }

  pub fn remove(&mut self, line: &str) {
    let mut pointer = self;
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

  pub fn engine(& self) -> Engine {
    Engine::new(self)
  }
}

pub struct Engine<'a> {
  history: Vec<Vec<&'a SearchTrie>>
}

impl<'a> Engine<'a> {
  pub fn new(search_trie: &'a SearchTrie) -> Self {
    Self { history: vec![vec![search_trie]] }
  }

  pub fn query(self, character: char) {
    println!("{}", character);
  }
}