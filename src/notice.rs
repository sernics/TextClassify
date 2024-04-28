pub struct Notice {
  id: u32,
  body: String
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