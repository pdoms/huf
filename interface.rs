use std::io;
use crate::error::{Result as R, Error};


pub fn usage(prog: &str) {
    println!("Usage: {prog} [Commands] [flags]");
    println!("Commands");
    println!("    in_file.............the source file. If a file with the extension '.huf' is encountered it is a 'decompression' operation. If this command is skipped, it is assumed that the source is inserted via stdout or a pipe.");
    println!("    print_out...........writes the output to stdout");
    println!("    -h/--help...........print this usage");
}

//NOTE if in is stdin then always to stdout - or implement a guard and arg 
//that requires out_file to be set
#[derive(Debug, Default)]
pub struct Args {
    pub program: String,
    pub in_file: Option<String>,
    pub decompress: bool,
    pub data: Option<String>,
    pub print_out: bool,
}



impl Args {
    pub fn from_env_args() -> R<Self> {
        let mut args = Args::default();
        let mut args_in = std::env::args();
        args.program = args_in.next().unwrap();
        let args_len = args_in.len();
        
        if args_len == 0 {
            //try stdin
            args.data = Some(try_read_stdin().map_err(|err| {
                usage(args.program.as_str());
                return Error::Args("".to_string(), err.to_string());
            })?);
        }

        if let Some(next) = args_in.next() {
            if next == "-h" || next == "--help" {
                usage(args.program.as_str());
                std::process::exit(1);
            }
             
            if args_len == 1 {
                if arg_is_print(next.as_str()) {
                    //then check stdin
                args.data = Some(try_read_stdin().map_err(|err| {
                    usage(args.program.as_str());
                    return Error::Args("".to_string(), err.to_string());
                    })?);
                } else {
                        //must be a file
                    args.decompress = next.ends_with(".huf");
                    args.in_file = Some(next);
                }
                return Ok(args)
            };
            if args_len == 2 {
                //expects file first
                args.in_file = Some(next);
                //next argument expected to be stdout
                if let Some(last) = args_in.next() {
                    args.print_out = arg_is_print(last.as_str());
                }
            }
        } else {
            //error - usage
        }
        println!("{:?}", args_in);
        println!("{:?}", args);
        Ok(args)
    }
}

fn arg_is_print(arg: &str) -> bool {
    arg == "1" || arg == "-" || arg == "stdout" || arg == "-p" || arg == "--print" 
}

fn try_read_stdin() -> Result<String, std::io::Error> {
    let stdin = io::stdin();
    let mut data = String::new();
    for line in stdin.lines() {
        data.push_str(line?.as_str());
    }
    Ok(data)
}

