#![allow(dead_code)]

extern crate regex;

mod trie;
use trie::Trie;

mod markov;
use markov::MarkovTable;

use std::fs;
use std::env;
use regex::Regex;

fn read_file(path: &std::path::Path) -> String
{
  println!("{:?}", path.file_name().unwrap());
  let data = fs::read_to_string(path).expect("Unable to read file");

  return data;
}

fn main() -> Result<(), Box<std::error::Error>>
{
  let mut trie = Trie::new();

  let mut path = env::current_dir().unwrap();
  path.push("inputs");

  let hyphens_re   = Regex::new(r"[-]").unwrap();
  let non_alpha_re = Regex::new(r"[^a-zA-Z\s]").unwrap(); //Leave whitespace!

  println!("Reading files...");
  if path.is_dir()
  {
    let dir_contents = fs::read_dir(&path)?;

    for entry in dir_contents
    {
      let file_path = entry?.path();
      if !file_path.is_dir()
      {
        let file_contents = read_file(&file_path);
        let file_contents = hyphens_re.replace_all(&file_contents, " ");
        let file_contents = non_alpha_re.replace_all(&file_contents, "");

        for word in file_contents.split_whitespace()
        {
          if word.len() > 0 { trie.put(word.to_string()); }
        }
      }
    }
  }

  // trie.print_contents();
  println!("\n\n{} unique words.", trie.get_count());

  let mut mt = MarkovTable::new(3);
  for word in trie.get_contents() { mt.record(&word); }

  for i in 0..1000 {
    let result = mt.generate().unwrap();
    if result.len() > 3 { println!("{}", result); }
  }

  return Ok(());
}
