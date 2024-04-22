use crate::preprocess::preprocess_word;

use rust_stemmers::Stemmer;

pub fn filter_word(word: &String) -> bool {
  let mut letter = ' ';
  let mut letter_count = 0;
  for c in word.chars() {
    if !c.is_alphabetic() {
      return false;
    }
    if c == letter {
      letter_count += 1;
    } else {
      letter = c;
      letter_count = 1;
    }
    if letter_count >= 3 {
      return false;
    }
  }
  !word.is_empty()
}

pub fn filter_string(stemmer: &Stemmer,word: &str) -> String {
  let result = word.split_whitespace().map(|x| x.to_string()).filter(filter_word).collect::<Vec<String>>().join(" ");
  preprocess_word(&stemmer, &result)
}