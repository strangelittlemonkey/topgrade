use super::utils::Check;
use failure;
#[cfg(windows)]
use std::env;
use std::path::PathBuf;
use std::process::Command;
#[cfg(unix)]
use utils::home_path;

const EMACS_UPGRADE: &str = include_str!("emacs.el");

#[cfg(unix)]
pub fn config_dir() -> PathBuf {
    home_path(".emacs.d")
}

#[cfg(windows)]
pub fn config_dir() -> PathBuf {
    [
        &env::var("APPDATA").expect("Could not find APPDATA"),
        ".emacs.d",
    ].iter()
        .collect()
}

pub fn init_file() -> PathBuf {
    config_dir().join("init.el")
}

pub fn upgrade(emacs: &PathBuf, init: &PathBuf) -> Result<(), failure::Error> {
    Command::new(&emacs)
        .args(&[
            "--batch",
            "-l",
            init.to_str().unwrap(),
            "--eval",
            EMACS_UPGRADE,
        ])
        .spawn()?
        .wait()?
        .check()?;

    Ok(())
}
