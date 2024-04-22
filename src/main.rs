use std::time::Instant;

mod dict;
mod preprocess;
mod corpus;
mod build_corpus;

fn main() {
  let path = std::env::args().nth(1).unwrap();
  //let saved_path = std::env::args().nth(2).unwrap();
  let path = std::path::PathBuf::from(path);
  // let dict = dict::Dict::new(&path);
  // dict.save_file(&std::path::PathBuf::from(saved_path));
  let start_time = Instant::now();
  let (corpus_p, corpus_s) = build_corpus::build_corpus(&path);
  let end_time = Instant::now();
  let execution_time = end_time - start_time;
  println!("Execution time: {:?}", execution_time);
  println!("Corpus p:");
  println!("{}", corpus_p);
  println!("Corpus s:");
  println!("{}", corpus_s);
}
