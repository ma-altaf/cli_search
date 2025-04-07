mod search_trie;
mod lines;

use std::{io::{stdin, stdout, Write}, time::Instant};

use search_trie::SearchTrie;
use lines::LINES;

fn main() {
    let mut trie = SearchTrie::new();

    let lines = LINES;

    for line in lines {
        trie.insert(&line);
    }

    println!("{:?}", trie.list());

    let mut engine = trie.engine(4);
    let mut input = String::new();

    loop {
        print!("\nEnter next char: ");
        stdout().flush().unwrap(); // ensures print is displayed before stdin
        stdin().read_line(&mut input).unwrap();
        
        let now1: Instant = Instant::now();
        engine.tp_query(input.trim().chars().last().unwrap());
        let elp1 = now1.elapsed().as_millis();

        let now2: Instant = Instant::now();
        let options = engine.tp_options();
        let elp2 = now2.elapsed().as_millis();
        
        println!("history:\n {:?}", options);

        println!("query time: {:?}", elp1);
        println!("options time: {:?}", elp2);
    }
}
