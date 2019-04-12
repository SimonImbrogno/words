use std::fs;
use std::env;

fn read_file(path: &std::path::Path) -> String
{
  println!("{:?}", path.file_name().unwrap());
  let data = fs::read_to_string(path).expect("Unable to read file");

  return data;
}

fn find_max_counter(list: &Vec<Counter>) -> (i32, usize)
{
  let mut max_indx = 0;
  let mut max = list[max_indx].count;

  for (indx, entry) in list.iter().enumerate()
  {
    if entry.count > max
    {
      max = entry.count;
      max_indx = indx;
    }
  }

  return (max, max_indx);
}

#[derive(Clone, Debug)]
struct Counter
{
  label: char,
  count: i32,
}

fn count_chars(string: &String, counters: &mut Vec<Counter>)
{
  for c in string.chars()
  {
    let as_integer = c as i32;
    match as_integer
    {
      65...90   => counters[(as_integer - 65) as usize].count += 1,
      97...122  => counters[(as_integer - 97) as usize].count += 1,
      _         => (), //Do nothin!
    };
  }
}

fn draw_graph(letter_counts: &Vec<Counter>)
{
  const MAX_HEIGHT: i32 = 40;

  let (max_count, _) = find_max_counter(&letter_counts);
  let step = max_count / MAX_HEIGHT;

  let mut row = String::new();

  for row_indx in (0..MAX_HEIGHT).rev()
  {
    for counter_indx in 0..26
    {
      if counter_indx > 0 { row.push_str("  ") }

      if row_indx > 0
      {
        let bar_reaches_row = (letter_counts[counter_indx].count / step) >= row_indx;
        if bar_reaches_row { row.push_str("███") }
        else               { row.push_str("   ") }
      }
      else
      {
        row.push_str(format!(" {} ", letter_counts[counter_indx].label).as_str());
      }
    }

    println!("{}", row);
    row.clear();
  }
}

fn main() -> Result<(), Box<std::error::Error>>
{
  let mut path = env::current_dir().unwrap();
  path.push("inputs");

  let mut counters = vec![Counter{ label: ' ', count: 0}; 26];
  for i in 0..26 { counters[i].label = (65 + i) as u8 as char }

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
        count_chars(&file_contents, &mut counters);

      }
    }
  }

  println!();
  counters.sort_by(|a: &Counter, b: &Counter| b.count.cmp(&a.count));
  draw_graph(&counters);

  return Ok(());
}
