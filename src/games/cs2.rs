use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;

use menu_rs::{Menu, MenuOption};

use crate::utils::config::{load_cfg, load_commands, load_file};
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
const COMMANDS_FILE_PATH: &str = "./resources/cs2/config/commands.txt";
const EXEC_FILE_PATH: &str = "./resources/cs2/config/exec.txt";

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
    let mode_args = match mode {
        GameMode::Competitive => [
            "+game_type 0",
            "+game_mode 1",
            "+mapgroup mg_active",
            "+map de_dust2",
        ],
        GameMode::Wingman => [
            "+game_type 0",
            "+game_mode 2",
            "+mapgroup mg_active",
            "+map de_dust2",
        ],
        GameMode::Casual => [
            "+game_type 0",
            "+game_mode 0",
            "+mapgroup mg_active",
            "+map de_dust2",
        ],
        GameMode::Deathmatch => [
            "+game_type 1",
            "+game_mode 2",
            "+mapgroup mg_active",
            "+map de_dust2",
        ],
        GameMode::Custom => [
            "+game_type 3",
            "+game_mode 0",
            "+mapgroup mg_active",
            "+map de_dust2",
        ],
    };

    // load commands from file
    let path = Path::new(COMMANDS_FILE_PATH);
    let commands_args = load_commands(&path)?;
    let commands_args = commands_args
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();

    // builds exec cfg file
    let exec_cfg_file = build_exec_cfg()?;
    let exec_arg = format!("+exec {}", exec_cfg_file);

    // builds server arguments
    let mut run_args = commands_args;
    run_args.extend_from_slice(mode_args.as_slice());
    run_args.push(&exec_arg);

    println!("Starting server...");
    println!("Args: {:?}", run_args);

    let server_dir_path = server_path();

    if cfg!(windows) {
        let mut args = vec!["/c", "cs2.exe", "-dedicated"];
        args.extend_from_slice(run_args.as_slice());

        let executable_path = server_dir_path.join("game").join("bin").join("win64");
        Command::new("cmd")
            .current_dir(executable_path)
            .args(args)
            .status()?;
    } else if cfg!(unix) {
        let mut args = vec!["-dedicated"];
        args.extend_from_slice(run_args.as_slice());

        let executable_path = server_dir_path
            .join("game")
            .join("bin")
            .join("linuxsteamrt64");
        Command::new("./cs2")
            .current_dir(executable_path)
            .args(args)
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

fn build_exec_cfg() -> Result<String, Box<dyn Error>> {
    let filename = "__exec_custom.cfg";
    let path = Path::new(EXEC_FILE_PATH);
    if !path.exists() {
        println!("Can't find exec.txt file, skipping exec .cfg creation");
        return Ok(filename.to_owned());
    }

    // loads cfgs file list
    let cfgs_dir_path = Path::new("./resources").join("cs2").join("cfgs");
    let mut exec_cfg_values = Vec::new();
    let reader = load_file(&path)?;
    for line in reader.lines() {
        let cfg_file = line?;
        let cfg_file_path = cfgs_dir_path.join(cfg_file);
        println!("Loading .cfg file: {:?}", cfg_file_path);

        let mut values = load_cfg(&cfg_file_path)?;
        exec_cfg_values.append(&mut values);
    }

    // builds exec cfg file path
    let server_path = server_path();
    let server_cfgs_dir_path = server_path.join("game").join("csgo").join("cfg");
    let exec_cfg_path = server_cfgs_dir_path.join(filename);

    // deletes custom cfg if already exists
    if exec_cfg_path.exists() {
        fs::remove_file(&exec_cfg_path)?;
    }

    // creates custom cfg file
    fs::write(&exec_cfg_path, exec_cfg_values.join("\n"))?;

    Ok(filename.to_owned())
}
