use crate::{
    models::card::Card, 
    screen::{
        basic_operations::clear_screen, 
        card_services::*, 
        read::read_int_data
}};

pub fn show_menu (deck: &mut Vec<Card>) {
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
            1 => create_card(deck),
            2 => change_card(deck),
            3 => delete_card(deck),
            4 => show_all_cards(deck),
            _ => println!("Opção inválida"),
        }
    }
}