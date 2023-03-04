// MIT License

// Copyright (c) 2023 clonne

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
    env, path::Path, fs::File, io::Read,
    collections::HashMap, collections::BTreeSet,
};

use farvm_utils::{diag,pool, Emit};
use farvm_tree;
use farvm_compiler;
use farvm_vm;

const VERSION:&str = "1.0.0-re2021";
const USAGE:&str =
"Usage: farvm <Subcommand> [Options] <Input-Path>...

These are Subcommands(default = run):
    build [Options] <Source-Path>...
        will all source-code file build to an compiled-object file
    run [Options] <Object-Path>
        run an compiled-object file
    emit <Phase> [Options] <Source-Path>
        output an text-format file from phase for build source-code file
    version -V --version
        output current version
    help -H --help [Subcommand]
        output (this usage) or (help for subcommand)

Examples use:
    output syntax tree:
        farvm emit -tree main.fa
    run an file:
        farvm main.fa
    build these sources:
        farvm build -o a.fao a.fa b.fa c.fa
";
const USAGE_BUILD:&str =
"build [Options] <Source-Path>...
";

fn read_to_string(filepath:&str) -> Result<String, String> {
    let target_path = Path::new(filepath);
    if target_path.is_file() {
        match File::open(&target_path) {
            Err(a) => {
                Err(format!("{:?} for open '{}'", a, target_path.display()))
            }
            Ok(mut file) => {
                let mut buf = String::new();
                match file.read_to_string(&mut buf) {
                    Err(a) => {
                        Err(format!("{:?} for read '{}'", a, target_path.display()))
                    }
                    Ok(_) => {
                        Ok(buf)
                    }
                }
            }
        }
    } else {
        Err(format!("it's not a file-path for '{}'", target_path.display()))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Params {
    confs: HashMap<String, String>,
    switchs: BTreeSet<String>,
    input_paths: Vec<String>,
}
impl Params {
    fn new() -> Params {
        Params {
            confs: HashMap::new(),
            switchs: BTreeSet::new(),
            input_paths: Vec::new(),
        }
    }
}

fn subcommand_build(params:Params) {
    println!("build {:?}", params)
}

fn subcommand_run(params:Params) {
    match read_to_string(params.input_paths[0].as_str()) {
        Err(e) => {println!("Error: '{}'", e)}
        Ok(code) => {
            let mut unique_pool = pool::make();
            let mut the_diag = diag::make();
            let the_tree = farvm_tree::build_v1(&code, &mut unique_pool, &mut the_diag);
            if the_diag.has_error() {
                println!("{}", the_diag)
            } else {
                println!("{}", the_tree.emit(&unique_pool));
                let program = farvm_compiler::build(&the_tree);
                let mut cpu = farvm_vm::make();
                farvm_vm::run(&mut cpu, &program);
            }
        }
    }
}

fn subcommand_emit(params:Params) {
    println!("emit {:?}", params)
}

fn make_params(args:&[String]) -> Params {
    let mut params = Params::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a.as_bytes()[0] == ('-' as u8) {
            match a.split_once('=') {
                Some((k,v)) => {
                    if v.len() > 0 {
                        let key = String::from(&k[1..]);
                        params.confs.insert(key, String::from(v));
                    } else {
                        panic!("Error: option '{}' it lask value", a)
                    }
                }
                None => {
                    let switch_name = &a[1..].to_uppercase();
                    if switch_name.len() > 0 {
                        params.switchs.insert(String::from(switch_name));
                    }
                }
            }
        } else {
            params.input_paths.push(a.clone()); break
        }
    }
    while let Some(a) = it.next() {
        params.input_paths.push(a.clone())
    }
    if params.input_paths.len() > 0 {
        params
    } else {
        panic!("Error: lack an input path")
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() > 0 {
        let subcommand = args[0].to_uppercase();
        match subcommand.as_str() {
            "BUILD" => {subcommand_build(make_params(&args[1..]))}
            "RUN" => {subcommand_run(make_params(&args[1..]))}
            "EMIT" => {subcommand_emit(make_params(&args[1..]))}
            "VERSION" | "-V" | "--V" | "--VERSION" => {
                println!("{}", VERSION)
            }
            "HELP" | "-H" | "--H" | "--HELP" => {
                let subcommand_name:String = match args.len() {
                    1 => {String::from("")}
                    _ => {args[1].to_uppercase()}
                };
                let usage = match subcommand_name.as_str() {
                    "BUILD" => {USAGE_BUILD}
                    _ => {USAGE}
                };
                println!("{}", usage)
            }
            _ => {subcommand_run(make_params(args))}
        }
    } else {
        println!("{}", USAGE)
    }
}
