use std::{env,io::Read};

mod tree;
mod machine;

mod task {
    use super::*;

    pub fn run_file(path:&String) {
        let mut f = match std::fs::File::open(&path) {
            Err(e) => {return println!("Error: {}", e)}
            Ok(f) => {f}
        };
        let mut code = String::new();
        f.read_to_string(&mut code).unwrap();
        let tokens = tree::token::build(&code);
        println!("{}", tree::token::text(&tokens))
    }
}

mod doargs {
    pub fn usage() {
        println!(
"Usage: farvm <path for .fa>"
        );
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let args = &args[1..];
    match args.len() {
        0 => {doargs::usage()}
        _ => {
            task::run_file(&args[0]);
        }
    }
}
