use menu_rs::{Menu, MenuOption};

use crate::utils::paths::get_app_dir_path;

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
    // TODO
}

fn install_server() {
    // builds path to the CS2 server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("cs2-server");

    let args = [
        &format!("+force_install_dir {}", server_dir_path.to_str().unwrap()),
        "+login anonymous",
        &format!("+app_update {} validate", CS2_APP_ID),
        "+quit",
    ];

    println!("Installing CS2 server...");
    println!("TODO: {:?}", args);
}
