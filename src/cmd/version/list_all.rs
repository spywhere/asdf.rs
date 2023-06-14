use crate::cli::stdout;
use crate::cli::Exit;

pub fn list_all() -> Result<(), Exit> {
    stdout!("list all");
    Ok(())
}
