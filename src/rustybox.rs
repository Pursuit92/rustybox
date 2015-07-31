use std::env::Args;
use std::process::exit;
use std::collections::HashMap;

type Function = Box<Fn(Args) -> i32>;

pub struct RBox {
    funcMap: HashMap<String, Function>,
}

impl RBox {
    pub fn new() -> RBox {
        RBox{
            funcMap: HashMap::new(),
        }
    }

    pub fn add_func<S>(&mut self, name: S, f: Function)
        where S: Into<String> {
            self.funcMap.insert(name.into(), f);
            
        }
    pub fn call<S>(&self, name: S, args: Args) -> i32
        where S: Into<String> {
            let sname: String = name.into();
            if let Some(f) = self.funcMap.get(&sname) {
                f(args)
            } else {
                println!("Command not found: {}", sname);
                self.usage()
            }
        }
    
    pub fn usage(&self) -> i32 {
        print!("
Usage: rustybox [function] [arguments]...
   or: rustybox --list[-full]
   or: function [arguments]...

  RustyBox is a multi-call binary that combines many common Unix
  utilities into a single executable.  Most people will create a
  link to rustybox for each function they wish to use and RustyBox
  will act like whatever it was invoked as.

Currently defined functions:
");
        for i in self.funcMap.keys() {
            println!("	{},", i);
        }

        0
    }
}
