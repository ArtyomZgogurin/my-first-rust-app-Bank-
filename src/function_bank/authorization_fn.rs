use crate::config::BANK_NAME;
use std::io; 
use crate::struct_client::Client;

pub fn registration_client() -> Client {
    println!("Регистрация в приложении банка {}", BANK_NAME);
    println!("Введите своё имя:");
    let mut client_name: String = String::new();
    match io::stdin().read_line(&mut client_name) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let name: String = client_name.trim().parse().unwrap();
    println!("Задайте PIN:");
    let mut pincode: String = String::new();
    match io::stdin().read_line(&mut pincode) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let pin: String = pincode.trim().parse().unwrap();

    let new_client: Client = Client {
        name: (name),
        pin: (pin),
        rub_balance: (0.0),
        usd_balance: (0.0),
        history: (Vec::new()),
    };
    return new_client;
}

pub fn authorization_clent() -> String {
    println!("Авторизация в приложение банка {}", BANK_NAME);
    println!("Введите своё имя:");
    let mut client_name: String = String::new();
    match io::stdin().read_line(&mut client_name) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let name: String = client_name.trim().parse().unwrap();
    return name;
}

pub fn chek_client_bank(name: String, base_clients: &Vec<Client>) -> bool {
    let mut serch_name_in_base: i8 = 0;
    for i in base_clients {
        if i.name == name {
            serch_name_in_base += 1;
            println!("{},добро пожаловать в приложение банка{}", name, BANK_NAME);
            return true;
        } else {
        }
    }
    if serch_name_in_base == 0 {
        println!("Вы не являетесь клиентом банка");
        return false;
    } else {
        println!("Ошибка");
        return false;
    }
}

pub fn chek_authorization_client(name: String, base_clients: Vec<Client>) -> Client {
    for i in base_clients {
        if i.name == name {
            return i;
        } else {
        }
    }
    panic!("ОШИБКА")
}
