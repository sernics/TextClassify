pub fn preprocess_word(word: &str) -> String {
  if word.chars().all(|c| c.is_ascii_punctuation()) {
    return "<PUNCT>".to_string();
  }

  if word.chars().all(|c| c.is_numeric()) {
    return "<NUM>".to_string();
  }

  if !word.chars().any(|c| c.is_alphabetic()) {
    return "<PUNCT>".to_string();
  }

  let mut letter = ' ';
  let mut letter_count = 0;
  let mut new_word = String::new();
  for c in word.chars() {
    if c == letter {
      letter_count += 1;
    } else {
      letter = c;
      letter_count = 1;
    }
    if letter_count < 4 {
      new_word.push(c);
    }
  }
  let word = new_word;

  word.to_lowercase()
}