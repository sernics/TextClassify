use std::fs::read_to_string;
use std::path::PathBuf;

use crate::corpus::Corpus;
use crate::notice::Notice;
use crate::filter_word::filter_string;

pub fn test_corpus(path: &PathBuf, corpus_p: Corpus, corpus_s: Corpus) {
  println!("Testing corpus...");
  let contents = read_to_string(path).unwrap();
  let mut contents = contents.split("\r").collect::<Vec<&str>>();
  contents.remove(0);

  println!("Processing test notices...");
  let mut count = 0;
  for i in 0..contents.len() - 1 {
    let data = filter_string(contents[i].split(";").collect::<Vec<&str>>()[1]);
    let notice = Notice::new((i + 1) as u32, data);
    let data = contents[i].split(";").collect::<Vec<&str>>();
    let probs = test_notice(&notice, &corpus_p, &corpus_s);
    let max_result = if probs.0 > probs.1 { "Phishing Email" } else { "Safe Email" };
    if max_result == data[2] {
      count += 1;
    }
  }
  // Calcular el porcentaje de aciertos
  println!("Porcentaje de aciertos: {}", (count as f32 / contents.len() as f32));
}

#[allow(dead_code)]
pub fn test_notice(notice: &Notice, corpus_p: &Corpus, corpus_s: &Corpus) -> (f32, f32) {
  let prob_p = corpus_p.calculate_probability(notice);
  let prob_s = corpus_s.calculate_probability(notice);
  (prob_p, prob_s)
}