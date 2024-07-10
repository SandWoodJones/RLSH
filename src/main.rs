use anyhow::Result;

mod shell;

fn main() -> Result<()>{
    shell::Rlsh::run()?;

    Ok(())
}
