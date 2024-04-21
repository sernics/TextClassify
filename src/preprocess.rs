use rust_stemmers::Stemmer;

pub fn preprocess_word(stemmer: &Stemmer, word: &str) -> String {
  stemmer.stem(word.to_lowercase().as_str()).to_string()
}