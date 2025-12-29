use std::env;
use std::path::Path;
use serde_json::to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: adapter_fleetyards_cli <path-to-export.csv>");
        std::process::exit(1);
    }

    let p = Path::new(&args[1]);
    match adapter_fleetyards::parse_file(p) {
        Ok(records) => {
            for r in records {
                match to_string(&r) {
                    Ok(j) => println!("{}", j),
                    Err(e) => eprintln!("serialization error: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("error reading file: {}", e);
            std::process::exit(2);
        }
    }
}
