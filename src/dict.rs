use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use rust_stemmers::{Algorithm, Stemmer};

use crate::preprocess::preprocess_word;

extern crate rust_stemmers;

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
    // Create a stemmer for the English language
    let stemmer = Stemmer::create(Algorithm::English); 
    Dict {
      set: contents
        .split(['\n', ' ', ',', ';'])
        .map(|word| preprocess_word(&stemmer, &word))
        .filter(filter_word)
        .collect(),
    }
  }

  #[allow(dead_code)]
  pub fn contains(&self, word: &str) -> bool {
    self.set.contains(word)
  }

  #[allow(dead_code)]
  pub fn save_file(&self, path: &PathBuf) {
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)
      .unwrap();
    file.write_all(format!("{}", self).as_bytes()).unwrap();
  }
}

impl std::fmt::Display for Dict {
  fn fmt(&self, output: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(output, "Número de palabras: {}\n", self.set.len())?;
    for word in self.set.iter() {
      write!(output, "{}\n", word)?;
    }

    Ok(())
  }
}
