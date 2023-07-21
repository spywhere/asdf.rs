mod install;
pub use install::{install, InstallOptions};

mod latest;
pub use latest::{latest, LatestOptions};

mod uninstall;
pub use uninstall::{uninstall, UninstallOptions};
