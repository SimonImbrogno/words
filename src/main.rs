use std::fs;
use std::env;

#[derive(Copy, Clone, Debug)]
struct CharCounter
{
  label: char,
  count: i32,
}

fn read_file(path: &std::path::Path) -> String
{
  println!("Attempting to read file:\n {:?}\n", path);
  let data = fs::read_to_string(path).expect("Unable to read file");

  return data;
}

fn main()
{
  let mut path = env::current_dir().expect("Unable to get current directory");
  path.push("foo.txt");

  let file_contents = read_file(&path);
  println!("Contents:\n{}", file_contents);

  let mut counters = [CharCounter{ count:0, label: 'a' }; 128];
  for i in 0..128
  {
    counters[i] = CharCounter { label: i as u8 as char, count: 0 };
  }

  for c in file_contents.chars()
  {
    let indx = c as u8 as usize;
    counters[indx].count += 1;
  }

  for counter in counters.iter()
  {
    if (counter.label as u8) < 32 || counter.count == 0
    {
      continue;
    }

    println!("{}: {}", counter.label, counter.count);
  }
}
