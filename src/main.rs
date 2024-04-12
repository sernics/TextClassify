mod dict;

fn main() {
  let path = std::env::args().nth(1).unwrap();
  let saved_path = std::env::args().nth(2).unwrap();
  let path = std::path::PathBuf::from(path);
  let dict = dict::Dict::new(&path);
  dict.save_file(&std::path::PathBuf::from(saved_path));
}
