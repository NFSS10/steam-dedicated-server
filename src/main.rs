#![allow(clippy::needless_return)]

mod games;
mod utils;

use std::error::Error;
use std::io::stdin;
use std::process::Command;

use games::cs2::menu;
use menu_rs::{Menu, MenuOption};
use utils::paths::get_steamcmd_exe_path;

fn main() {
    // checks if SteamCMD is set up correctly
    match verify_os() {
        Ok(_) => {}
        Err(_) => println!("Setup required. Please ensure that SteamCMD is correctly set up."),
    }

    games_menu();

    // avoids immediately closing the console in Windows
    if cfg!(windows) {
        println!("\nPress any key to close");
        stdin().read_line(&mut String::new()).unwrap();
    }
}

fn games_menu() {
    let menu = Menu::new(vec![
        MenuOption::new("Counter Strike 2", menu).hint("Runs the server")
    ]);
    menu.show();
}

fn verify_os() -> Result<(), Box<dyn Error>> {
    if cfg!(windows) {
        let steacmd_exe_path = get_steamcmd_exe_path()?;
        if !steacmd_exe_path.exists() {
            Err("steamcmd.exe not found")?;
        }
    } else if cfg!(unix) {
        Command::new("steamcmd").arg("+quit").output()?;
    }

    Ok(())
}
