extern crate rand;

use std::vec::Vec;
use rand::Rng;

const CHARSET_SIZE: u32 = 27;

#[derive(Debug)]
pub struct MarkovTable
{
  size: usize,
  table: Vec<ProbabilityTable>,
}

#[derive(Debug, Default, Clone)]
struct ProbabilityTable
{
  p: [u32; CHARSET_SIZE as usize],
}

fn char_to_table_indx(c: char) -> Result<usize, ()>
{
  let as_int = c as usize;
  return match as_int
  {
    32        => Ok(0),
    97...122  => Ok(as_int - 96),
    _         => Err(()),
  };
}


fn table_indx_to_char(indx: usize) -> Result<char, ()>
{
  return match indx
  {
    0      => Ok(32 as u8 as char),
    1...26 => Ok((indx + 96) as u8 as char),
    _      => Err(()),
  };
}

impl MarkovTable
{
  // Constructor
  pub fn new(depth: u32) -> MarkovTable
  {
    let computed_size = (CHARSET_SIZE.pow(depth)) as usize;
    return MarkovTable
    {
      size: computed_size,
      table: vec![ProbabilityTable { p: Default::default() } ; computed_size],
    };
  }

  pub fn record(&mut self, key: &String) -> Result<(), ()>
  {
    let mut prepared_str = String::from("   ");
    prepared_str.push_str(key);
    prepared_str.push_str(" ");

    let mut key_indx = 0;
    while (key_indx + 3) < prepared_str.len()
    {
      let mut test_string = prepared_str[key_indx..key_indx+4].to_string();

      let hi  = char_to_table_indx(test_string.remove(0))? * CHARSET_SIZE as usize * CHARSET_SIZE as usize;
      let mid = char_to_table_indx(test_string.remove(0))? * CHARSET_SIZE as usize;
      let lo  = char_to_table_indx(test_string.remove(0))?;
      let table_indx = hi + mid + lo;

      let final_indx = char_to_table_indx(test_string.remove(0))?;
      self.table[table_indx].p[final_indx] += 1;

      key_indx += 1;
    }

    Ok(())
  }

  pub fn generate(&self) -> Result<String, ()>
  {
    let mut result = String::from("   ");

    let mut curr_indx = 0;
    loop
    {
      let mut test_string = result[curr_indx..curr_indx+3].to_string();
      curr_indx += 1;

      // println!("test_string {}", test_string);
      let hi  = char_to_table_indx(test_string.remove(0))? * CHARSET_SIZE as usize * CHARSET_SIZE as usize;
      let mid = char_to_table_indx(test_string.remove(0))? * CHARSET_SIZE as usize;
      let lo  = char_to_table_indx(test_string.remove(0))?;
      let table_indx = hi + mid + lo;

      let mut sum = 0;
      for i in 0..26 { sum += self.table[table_indx].p[i]; }

      // println!("sum {}", sum);
      let mut rng = rand::thread_rng();
      let mut random = rng.gen_range(0, sum+1) as i32;
      // println!("random {}", random);

      let mut next_char_indx = 0;
      // println!("the table {:?}", self.table[table_indx]);
      for i in 0..27
      {
        next_char_indx = i;
        random -= self.table[table_indx].p[i] as i32;
        if random <= 0 { break; }
      }
      // println!("next_char_indx {}", next_char_indx);

      let next_char = table_indx_to_char(next_char_indx)?;
      if next_char == ' ' { break; }

      result.push(next_char);
      // println!("result {}", result);
    }

    result.remove(0);
    result.remove(0);
    result.remove(0);
    Ok(result)
  }
}
