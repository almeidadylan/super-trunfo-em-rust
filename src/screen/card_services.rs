use crate::{
    models::card::Card, 
    screen::{
        basic_operations::* , 
        read::read_string_data
    }
};


/*fn enter_customer_data (client: &mut Client) {
    println!("Digite o nome");
    client.name = read_string_data();

    println!("Digite o cpf");
    client.cpf = read_string_data();

    println!("Digite o endereço");
    client.address = read_string_data();
}*/
//#[derive(Debug)]

pub fn create_client (deck: &mut Vec<Card>) {
    //clear_screen();
    println!("{:?}", deck);

    let teste = Card { id: 1, name: "Harpia".to_string(), code: "7B".to_string(), height: 1.00, weight: 10.5,  length: 3.5, speed: 9, killer_instinct: 4 };

    deck.push(teste);

   // let mut client = Client::default();

  //  client.id = clients.len() + 1;

  //  enter_customer_data(&mut client);

   // clients.push(client);

   // clear_screen();
    println!("Carta cadastrada com sucesso");
    wait(2);
}

pub fn show_card(card: &Card) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        Código: {}\n\
        Altura: {}\n\
        Comprimento: {}\n\
        Peso: {}\n\
        Velocidade: {}\n\
        Instinto Assassino: {}
    ", card.id, card.name, card.code, card.height, card.length, card.weight, card.speed, card.killer_instinct);
}


pub fn show_all_cards (deck: &Vec<Card>) {
   loop {
        clear_screen();

        println!(" Digite 'q' para voltar ao menu principal ");
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
          break;
       }
   }
}