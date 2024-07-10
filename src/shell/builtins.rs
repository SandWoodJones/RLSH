use std::env;
use anyhow::{Result, anyhow};
use strum::{IntoEnumIterator, EnumIter, EnumMessage};

use super::{Rlsh, Signals};

#[derive(strum::Display, EnumIter, EnumMessage)]
#[strum(serialize_all = "lowercase")]
pub enum Builtins {
    #[strum(message="change the shell working directory")]
    Cd,
    #[strum(message="display information about builtin commands")]
    Help,
    #[strum(message="exit the shell")]
    Exit
}

impl Builtins {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cd" => Some(Self::Cd),
            "help" => Some(Self::Help),
            "exit" => Some(Self::Exit),
            _ => None
        }
    }
    
    pub fn func(&self) -> fn(&mut Rlsh, Vec<String>) -> Result<Option<Signals>> {
        match self {
            Self::Cd => Self::rlsh_cd,
            Self::Help => Self::rlsh_help,
            Self::Exit => Self::rlsh_exit
        }
    }
    
    fn rlsh_cd(state: &mut Rlsh, args: Vec<String>) -> Result<Option<Signals>> {
        if args.len() == 1 {
            if let Some(h) = home::home_dir() {
                state.old_pwd = Some(env::current_dir()?.clone());
                env::set_current_dir(h)?;
                return Ok(None);
            }
            else { return Err(anyhow!("cd: could not find home directory")); }
        }

        if args[1] == "-" {
            if let Some(o) = state.old_pwd.take() {
                state.old_pwd = Some(env::current_dir()?);
                env::set_current_dir(o)?;
                return Ok(None);
            } else { return Err(anyhow!("cd: OLDPWD not set")); }
        }

        state.old_pwd = Some(env::current_dir()?);
        env::set_current_dir(&args[1])?;

        Ok(None)
    }

    
    fn rlsh_help(_: &mut Rlsh, _: Vec<String>) -> Result<Option<Signals>> {
        println!("SandWood Jones' RLSH (a clone of Stephen Brennan's LSH)");
        println!("Type program names and arguments, and hit enter.");
        println!("The following are built in:");

        for v in Builtins::iter() {
            println!("\t{v} - {}", v.get_message().expect("all builtin variants should have a message"));
        }

        println!("Use the man command for information on other programs.");

        Ok(None)
    }

    fn rlsh_exit(_: &mut Rlsh, _: Vec<String>) -> Result<Option<Signals>> { Ok(Some(Signals::EXIT_SIGNAL)) }
}
