use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

use crate::filter_word::filter_string;

pub struct Dict {
  set: BTreeSet<String>,
}

impl Dict {
  pub fn new(path: &PathBuf) -> Dict {
    println!("Loading dictionary...");
    let contents = read_to_string(path).unwrap();
    Dict {
      set: contents
        .split(['\n', ' ', ',', ';'])
        .map(|word| filter_string(&word))
        .collect(),
    }
  }

  #[allow(dead_code)]
  pub fn contains(&self, word: &str) -> bool {
    self.set.contains(word)
  }

  #[allow(dead_code)]
  pub fn save_file(&self, path: &PathBuf) {
    println!("Saving dictionary...");
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)
      .unwrap();
    file.write_all(format!("{}", self).as_bytes()).unwrap();
  }

  pub fn get_dict(&self) -> &BTreeSet<String> {
    &self.set
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
