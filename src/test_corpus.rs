use std::fs::read_to_string;
use std::path::PathBuf;

use crate::corpus::Corpus;
use crate::notice::Notice;
use crate::filter_word::filter_string;

pub fn test_corpus(path: &PathBuf, corpus_p: Corpus, corpus_s: Corpus, clasification_path: PathBuf, resumen_path: PathBuf) {
  println!("Testing corpus...");
  let contents = read_to_string(path).unwrap();
  let mut contents = contents.split("\r").collect::<Vec<&str>>();
  contents.remove(0);

  println!("Processing test notices...");
  let mut count = 0;
  let mut classification = String::new();
  let mut resumen = String::new();
  for i in 0..contents.len() - 1 {
    let content = contents[i].split(";").collect::<Vec<&str>>()[1];
    classification.push_str(&content.chars().take(10).collect::<String>());
    classification.push_str(",");
    let data = filter_string(content);
    let notice = Notice::new((i + 1) as u32, data);
    let data = contents[i].split(";").collect::<Vec<&str>>();
    let probs = test_notice(&notice, &corpus_p, &corpus_s);
    classification.push_str(&format!("{:.2}", probs.0));
    classification.push_str(",");
    classification.push_str(&format!("{:.2}", probs.1));
    let max_result : &str;
    if probs.0 > probs.1 {
      classification.push_str(",Phishing Email\n");
      resumen.push_str("P\n");
      max_result = "Phishing Email";
    } else {
      classification.push_str(",Safe Email\n");
      resumen.push_str("S\n");
      max_result = "Safe Email";
    }
    if max_result == data[2] {
      count += 1;
    }
  }
  println!("Saving classifications results...");
  std::fs::write(clasification_path, classification).unwrap();
  println!("Saving resumen results...");
  std::fs::write(resumen_path, resumen).unwrap();
  // Calcular el porcentaje de aciertos
  println!("Porcentaje de aciertos: {:.2}", (count as f32 / contents.len() as f32) * 100.0);
}

#[allow(dead_code)]
pub fn test_notice(notice: &Notice, corpus_p: &Corpus, corpus_s: &Corpus) -> (f32, f32) {
  let prob_p = corpus_p.calculate_probability(notice);
  let prob_s = corpus_s.calculate_probability(notice);
  (prob_p, prob_s)
}