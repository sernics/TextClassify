mod dict;
mod preprocess;
mod corpus;
mod build_corpus;
mod filter_word;

fn main() {
  let path = std::env::args().nth(1).unwrap();
  let saved_path = std::env::args().nth(2).unwrap();
  let saved_corpus_p = std::env::args().nth(3).unwrap();
  let saved_corpus_s = std::env::args().nth(4).unwrap();
  let path = std::path::PathBuf::from(path);
  let dict = dict::Dict::new(&path);
  dict.save_file(&std::path::PathBuf::from(saved_path));
  let (corpus_p, corpus_s) = build_corpus::build_corpus(&path);
  println!("Corpus p:");
  corpus_p.save_file(&std::path::PathBuf::from(saved_corpus_p));
  println!("Corpus s:");
  corpus_s.save_file(&std::path::PathBuf::from(saved_corpus_s));
}
