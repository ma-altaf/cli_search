mod search_trie;

use search_trie::SearchTrie;

fn main() {
    let mut trie = SearchTrie::new();

    let lines = [
        "line 1",
        "line 2",
        "not a line"
    ];

    for line in lines {
        trie.insert(line);
    }
    
}
