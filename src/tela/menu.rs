use crate::tela::{
    operacoes_basicas::clear_screen, read::read_int_data
};

pub fn show_menu () {
    loop {
        clear_screen();
        println!("\n\
        =============== Super Trunfo =============== \n\
        0 - Sair do programa \n\
        1 - Cadastrar Carta \n\
        2 - Alterar Carta \n\
        3 - Excluir uma carta \n\
        4 - mostrar todas as cartas \n\
        ");

        let option = read_int_data();

        match option {
            0 => {
                println!("Finalizando ...");
                return;
            },
            1 => println!("Opção 1"),
            2 => println!("Opção 2"),
            3 => println!("Opção 3"),
            4 => println!("Opção 4"),
            _ => println!("Opção inválida"),
        }
    }
}