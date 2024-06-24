use std::collections::HashSet;
use std::io::{Result, BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Word {
  pub code: String,
  pub word: String,
}

pub struct Dict {
  pub main: HashSet<Word>,
  pub user_new: HashSet<Word>,
  pub user_deleted: HashSet<Word>,
}

impl Dict {
  pub fn load_dict(main_path: &str) -> Result<Dict> {
    let main = load_main(main_path)?;
    let user_path = main_path.replace(".main.", ".user.");
    let (user_new, user_deleted) = load_user(&user_path)?;
    Ok(Dict {
      main, user_new, user_deleted,
    })
  }

  pub fn all_words(&self) -> HashSet<Word> {
    let mut all: HashSet<_> = self.user_new.union(&self.main).cloned().collect();
    all.retain(|w| !self.user_deleted.contains(w));
    all
  }
}

impl std::fmt::Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
    -> std::result::Result<(), std::fmt::Error>
  {
    write!(f, "{} {}", self.code, self.word)
  }
}

fn load_main(p: &str) -> Result<HashSet<Word>> {
  let mut r = HashSet::new();
  let mut is_data = false;

  let child = Command::new("libime_tabledict")
    .arg("-d")
    .arg(p)
    .arg("-")
    .stdout(Stdio::piped())
    .spawn()?;
  let reader = BufReader::new(child.stdout.unwrap());

  for_each_line(reader, |mut line| {
    if line.ends_with('\n') {
      line = &line[..line.len()-1];
    }
    if is_data {
      let mut it = line.splitn(2, ' ');
      let code = String::from(it.next().unwrap());
      let word = String::from(it.next().unwrap());
      r.insert(Word { code, word });
    } else if line == "[Data]" {
      is_data = true;
    }
  })?;

  Ok(r)
}

enum UserdictSection {
  New,
  Auto,
  Delete,
}

fn load_user(p: &str) -> Result<(HashSet<Word>, HashSet<Word>)> {
  let mut user_new = HashSet::new();
  let mut user_deleted = HashSet::new();
  let mut section = UserdictSection::New;

  let child = Command::new("libime_tabledict")
    .arg("-du")
    .arg(p)
    .arg("-")
    .stdout(Stdio::piped())
    .spawn()?;
  let reader = BufReader::new(child.stdout.unwrap());

  for_each_line(reader, |mut line| {
    if line.ends_with('\n') {
      line = &line[..line.len()-1];
    }
    if line == "[Auto]" {
      section = UserdictSection::Auto;
    } else if line == "[Delete]" {
      section = UserdictSection::Delete;
    } else {
      let mut it = line.splitn(2, ' ');
      let code = String::from(it.next().unwrap());
      let word = String::from(it.next().unwrap());
      let r = match section {
        UserdictSection::New => &mut user_new,
        UserdictSection::Delete => &mut user_deleted,
        UserdictSection::Auto => return,
      };
      r.insert(Word { code, word });
    }
  })?;

  Ok((user_new, user_deleted))
}

fn for_each_line<R: BufRead>(mut r: R, mut func: impl FnMut(&str)) -> Result<()> {
  let mut buffer = String::new();

  loop {
    buffer.clear();
    let n = r.read_line(&mut buffer)?;
    if n == 0 { break; }
    func(&buffer[..n]);
  }

  Ok(())
}
