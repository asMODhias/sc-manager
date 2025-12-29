use adapter_uex::{parse_file, UexRecord};
use serde_json::to_string;
use std::env;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: adapter_uex_cli <file>");
        std::process::exit(2);
    }

    let path = Path::new(&args[1]);
    let recs = parse_file(path)?;

    for r in recs {
        println!("{}", to_string(&r)?);
    }

    Ok(())
}
