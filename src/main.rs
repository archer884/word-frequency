use std::{borrow::Cow, fs, io, process};

use clap::Parser;
use hashbrown::HashMap;
use natural::tokenize::tokenize;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// path to manuscript text
    path: String,

    /// the target word
    word: String,
}

impl Args {
    fn lowercase_word(&self) -> Cow<str> {
        if self.word.bytes().any(|u| !u.is_ascii_lowercase()) {
            Cow::Owned(self.word.to_ascii_lowercase())
        } else {
            Cow::Borrowed(&self.word)
        }
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let word = args.lowercase_word();
    let text = fs::read_to_string(&args.path)?;

    let mut total_count = 0;

    let tokens = tokenize(&text);
    let tokens = tokens
        .into_iter()
        .inspect(|_| total_count += 1)
        .map(|s| s.trim().to_ascii_lowercase());

    let token_count = count_tokens(tokens);
    let target_word_count = token_count.get(&*word).unwrap();

    println!("total word count: {total_count}\n{target_word_count}");
    println!("(values are approximate)");

    Ok(())
}

fn count_tokens(tokens: impl IntoIterator<Item = String>) -> HashMap<String, usize> {
    let mut token_count = HashMap::new();
    for token in tokens {
        *token_count.entry(token).or_default() += 1;
    }
    token_count
}
