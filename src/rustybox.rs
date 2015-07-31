use std::env::Args;
use std::collections::HashMap;

pub type Function = Box<Fn(Args) -> i32>;

pub struct RBox {
    func_map: HashMap<String, Function>,
}

impl RBox {
    pub fn new() -> RBox {
        RBox{
            func_map: HashMap::new(),
        }
    }

    pub fn add_func<S>(&mut self, (name, f): (S, Function))
        where S: Into<String> {
            self.func_map.insert(name.into(), f);
        }
    pub fn call<S>(&self, name: S, args: Args) -> i32
        where S: Into<String> {
            let sname: String = name.into();
            if let Some(f) = self.func_map.get(&sname) {
                f(args)
            } else {
                println!("Command not found: {}", sname);
                self.usage()
            }
        }
    
    pub fn usage(&self) -> i32 {
        print!("Usage: rustybox [function] [arguments]...
   or: rustybox --list[-full]
   or: function [arguments]...

  RustyBox is a multi-call binary that combines many common Unix
  utilities into a single executable.  Most people will create a
  link to rustybox for each function they wish to use and RustyBox
  will act like whatever it was invoked as.

Currently defined functions:
");
        for i in self.func_map.keys() {
            println!("	{},", i);
        }

        0
    }
}

static HEADER: &'static str = "RustyBox multi-call binary.";

pub fn usage(custom: &str) -> i32 {
    println!("{}", HEADER);
    println!("{}", custom);
    0
}
