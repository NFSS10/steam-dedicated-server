use std::error::Error;
use std::process::Command;

use menu_rs::{Menu, MenuOption};

use crate::utils::paths::{get_app_dir_path, get_steamcmd_exe_path};

pub fn menu() {
    let menu = Menu::new(vec![
        MenuOption::new("Run server", run_server_menu).hint("Runs the server"),
        MenuOption::new("Install server", install_server).hint("Server installation/verification"),
    ]);
    menu.show();
}

pub fn run_server_menu() {
    let menu = Menu::new(vec![
        MenuOption::new("Competitive", || start_server(GameMode::Competitive)),
        MenuOption::new("Wingman", || start_server(GameMode::Wingman)),
        MenuOption::new("Casual", || start_server(GameMode::Casual)),
        MenuOption::new("Deathmatch", || start_server(GameMode::Deathmatch)),
        MenuOption::new("Custom", || start_server(GameMode::Custom)),
    ]);
    menu.show();
}

const CS2_APP_ID: u32 = 730;

enum GameMode {
    Competitive,
    Wingman,
    Casual,
    Deathmatch,
    Custom,
}

fn start_server(mode: GameMode) {
    if let Err(err) = _start_server(mode) {
        eprintln!("Error running server: {}", err);
    }
}

fn install_server() {
    if let Err(err) = _install_server() {
        eprintln!("Error installing server: {}", err);
    }
}

fn _start_server(mode: GameMode) -> Result<(), Box<dyn Error>> {
    // TODO

    Ok(())
}

fn _install_server() -> Result<(), Box<dyn Error>> {
    // builds path to the CS2 server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("cs2-server");

    // builds install arguments
    let args = [
        &format!("+force_install_dir {}", server_dir_path.to_str().unwrap()),
        "+login anonymous",
        &format!("+app_update {} validate", CS2_APP_ID),
        "+quit",
    ];

    // runs install command
    if cfg!(windows) {
        let steacmd_exe_path = get_steamcmd_exe_path()?;
        Command::new(steacmd_exe_path).args(args).status()?;
    } else if cfg!(unix) {
        Command::new("steamcmd").args(args).status()?;
    }

    Ok(())
}
