use std::path::PathBuf;
use anyhow::{Result, Context};
use std::io::{self, Write};
use std::process::{Child, Command};

mod builtins;
use builtins::Builtins;

#[allow(non_camel_case_types)]
enum Signals {
    EXIT_SIGNAL
}

#[derive(Default)]
pub struct Rlsh {
    terminate: bool,
    old_pwd: Option<PathBuf>
}

impl Rlsh {
    pub fn new() -> Self { Default::default() }
    pub fn run() -> Result<()> { Self::new().rlsh_loop() }
}

impl Rlsh {
    pub fn rlsh_loop(&mut self) -> Result<()> {
        print!("$ ");
        loop {
            io::stdout().flush()?;

            let line = Self::rlsh_read_line()?;
            let args = Self::rlsh_split_line(line);

            match self.rlsh_execute(args) {
                Ok(None) => {
                    if self.terminate { return Ok(()); }
                    print!("$ ")
                },

                Ok(Some(mut c)) => {
                    c.wait()?;
                    print!("$ ");
                },

                Err(e) => {
                    println!("{e:#}");
                    print!("$ ");
                }
            }
        }
    }

    fn rlsh_read_line() -> Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        Ok(input)
    }

    fn rlsh_split_line(line: String) -> Vec<String> {
        line.split_whitespace().map(String::from).collect()
    }

    fn rlsh_execute(&mut self, args: Vec<String>) -> Result<Option<Child>>{
        if args.is_empty() { return Ok(None); }

        if let Some(f) = Builtins::from_str(&args[0]) {
            if let Some(Signals::EXIT_SIGNAL) = f.func()(self, args).with_context(|| "rlsh")? { self.terminate = true; }
            return Ok(None);
        }

        Ok(Some(Self::rlsh_launch(args)?))
    }

    fn rlsh_launch(args: Vec<String>) -> Result<Child> {
        Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .with_context(|| format!("{}: command not found", args[0]))
    }
}
