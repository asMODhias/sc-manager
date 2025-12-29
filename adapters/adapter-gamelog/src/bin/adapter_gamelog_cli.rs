use std::env;
use std::path::Path;
use serde_json::to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: adapter_gamelog_cli <path-to-game.log>");
        std::process::exit(1);
    }

    let p = Path::new(&args[1]);
    match adapter_gamelog::parser::parse_file(p) {
        Ok(suggestions) => {
            for s in suggestions {
                // print one JSON object per line
                match to_string(&s) {
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
