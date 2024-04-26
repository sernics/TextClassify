use std::collections::{BTreeSet, HashMap};
use std::io::Write;

use crate::dict::Dict;

pub struct Notice {
  id: u32,
  body: String
}

pub struct Corpus {
  notices: u32,
  words: u32,
  notices_list: BTreeSet<Notice>,
  words_count: HashMap<String, u32>
}

impl Notice {
  pub fn new(id: u32, body: String) -> Notice {
    Notice {
      id: id,
      body: body
    }
  }
  #[allow(dead_code)]
  pub fn get_id(&self) -> u32 {
    self.id
  }
  #[allow(dead_code)]
  pub fn get_body(&self) -> &str {
    &self.body
  }
}

impl Eq for Notice {}

impl PartialEq for Notice {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl PartialOrd for Notice {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Notice {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.id.cmp(&other.id)
  }
}

impl Corpus {
  pub fn new() -> Corpus {
    Corpus {
      notices: 0,
      words: 0,
      notices_list: BTreeSet::new(),
      words_count: HashMap::new()
    }
  }
  pub fn add_notice(&mut self, notice: Notice) {
    self.notices += 1;
    self.words += notice.body.split_whitespace().count() as u32;
    for word in notice.body.split_whitespace() {
      let count = self.words_count.entry(word.to_string()).or_insert(0);
      *count += 1;
    }
    self.notices_list.insert(notice);
  }
  #[allow(dead_code)]
  pub fn get_notices(&self) -> &BTreeSet<Notice> {
    &self.notices_list
  }
  #[allow(dead_code)]
  pub fn get_notice(&self, id: u32) -> &Notice {
    self.notices_list.iter().find(|x| x.id == id).unwrap()
  }
  #[allow(dead_code)]
  pub fn laplazian_smoothing(&mut self, dict: &Dict) {
    for word in dict.get_dict() {
      let count = self.words_count.entry(word.to_string()).or_insert(0);
      *count += 1;
      self.words += 1;
    }
  }
  pub fn save_file(&self, path: &std::path::PathBuf) {
    println!("Saving corpus {}", path.display());
    if let Ok(mut file) = std::fs::File::create(path) {
      if let Err(err) = write!(file, "Número de documentos del corpus: {}\nNúmero de palabras del corpus: {}\n", self.notices, self.words) {
        eprintln!("Error writing to file: {}", err);
        return;
      }
      for (key, value) in &self.words_count {
        let log_prob = ((*value as f32) / (self.words as f32)).log2();
        if let Err(err) = write!(file, "Palabra: {}: Frecuencia: {} LogProb:{}\n", key, value, log_prob) {
          eprintln!("Error writing to file: {}", err);
          return;
        }
      }
      println!("File saved successfully.");
    } else {
      eprintln!("Error creating file.");
    }
  }
}

impl std::fmt::Display for Corpus {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "Número de documentos del corups: {}\nNúmero de palabras del corpus: {}", 
               self.notices, self.words)?;
    for (key, value) in &self.words_count {
      let log_prob = ((*value as f32) / (self.words as f32)).log2();
      write!(f, "Palabra: {}: Frecuencia: {} LogProb:{}\n", key, value, log_prob)?;
    }
    Ok(())
  }
}