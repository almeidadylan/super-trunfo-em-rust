mod screen;
mod models;

use screen::menu::show_menu;
use models::card::Card;

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    show_menu(&mut deck);
}
