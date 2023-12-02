#![allow(clippy::needless_return)]

mod games;
mod utils;

use games::cs2::menu;
use menu_rs::{Menu, MenuOption};

fn main() {
    games_menu();
}

fn games_menu() {
    let menu = Menu::new(vec![
        MenuOption::new("Counter Strike 2", menu).hint("Runs the server")
    ]);
    menu.show();
}
