use std::{fs::read_to_string, path::PathBuf};

use crate::corpus::Corpus;
use crate::notice::Notice;
use crate::filter_word::filter_string;
use crate::dict::Dict;

use rust_stemmers::{Algorithm, Stemmer};

pub fn build_corpus(path: &PathBuf, dict: &Dict) -> (Corpus, Corpus) {
  println!("Building corpus...");
  let mut corpus_p = Corpus::new(); // Phising corpus
  let mut corpus_s = Corpus::new(); // Safe corpus

  let contents = read_to_string(path).unwrap();

  let mut contents = contents.split("\r").collect::<Vec<&str>>();

  contents.remove(0);

  // let (contents, test_contents) = contents.split_at(10000);

  let contents = contents.iter().map(|x| x.split(";").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

  let stemmer = Stemmer::create(Algorithm::English);

  println!("Processing notices...");
  for i in 0..contents.len() {
    let data = filter_string(&stemmer, contents[i][1]);
    let notice = Notice::new((i + 1) as u32, data);
    match contents[i][2] {
      "Phishing Email" => corpus_p.add_notice(notice),
      "Safe Email" => corpus_s.add_notice(notice),
      _ => panic!("Invalid notice type")
    }
  }

  println!("Laplazian smoothing...");
  corpus_p.laplazian_smoothing(dict);
  corpus_s.laplazian_smoothing(dict);

  (corpus_p, corpus_s)
}