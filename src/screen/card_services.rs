use crate::{
    models::card::Card,
    screen::{basic_operations::*, read::{read_int_data, read_string_data, read_float_data}},
};

//#[derive(Debug)]

// Pega os dados da carta via input do usuario
fn enter_card_data (card: &mut Card) {
    println!("Digite o nome do animal: ");
    card.name = read_string_data();

    println!("Digite o código do animal(A1): ");
    card.code = read_string_data();

    println!("Digite a altura do animal(m): ");
    card.height = read_float_data();

    println!("Digite o comprimento do animal(m): ");
    card.length = read_float_data();

    println!("Digite o peso do animal(kg): ");
    card.weight = read_float_data();

    println!("Digite a velocidade do animal(km/h): ");
    card.speed = (read_int_data()) as u8;

    println!("Digite o instinto assino do animal(1-10): ");
    card.killer_instinct = (read_int_data()) as u8;
}

// cria a carta usando os dados de input
pub fn create_card(deck: &mut Vec<Card>) {

    let mut card = Card {
        id: deck.len() + 1,
        name: "".to_string(),
        code: "A1".to_string(),
        height: 1.0,
        weight: 1.0,
        length: 1.0,
        speed: 1,
        killer_instinct: 1
    };
    enter_card_data(&mut card);

    deck.push(card);

    println!("Carta cadastrada com sucesso");
    wait(2);
}

// verificar se a carta existe
fn there_are_no_cards(deck: &[Card]) -> bool {
    if deck.len() == 0 {
        println!("Não existem cartas cadastradas");
        wait(2);
        return true;
    }
    return false;
}

// mostra os valores de uma carta
fn show_card(card: &Card) {
    println!(
        "\
        ID: {}\n\
        Nome: {}\n\
        Código: {}\n\
        Altura: {:.2} m\n\
        Comprimento: {:.2} m\n\
        Peso: {:.3} kg\n\
        Velocidade: {}\n\
        Instinto Assassino: {}
    ",
        card.id,
        card.name,
        card.code,
        card.height,
        card.length,
        card.weight,
        card.speed,
        card.killer_instinct
    );
}

// mostra todas as cartas
pub fn show_all_cards(deck: &Vec<Card>) {
    if there_are_no_cards(deck) {
        return;
    }
    clear_screen();

    println!();
    println!("{}", "-".to_string().repeat(40));

    for card_value in deck {
        show_card(card_value);
        println!("{}", "-".to_string().repeat(40));
    }

    println!();
    println!(" Digite 'q' para voltar ao menu principal ");

    let input = read_string_data();

    if input == String::from("q") {
        return;
    }
}

// Pega o input do id que o usuario quer alterar
fn get_card_id() -> usize {
    clear_screen();
    println!("Digite o id da carta");
    read_int_data()
}


// Altera os valores da carta
pub fn change_card (deck: &mut Vec<Card>) {
    clear_screen();

    if there_are_no_cards(deck) {
        return;
    }

    let id = get_card_id(); 
    if let Some( ( index, card ) ) = search_card_by_id(deck, id){
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));

        show_card(card);
        println!("{}", "-".to_string().repeat(40));

        enter_card_data(&mut deck[index] );
        clear_screen();
        println!("Cliente alterado com sucesso");
    } else {
        clear_screen();
        println!("Carta não encontrada.");
        wait(2);
    }
}

// procura o id da carta no banco de dados
fn search_card_by_id ( deck: &Vec<Card>, id: usize ) -> Option<( usize, &Card )>{
    deck.iter().enumerate().find( |(_, card) | card.id == id )
}

pub fn delete_card (deck: &mut Vec<Card>) {
    clear_screen();

    if there_are_no_cards(deck) {
        return;
    }

    let id = get_card_id();
    if let Some( (index, card) ) = search_card_by_id(deck, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Confirma a exclusão do cliente abaixo?");
        println!("{}", "-".to_string().repeat(40));

        show_card(card);

        println!("{}", "-".to_string().repeat(40));
        println!("s - Sim\nn - Não");
        let option = read_string_data();

        if option == "s" {
            deck.remove(index);

            clear_screen();
            println!("Cliente exluído com sucesso");
            wait(2);
        } 
        if option == "n" {
            clear_screen();
            println!("Você escolheu não excluir a carta");
            wait(2);
        }
        else {
            clear_screen();
            println!("Você digitou uma opção inválida");
            wait(2);
        }
    } else {
        clear_screen();
        println!("Cliente não encontrado!");
        wait(2);
    }
}