mod Search_trie;

use Search_trie::SearchTrie;

fn main() {
    let mut root = SearchTrie::new();

    let lines = [
        "line 1",
        "line 2",
        "not a line"
    ];

    for line in lines {
        root.insert(line);
    }
}
