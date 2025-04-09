mod search_trie;
mod lines;

use std::{io::{stdin, stdout, Write}, time::Instant};

use search_trie::{SearchEngine, SearchTrie};
use lines::LINES;

fn main() {
    let mut trie = SearchTrie::new();

    let lines = LINES;
    // let lines = ["line 1", "line 2", "not a line"];

    for line in lines {
        trie.insert(&line);
    }

    // println!("{:?}", trie.list());

    let query = "ABB";

    // interactive_engine(trie.tp_engine(1024));

    engine_performance(&trie, &query);
    t_engine_performance(&trie, &query);
    tp_engine_performance(&trie, &query, 4);
}

fn interactive_engine<T>(engine: T) 
where T: SearchEngine  {
    let mut engine = engine;
    let mut input = String::new();

    loop {
        print!("\nEnter next char: ");
        stdout().flush().unwrap(); // ensures print is displayed before stdin
        stdin().read_line(&mut input).unwrap();
        
        let now1: Instant = Instant::now();
        engine.query(input.trim().chars().last().unwrap());
        let elp1 = now1.elapsed().as_millis();

        let now2: Instant = Instant::now();
        let options = engine.options();
        let elp2 = now2.elapsed().as_millis();
        
        println!("history:\n {:?}", options);

        println!("query time: {:?}", elp1);
        println!("options time: {:?}", elp2);
    }
}

fn engine_performance(trie: &SearchTrie, query: &str) {
    let query_len: u128 = query.len().try_into().unwrap();
    let mut engine = trie.engine();

    let mut avg_query_time = 0;
    let mut avg_opt_time = 0;

    for c in query.chars() {
        let now1: Instant = Instant::now();
        engine.query(c);
        let query_time = now1.elapsed().as_millis();
    
        let now2: Instant = Instant::now();
        engine.options();
        let opt_time = now2.elapsed().as_millis();

        avg_query_time += query_time;
        avg_opt_time += opt_time;
    }

    println!("\nEngine performance:");
    println!("query time: {:?}", avg_query_time / query_len);
    println!("options time: {:?}", avg_opt_time / query_len);
    stdout().flush().unwrap();
}

fn t_engine_performance(trie: &SearchTrie, query: &str) {
    let query_len: u128 = query.len().try_into().unwrap();
    let mut engine = trie.t_engine();

    let mut avg_query_time = 0;
    let mut avg_opt_time = 0;

    for c in query.chars() {
        let now1: Instant = Instant::now();
        engine.query(c);
        let query_time = now1.elapsed().as_millis();
    
        let now2: Instant = Instant::now();
        engine.options();
        let opt_time = now2.elapsed().as_millis();

        avg_query_time += query_time;
        avg_opt_time += opt_time;
    }

    println!("\nTEngine performance:");
    println!("query time: {:?}", avg_query_time / query_len);
    println!("options time: {:?}", avg_opt_time / query_len);
    stdout().flush().unwrap();
}

fn tp_engine_performance(trie: &SearchTrie, query: &str, thread_count: usize) {
    let query_len: u128 = query.len().try_into().unwrap();
    let mut engine = trie.tp_engine(thread_count);

    let mut avg_query_time = 0;
    let mut avg_opt_time = 0;

    for c in query.chars() {
        let now1: Instant = Instant::now();
        engine.query(c);
        let query_time = now1.elapsed().as_millis();
    
        let now2: Instant = Instant::now();
        engine.options();
        let opt_time = now2.elapsed().as_millis();

        avg_query_time += query_time;
        avg_opt_time += opt_time;
    }

    println!("\nTPEngine performance:");
    println!("query time: {:?}", avg_query_time / query_len);
    println!("options time: {:?}", avg_opt_time / query_len);
    stdout().flush().unwrap();
}