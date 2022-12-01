use std::{fs, io, process};

use clap::Parser;
use hashbrown::HashMap;
use natural::tokenize::tokenize;
use unicase::UniCase;

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
        .map(|s| s.trim());

    let token_count = count_tokens(tokens);
    let target_word_counts = args.target_words.iter().map(|s| {
        let word = UniCase::new(s.as_ref());
        let count = token_count.get(&word).copied().unwrap_or_default();
        (word, count)
    });

    println!("total word count: {total_count}");
    for (word, count) in target_word_counts {
        println!("{word}: {count}");
    }

    Ok(())
}

fn count_tokens<'a>(tokens: impl IntoIterator<Item = &'a str>) -> HashMap<UniCase<&'a str>, usize> {
    let mut token_count = HashMap::new();
    for token in tokens {
        *token_count.entry(UniCase::new(token)).or_default() += 1;
    }
    token_count
}
