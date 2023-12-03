use std::error::Error;
use std::path::{PathBuf, Path};
use std::process::Command;

use menu_rs::{Menu, MenuOption};

use crate::utils::paths::{get_app_dir_path, get_steamcmd_exe_path};
use crate::utils::config::load_commands;

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
const COMMANDS_FILE_PATH: &str = "./resources/cs2/config/commands.txt";


enum GameMode {
    Competitive,
    Wingman,
    Casual,
    Deathmatch,
    Custom,
}

fn start_server(mode: GameMode) {
    if let Err(err) = _start_server(mode) {
        let msg = format!(
            "{} {}",
            "Error running the server!",
            "Make sure the server is installed before trying to run it."
        );
        eprintln!("{} Error: {}", msg, err);
    }
}

fn install_server() {
    if let Err(err) = _install_server() {
        eprintln!("Error installing server: {}", err);
    }
}

fn _start_server(mode: GameMode) -> Result<(), Box<dyn Error>> {
    let args = match mode {
        GameMode::Competitive => "+game_type 0 +game_mode 1 +mapgroup mg_active +map de_dust2",
        GameMode::Wingman => "+game_type 0 +game_mode 2 +mapgroup mg_active +map de_dust2",
        GameMode::Casual => "+game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2",
        GameMode::Deathmatch => "+game_type 1 +game_mode 2 +mapgroup mg_active +map de_dust2",
        GameMode::Custom => "+game_type 3 +game_mode 0 +mapgroup mg_active +map de_dust2",
    };

    let server_dir_path = server_path();

    println!("Starting server...");
    println!("Args: {}", args);

    // load commands from file
    let path = Path::new(COMMANDS_FILE_PATH);
    let commands_args = load_commands(&path)?;
    let commands_args = commands_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    
    if cfg!(windows) {
        let executable_path = server_dir_path.join("game").join("bin").join("win64");
        Command::new("cmd")
            .current_dir(executable_path)
            .args(["/c", "cs2.exe", "-dedicated"])
            // TODO .arg(args)
            .status()?;
    } else if cfg!(unix) {
        let executable_path = server_dir_path
            .join("game")
            .join("bin")
            .join("linuxsteamrt64");
        Command::new("./cs2")
            .current_dir(executable_path)
            .args(["-dedicated"])
            // TODO args
            .status()?;
    } else {
        Err("OS not supported")?;
    }

    Ok(())
}

fn _install_server() -> Result<(), Box<dyn Error>> {
    let server_dir_path = server_path();

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

fn server_path() -> PathBuf {
    // builds path to the CS2 server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("cs2-server");
    return server_dir_path;
}
