use crate::cmd::Context;

use crate::cli::stdout;
use crate::cli::Exit;

pub struct ListAllOptions {
    pub plugin: String,
    pub version: Option<String>,
}

pub fn list_all(context: Context, options: ListAllOptions) -> Result<(), Exit> {
    stdout!("list all {} in {}", options.plugin, context.data_dir);
    Ok(())
}
