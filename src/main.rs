mod function_bank;
mod config;
mod struct_client;
use struct_client::Client;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;




fn main() {
    let mut clients: Vec<Client> = vec![
        Client {
            name: "Анна".to_string(),
            pin: "1234".to_string(),
            rub_balance: 1500.0,
            usd_balance: 20.0,
            history: Vec::new(),
        },
        Client {
            name: "Иван".to_string(),
            pin: "5678".to_string(),
            rub_balance: 800.0,
            usd_balance: 100.0,
            history: Vec::new(),
        },
    ];
    println!("Здравствуйте, выберите действие:\n1.Зарегестрироваться\n2.Авторизоваться\n3.Выход");
    match function_bank::menu_fn::first_menu() {
        1 => {
            clients.push(function_bank::authorization_fn::registration_client());
            let name: String = function_bank::authorization_fn::authorization_clent();
            let mut authorized_client: Client = function_bank::authorization_fn::chek_authorization_client(name, clients);
            function_bank::pincode_fn::chek_pincode_client(&mut authorized_client);
            loop {
                println!(
                    "{}, выберите действие:\n1 - Проверить баланс\n2 - Пополнить счет\n3 - Снять деньги\n4 - История операций\n5 - Конвертация валют\n6 - Выход",
                    &authorized_client.name
                );
                match function_bank::menu_fn::menu_selection() {
                    1 => authorized_client.check_balance(),
                    2 => authorized_client.deposit(),
                    3 => authorized_client.withdraw(),
                    4 => authorized_client.show_history(),
                    5 => authorized_client.convert(),
                    6 => {
                        authorized_client.history_in_file();
                        break;
                    }
                    _ => {
                        panic!("ОШИБКА")
                    }
                }
            }
        }
        2 => {
            let name: String = function_bank::authorization_fn::authorization_clent();

            if function_bank::authorization_fn::chek_client_bank(name.clone(), &clients) {
                let mut authorized_client: Client = function_bank::authorization_fn::chek_authorization_client(name, clients);
                function_bank::pincode_fn::chek_pincode_client(&mut authorized_client);
                loop {
                    println!(
                        "{}, выберите действие:\n1 - Проверить баланс\n2 - Пополнить счет\n3 - Снять деньги\n4 - История операций\n5 - Конвертация валют\n6 - Выход",
                        &authorized_client.name
                    );
                    match function_bank::menu_fn::menu_selection() {
                        1 => authorized_client.check_balance(),
                        2 => authorized_client.deposit(),
                        3 => authorized_client.withdraw(),
                        4 => authorized_client.show_history(),
                        5 => authorized_client.convert(),
                        6 => {authorized_client.history_in_file();
                            break;
                        }
                        _ => {
                            panic!("ОШИБКА")
                        }
                    }
                }
            } else {
                panic!("Вы не являетесь клиентом банка")
            }
        }
        3 => {
            println!("Досвидания")
        }
        _ => {
            panic!("Ошибка главного меню")
        }
    }
}

