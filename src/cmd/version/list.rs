use crate::cmd::Context;

use crate::cli::stdout;
use crate::cli::Exit;

pub struct ListOptions {
    pub plugin: Option<String>,
    pub version: Option<String>,
}

pub fn list(context: Context, options: ListOptions) -> Result<(), Exit> {
    if let Some(plugin) = options.plugin {
        stdout!("list {} in {}", plugin, context.data_dir);
    } else {
        stdout!("list in {}", context.data_dir);
    }
    Ok(())
}
