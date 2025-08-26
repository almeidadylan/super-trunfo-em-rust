use crate::{
    models::client::Client, 
    screen::{
        basic_operations::clear_screen, read::read_string_data
    }
};

fn enter_customer_data (client: &mut Client) {
    println!("Digite o nome");
    client.name = read_string_data();

    println!("Digite o cpf");
    client.cpf = read_string_data();

    println!("Digite o endereço");
    client.address = read_string_data();
}

pub fn create_client (clients: &mut Vec<Client>) {
    clear_screen();

    let mut client = Client::default();

    client.id = clients.len() + 1;

    enter_customer_data(&mut client);

    clients.push(client);

    clear_screen();
    println!("Carta cadastrada com sucesso");
}