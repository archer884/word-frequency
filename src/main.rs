use std::{fs, io, process};

use clap::Parser;
use hashbrown::HashMap;
use natural::tokenize::tokenize;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// path to manuscript text
    path: String,

    /// target words
    target_words: Vec<String>,
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let text = fs::read_to_string(&args.path)?;

    let mut total_count = 0;

    let tokens = tokenize(&text);
    let tokens = tokens
        .into_iter()
        .inspect(|_| total_count += 1)
        .map(|s| s.trim().to_ascii_lowercase());

    let token_count = count_tokens(tokens);
    let target_word_counts = args.target_words.iter().map(|s| {
        let word = s.to_ascii_lowercase();
        let count = token_count.get(&word).copied().unwrap_or_default();
        (word, count)
    });

    println!("total word count: {total_count}");
    for (word, count) in target_word_counts {
        println!("{word}: {count}");
    }

    Ok(())
}

fn count_tokens(tokens: impl IntoIterator<Item = String>) -> HashMap<String, usize> {
    let mut token_count = HashMap::new();
    for token in tokens {
        *token_count.entry(token).or_default() += 1;
    }
    token_count
}
