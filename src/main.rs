use std::{env,io::Read};

mod tree;
mod machine;

fn show_version(){
    println!("1.0.0-re2021")
}

fn show_usage() {
    println!(
"Usage: farvm [Option] <Target>

These are valid options:
    .tokens         Output token-text from Target(As:File) only
    .tree           Output syntax-tree from Target(As:File) only
    .ir             Output ir-code from Target(As:File) only
    .run            <Default> Compile files from Target(File|Directory) then Run

Examples use:
    display syntax tree:
        farvm .tree main.fa
    run an file:
        farvm main.fa
    run an directory(start by main.fa):
        farvm ../examples/a-dir
");
}

mod task {
    use std::borrow::BorrowMut;

    use super::*;

    fn for_one(path:&str) -> String {
        match std::fs::File::open(&path) {
            Err(e) => {panic!("Error: {}", e)}
            Ok(f)  => {
                let mut f = &f;
                let mut code = String::new();
                f.read_to_string(&mut code).unwrap();
                code
            }
        }
    }

    pub fn tokens_only(path:&str) {
        let code = for_one(path);
        let mut errors:Vec<tree::Error> = Vec::new();
        let tokens = tree::token::build(&code, &mut errors);
        println!("{}", tree::token::text(&tokens))
    }

    pub fn tree_only(path:&str) {
        let code = for_one(path);
        let (root,_) = tree::build(&code);
        println!("{}", root)
    }

    pub fn ir_only(path:&str) {
        let code = for_one(path);
    }

    pub fn run(path:&str) {
        println!("{}", path)
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let args = &args[1..];
    match args.len() {
        0 => {show_usage()}
        1 => {
            match args[0].as_str() {
                ".tokens" | ".tree" | ".run" => {show_usage()}
                ".v" | ".ver" | ".version" | "-V" | "-v" | "--version" => {show_version()}
                path@_ => {
                    task::run(path)
                }
            }
        }
        2 => {
            match args[0].as_str() {
                ".tokens" => {
                    task::tokens_only(args[1].as_str())
                }
                ".tree" => {
                    task::tree_only(args[1].as_str())
                }
                ".ir" => {
                    task::ir_only(args[1].as_str())
                }
                ".run" => {
                    task::run(args[1].as_str())
                }
                path@_ => {
                    task::run(path)
                }
            }
        }
        _ => {show_usage()}
    }
}
