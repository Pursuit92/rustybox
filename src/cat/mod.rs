use std::env::Args;
use std::fs::File;
use std::io::copy;
use std::io::empty;
use std::io::Read;
use std::io::stdout;
use std::io::stdin;
use rustybox::Function;

pub fn binding() -> (&'static str, Function) {
    ("cat", Box::new(cat))
}

fn cat(args: Args) -> i32 {
    let mut rd: Box<Read> = {
        let mut files = false;
        let mut out_rd: Box<Read> = Box::new(empty());
        for fname in args {
            if fname == "-" {
                out_rd = Box::new(out_rd.chain(stdin()));
            } else {
                let f_res = File::open(fname.clone());
                match f_res {
                    Ok(file) => out_rd = Box::new(out_rd.chain(file)),
                    Err(error) => {
                        println!("cat: {}: {}", fname, error);
                        return -1;
                    },
                }
            }
        }
        if files {
            out_rd
        } else {
            Box::new(stdin())
        }
    };
    match copy(&mut rd, &mut stdout()) {
        Ok(_) => 0,
        Err(error) => {
            println!("cat: {}", error);
            -1
        }
    }
}
