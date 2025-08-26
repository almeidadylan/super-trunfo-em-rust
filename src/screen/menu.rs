use crate::{models::client::Client, screen::{
    basic_operations::clear_screen, client_services::*, read::read_int_data
}};

pub fn show_menu (client: Client) {
    loop {
        clear_screen();
        println!("\n\
        =============== Super Trunfo =============== \n\
        0 - Sair do programa \n\
        1 - Cadastrar Carta \n\
        2 - Alterar Carta \n\
        3 - Excluir uma carta \n\
        4 - Mostrar todas as cartas \n\
        ");

        let option = read_int_data();

        match option {
            0 => {
                println!("Finalizando ...");
                return;
            },
            1 => create_client(client),
            2 => println!("Opção 2"),
            3 => println!("Opção 3"),
            4 => println!("Opção 4"),
            _ => println!("Opção inválida"),
        }
    }
}