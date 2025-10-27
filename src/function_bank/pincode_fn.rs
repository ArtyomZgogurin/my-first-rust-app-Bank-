use std::io; 
use crate::struct_client::Client;

pub fn chek_pincode_client(client: &mut Client) {
    let mut chek_try = 3;
    loop {
        println!("Введите PIN:");
        let mut pincode: String = String::new();
        match io::stdin().read_line(&mut pincode) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА - {}", e),
        }
        let pin: String = pincode.trim().parse().unwrap();
        if chek_try > 0 {
            if pin == client.pin {
                println!("PIN CODE верный");
                println!("Вы авторизованы в личном кабинете");
                break;
            } else {
                chek_try -= 1;
                println!(
                    "PIN CODE неверный, попробуйте еще раз. Количество оствшихся попыток{}",
                    chek_try
                );
                if chek_try == 0 {
                    println!("В доступе отказано, попытки ввода исчерпаны");
                    break;
                }
            }
        }
    }
}