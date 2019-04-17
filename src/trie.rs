use std::fmt;

#[derive(Debug)]
pub struct Trie
{
  root: bool,
  terminal: bool,
  label: char,
  count: u32,
  next: [Option<Box<Trie>>; 26],
}

impl fmt::Display for Trie {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} '{}' ({}) -> [", if self.terminal { "x" } else {" "}, self.label, self.count)?;

    let mut first = true;
    for elem in self.next.iter()
    {
      if let Some(sub) = elem
      {
        let label = sub.label;
        if first
        {
          first = false;
          write!(f, "{}", label)?;
        }
        else
        {
          write!(f, ", {}", label)?;
        }
      }
    }

    return write!(f, "]");
  }
}

impl Trie
{
  // Constructor (Root)
  pub fn new() -> Trie {
    let mut new_trie = Trie::new_branch(' ');
    new_trie.root = true;
    return new_trie;
  }

  // Constructor
  fn new_branch(label: char) -> Trie {
    Trie {
      count: 0,
      root: false,
      terminal: false,
      label: label,
      next: Default::default(),
    }
  }

  fn get_sub(&mut self, label: char) -> &mut Trie
  {
    let as_int = label as u32;
    let indx = match as_int
    {
      97...122  => (as_int - 97) as usize,
      _         => self.next.len(),
    };

    // If this incoming char is a crap value, return the parent for a cheeky little skip.
    // i.e: we begin inserting again from the parent at the next char!
    if indx == self.next.len() { return self; };

    if let None = self.next[indx]
    {
      self.next[indx] = Some(Box::new(Trie::new_branch(label)));
    }

    return &mut *(self.next[indx].as_mut().unwrap());
  }

  pub fn put(&mut self, mut key: String) -> bool
  {
    if key.len() > 0
    {
      let as_int = (&mut key).remove(0) as u8;
      let first_char = match as_int
      {
        65...90   => (as_int + 32) as char,
        97...122  => as_int as char,
        _         => ' ',
      };

      let sub_tree = self.get_sub(first_char);

      let result = sub_tree.put(key);
      if result { self.count += 1; }
      return result;
    }
    else
    {
      if self.terminal == false
      {
        self.terminal = true;
        self.count += 1;
        return true;
      }
      else { return false; }
    }
  }

  pub fn get_count(&self) -> u32
  {
    return self.count;
  }

  pub fn print_self(&self)
  {
    println!("{}", self);
    for elem in self.next.iter()
    {
      if let Some(sub) = elem
      {
        sub.print_self();
      }
    }
  }

  pub fn print_contents(&self)
  {
    self.print_contents_recur(true, &mut String::with_capacity(256));
  }

  fn print_contents_recur(&self, compact: bool, char_stack: &mut String)
  {
    if !self.root
    {
      char_stack.push(self.label);
      if self.terminal {
        if compact { print!("{} ", char_stack); }
        else       { println!("{}", char_stack); }
      }
    }

    for elem in self.next.iter()
    {
      if let Some(sub) = elem
      {
        sub.print_contents_recur(compact, char_stack);
      }
    }

    if !self.root {
      char_stack.pop();
    }
  }
}
