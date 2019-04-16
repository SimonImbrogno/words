mod trie;
use trie::Trie;

use std::fs;
use std::env;

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

  let mut unique_word_count = 0;

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

        for word in file_contents.split_whitespace()
        {
          let string = word.to_string().replace(".", "");
          if trie.put(string) == true { unique_word_count += 1 };
        }
      }
    }
  }

  trie.print_contents();
  println!("{} unique words.", unique_word_count);

  return Ok(());
}
