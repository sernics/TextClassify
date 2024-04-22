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