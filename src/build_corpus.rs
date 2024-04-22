use std::{fs::read_to_string, path::PathBuf};

use crate::corpus::{Corpus, Notice};
use crate::filter_word::filter_string;

use rust_stemmers::{Algorithm, Stemmer};

pub fn build_corpus(path: &PathBuf) -> (Corpus, Corpus) {
  println!("Building corpus...");
  let mut corpus_p = Corpus::new(); // Phising corpus
  let mut corpus_s = Corpus::new(); // Safe corpus

  let contents = read_to_string(path).unwrap();

  let contents = contents.replace("\n", " ");
  let contents = contents.split("\r").collect::<Vec<&str>>();
  let mut contents = contents.iter().map(|x| x.split(";").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

  contents.remove(0);

  let stemmer = Stemmer::create(Algorithm::English);

  println!("Processing notices...");
  for i in 0..contents.len() - 1 {
    let data = filter_string(&stemmer, contents[i][1]); // Aqui ya se hace el preprocesado
    let notice = Notice::new((i + 1) as u32, data);
    match contents[i][2] {
      "Phishing Email" => corpus_p.add_notice(notice),
      "Safe Email" => corpus_s.add_notice(notice),
      _ => panic!("Invalid notice type")
    }
  }
  (corpus_p, corpus_s)
}
