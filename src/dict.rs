use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

fn filter_word(word: &String) -> bool {
  let mut letter = ' ';
  let mut letter_count = 0;
  for c in word.chars() {
    if !c.is_alphabetic() {
      return false;
    }
    if c == letter {
      letter_count += 1;
    } else {
      letter = c;
      letter_count = 1; 
    }
    if letter_count >= 3 {
      return false;
    }
  } 
  !word.is_empty()
}

pub struct Dict {
  set: BTreeSet<String>,
}

impl Dict {
  pub fn new(path: &PathBuf) -> Dict {
    let contents = read_to_string(path).unwrap();
    Dict {
      set: contents
        .split(['\n', ' ', ',', ';'])
        .map(|raw| raw.to_lowercase())
        .filter(filter_word)
        .collect(),
    }
  }
  pub fn contains(&self, word: &str) -> bool {
    self.set.contains(word)
  }
  pub fn save_file(&self, path: &PathBuf) {
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)
      .unwrap();
    let header = format!("Número de palabras: {}\n", self.set.len());
    let contents = self.set.iter().fold(String::new(), |acc, word| {
      acc + word + "\n"
    });
    file.write_all((header + contents.as_str()).as_bytes()).unwrap();
  }
}

impl std::fmt::Display for Dict {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Número de palabras: {}\n", self.set.len())?;
    for word in self.set.iter() {
      write!(f, "{}\n",word)?;
    }

    Ok(())
  }
}
