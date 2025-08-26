mod screen;
mod models;

use screen::menu::show_menu;
use models::client::Client;

fn main() {
    let mut client: Vec<Client> = Vec::new();
    show_menu(client);
}
