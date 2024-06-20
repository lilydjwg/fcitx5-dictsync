use std::collections::HashSet;
use std::io::{Result, Write, BufWriter};
use std::fs::File;

use clap::Parser;

mod loaddata;

use loaddata::{Dict, Word};

#[derive(Parser)]
struct Args {
  #[arg()]
  a_main: String,
  #[arg()]
  a_user: String,
  #[arg()]
  b_main: String,
  #[arg()]
  b_user: String,
  #[arg()]
  a_script: String,
  #[arg()]
  b_script: String,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let a = Dict::load_dict(&args.a_main, &args.a_user)?;
  let b = Dict::load_dict(&args.b_main, &args.b_user)?;
  let r = compare_dict(&a, &b);

  write_result(&args.a_script, &r.a_add, &r.a_delete)?;
  write_result(&args.b_script, &r.b_add, &r.b_delete)?;
  Ok(())
}

fn write_result(
  path: &str,
  to_add: &HashSet<Word>,
  to_del: &HashSet<Word>,
) -> Result<()> {
  let f = File::create(path)?;
  let mut f = BufWriter::new(f);
  for w in to_add {
    writeln!(f, "insert {} {}", w.code, w.word)?;
  }
  for w in to_del {
    writeln!(f, "delete {} {}", w.code, w.word)?;
  }
  if !to_add.is_empty() || !to_del.is_empty() {
    writeln!(f, "save")?;
  }
  Ok(())
}

struct CompareResult {
  a_add: HashSet<Word>,
  a_delete: HashSet<Word>,
  b_add: HashSet<Word>,
  b_delete: HashSet<Word>,
}

fn compare_dict(a: &Dict, b: &Dict) -> CompareResult {
  let mut b_add: HashSet<_> = a.user_new.difference(&a.main).cloned().collect();
  let mut a_add: HashSet<_> = b.user_new.difference(&b.main).cloned().collect();
  a_add.retain(|w|
    !a.main.contains(w) && !a.user_new.contains(w) && !a.user_deleted.contains(w)
  );
  b_add.retain(|w|
    !b.main.contains(w) && !b.user_new.contains(w) && !b.user_deleted.contains(w)
  );

  let mut a_delete = b.user_deleted.clone();
  a_delete.retain(|w|
    !a.user_deleted.contains(w) && (a.main.contains(w) || a.user_new.contains(w))
  );
  let mut b_delete = a.user_deleted.clone();
  b_delete.retain(|w|
    !b.user_deleted.contains(w) && (b.main.contains(w) || b.user_new.contains(w))
  );

  CompareResult { a_add, a_delete, b_add, b_delete }
}
