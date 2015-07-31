use std::env::Args;
use std::fs::File;
use std::io::copy;
use std::io::empty;
use std::io::Read;
use std::io::Error;
use std::io::stdout;

pub fn cat(args: Args) -> i32 {
    let mut rd: Box<Read> = {
        let mut outRd: Box<Read> = Box::new(empty());
        for fname in args {
            let fRes = File::open(fname.clone());
            match fRes {
                Ok(file) => outRd = Box::new(outRd.chain(file)),
                Err(error) => {
                    println!("cat: {}: {}", fname, error);
                    return -1;
                },
            }
        }
        outRd
    };
    copy(&mut rd, &mut stdout());
    0
}
