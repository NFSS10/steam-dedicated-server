use menu_rs::{Menu, MenuOption};

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
    // TODO
}
