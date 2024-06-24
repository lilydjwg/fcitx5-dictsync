use std::collections::HashSet;
use std::io::{Result, Write, BufWriter};
use std::fs::File;

use clap::Parser;

mod loaddata;

use loaddata::{Dict, Word};

#[derive(Parser)]
struct Args {
  #[arg()]
  base_main: String,
  #[arg()]
  a_main: String,
  #[arg()]
  b_main: String,
  #[arg()]
  a_script: String,
  #[arg()]
  b_script: String,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let base = Dict::load_dict(&args.base_main)?;
  let a = Dict::load_dict(&args.a_main)?;
  let b = Dict::load_dict(&args.b_main)?;
  let base_all = base.all_words();
  let a_all = a.all_words();
  let b_all = b.all_words();
  let mut a_change = compare_dict(&base_all, &a_all);
  let mut b_change = compare_dict(&base_all, &b_all);

  a_change.added.retain(|w| !b_all.contains(w));
  a_change.deleted.retain(|w| b_all.contains(w));

  b_change.added.retain(|w| !a_all.contains(w));
  b_change.deleted.retain(|w| a_all.contains(w));

  write_result(&args.a_script, &b_change)?;
  write_result(&args.b_script, &a_change)?;

  Ok(())
}

fn write_result(
  path: &str,
  change: &Changes,
) -> Result<()> {
  let f = File::create(path)?;
  let mut f = BufWriter::new(f);
  for w in &change.added {
    writeln!(f, "insert {} {}", w.code, w.word)?;
  }
  for w in &change.deleted {
    writeln!(f, "delete {} {}", w.code, w.word)?;
  }
  if !change.added.is_empty() || !change.deleted.is_empty() {
    writeln!(f, "save")?;
  }
  Ok(())
}

struct Changes {
  added: HashSet<Word>,
  deleted: HashSet<Word>,
}

fn compare_dict(old: &HashSet<Word>, new: &HashSet<Word>) -> Changes {
  Changes { 
    added: new.difference(old).cloned().collect(),
    deleted: old.difference(new).cloned().collect(),
  }
}
