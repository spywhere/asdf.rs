use std::convert::From;
use crate::cli::options::Envs;

pub struct Context {
    pub data_dir: String,
}

impl From<Envs> for Context {
    fn from(envs: Envs) -> Self {
        let data_dir = envs.data_dir;
        Self { data_dir }
    }
}

mod version;
pub use version::*;
