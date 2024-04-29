use crate::preprocess::preprocess_word;

use rust_stemmers::Stemmer;

fn filter_word(word: &String) -> bool {
  !word.is_empty()
}

pub fn filter_string(stemmer: &Stemmer,word: &str) -> String {
  let result = word.split_whitespace().map(|x| x.to_string()).filter(filter_word).collect::<Vec<String>>().join(" ");
  preprocess_word(&stemmer, &result)
}