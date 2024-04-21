use std::collections::BTreeSet;

pub struct Notice {
  id: u32,
  body: String
}

pub struct Corpus {
  notices: u32,
  words: u32,
  notices_list: BTreeSet<Notice>,
}

impl Notice {
  pub fn new(id: u32, body: String) -> Notice {
    Notice {
      id: id,
      body: body
    }
  }
  pub fn get_id(&self) -> u32 {
    self.id
  }
  pub fn get_body(&self) -> &str {
    &self.body
  }
}

impl Eq for Notice {}

impl PartialEq for Notice {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl PartialOrd for Notice {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Notice {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.id.cmp(&other.id)
  }
}

impl Corpus {
  pub fn new() -> Corpus {
    Corpus {
      notices: 0,
      words: 0,
      notices_list: BTreeSet::new(),
    }
  }
  pub fn add_notice(&mut self, notice: Notice) {
    self.notices += 1;
    self.words += notice.body.split_whitespace().count() as u32;
    self.notices_list.insert(notice);
  }
  pub fn get_notices(&self) -> &BTreeSet<Notice> {
    &self.notices_list
  }
  pub fn get_notice(&self, id: u32) -> &Notice {
    self.notices_list.iter().find(|x| x.id == id).unwrap()
  }
}