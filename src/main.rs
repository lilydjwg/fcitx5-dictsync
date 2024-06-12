use std::collections::HashSet;
use std::io::{Result, stdout, Write};

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
}

fn main() -> Result<()> {
  let args = Args::parse();
  let a = Dict::load_dict(&args.a_main, &args.a_user)?;
  let b = Dict::load_dict(&args.b_main, &args.b_user)?;
  let r = compare_dict(&a, &b);
  let mut stdout = stdout().lock();

  writeln!(stdout, "[a_add]")?;
  for w in r.a_add {
    writeln!(stdout, "{}", w)?;
  }
  writeln!(stdout, "[a_delete]")?;
  for w in r.a_delete {
    writeln!(stdout, "{}", w)?;
  }
  writeln!(stdout, "[b_add]")?;
  for w in r.b_add {
    writeln!(stdout, "{}", w)?;
  }
  writeln!(stdout, "[b_delete]")?;
  for w in r.b_delete {
    writeln!(stdout, "{}", w)?;
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
