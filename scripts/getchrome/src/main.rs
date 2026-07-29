//! Downloads a Chrome for Testing build (browser + matching ChromeDriver) to a local directory.
//!
//! The download directory can be passed as the first argument, and defaults to the
//! `.chrome-for-testing` directory in the root of the repository.

use std::path::PathBuf;

fn main() {
    env_logger::init();

    let install_dir = std::env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(getchrome::default_install_dir);
    let chrome = getchrome::download(&install_dir).expect("failed to download chrome-for-testing");

    println!("chrome-for-testing {}", chrome.version);
    println!("chrome:       {}", chrome.chrome.display());
    println!("chromedriver: {}", chrome.chromedriver.display());
}
