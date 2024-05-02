mod dict;
mod preprocess;
mod corpus;
mod build_corpus;
mod filter_word;
mod notice;
mod test_corpus;
use std::time::Instant;

fn main() {
  let start_time = Instant::now();

  let path = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
  let saved_vocabulary = std::env::args().nth(2).unwrap();
  let train_path = std::path::PathBuf::from(std::env::args().nth(3).unwrap());
  let test_path = std::path::PathBuf::from(std::env::args().nth(4).unwrap());
  let saved_corpus_p = std::env::args().nth(5).unwrap();
  let saved_corpus_s = std::env::args().nth(6).unwrap();
  let clasification_path = std::path::PathBuf::from(std::env::args().nth(7).unwrap());
  let resumen_path = std::path::PathBuf::from(std::env::args().nth(8).unwrap());
  let check : bool = std::env::args().nth(9).unwrap().parse().unwrap();
  let dict = dict::Dict::new(&path);
  dict.save_file(&std::path::PathBuf::from(saved_vocabulary));
  let (corpus_p, corpus_s) = build_corpus::build_corpus(&train_path, &dict);
  corpus_p.save_file(&std::path::PathBuf::from(saved_corpus_p));
  corpus_s.save_file(&std::path::PathBuf::from(saved_corpus_s));
  test_corpus::test_corpus(&test_path, corpus_p, corpus_s, clasification_path, resumen_path, check);
  let elapsed_time = start_time.elapsed();

  println!("Execution time: {:?}", elapsed_time);
}
