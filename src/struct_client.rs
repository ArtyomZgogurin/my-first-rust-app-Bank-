use std::io;

use crate::config::RUB_TO_USD;
use crate::config::BANK_NAME;
use crate::config::MAX_PIN_ATTEMPTS;

use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

const PATH_HISTORY: &str ="history_log_client/";
pub struct Client {
    pub name: String,
    pub pin: String,
    pub rub_balance: f64,
    pub usd_balance: f64,
    pub history: Vec<String>,
    
}
impl Client {
    pub fn change_history(&mut self, change: String) {
        self.history.push(change);
    }

    pub fn deposit(&mut self) {
        println!("Укажите сумму пополнения:");
        let mut change_schet: String = String::new();
        match io::stdin().read_line(&mut change_schet) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА - {}", e),
        }
        let up_schet: f64 = change_schet.trim().parse().unwrap();
        self.rub_balance += up_schet;
        println!("Счет пополнен, баланс увеличен на {}", up_schet);
        let ident: String = String::from("Пополнение:");
        self.change_history(ident);
        self.change_history(change_schet);
    }

    pub fn withdraw(&mut self) {
        println!("Укажите сумму которую хотите снять:");
        let mut change_schet: String = String::new();
        match io::stdin().read_line(&mut change_schet) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА - {}", e),
        }
        let up_schet: f64 = change_schet.trim().parse().unwrap();
        self.rub_balance -= up_schet;
        println!("Сумма {} списана с вашего счета", up_schet);
        let ident: String = String::from("Снятие:");
        self.change_history(ident);
        self.change_history(change_schet);
    }

    pub fn check_balance(&mut self) {
        println!(
            "Ваш баланс {} руб, {} USD",
            self.rub_balance, self.usd_balance
        );
        let ident: String = String::from("Проверка баланса");
        self.change_history(ident);
    }

    pub fn show_history(&self) {
        for i in &self.history {
            println!("{}", i);
        }
    }
    pub fn convert(&mut self) {
        println!("Выберите валюты которую хотите конвертировать:\n1.Рубль\n2.USD");
        let mut choice_convert: String = String::new();
        match io::stdin().read_line(&mut choice_convert) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА - {}", e),
        }
        let num: u8 = choice_convert.trim().parse().unwrap();
        match num {
            1 => {
                println!("Укажите сумму которую хотите конвертировать из Рубли в USD:");
                let mut change_schet: String = String::new();
                match io::stdin().read_line(&mut change_schet) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА - {}", e),
                }
                let up_schet: f64 = change_schet.trim().parse().unwrap();
                self.rub_balance -= &up_schet;
                self.usd_balance += &up_schet / RUB_TO_USD;
                let ident: String = String::from("Конвертация из Рубли в USD.");
                self.change_history(ident);
            }
            2 => {
                println!("Укажите сумму которую хотите конвертировать из USD в Рубли:");
                let mut change_schet: String = String::new();
                match io::stdin().read_line(&mut change_schet) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА - {}", e),
                }
                let up_schet: f64 = change_schet.trim().parse().unwrap();
                self.usd_balance -= &up_schet;
                self.rub_balance += &up_schet * RUB_TO_USD;
                let ident: String = String::from("Конвертация из USD в Рубли.");
                self.change_history(ident);
            }
            _ => {
                println!("Неверное число операции")
            }
        }
    }
    pub fn history_in_file(&self) {
    let path = PATH_HISTORY.to_owned()+&self.name.to_string() + ".txt";
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
    {
        for i in &self.history {
            let _ = writeln!(file, "{}", i);
        }
    }
}
}
