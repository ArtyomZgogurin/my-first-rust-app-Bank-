use std::io;
const BANK_NAME: &str = "Банк Солбиф";
const MAX_PIN_ATTEMPTS: i8 = 3;
const RUB_TO_USD: f64 = 83.30;


struct Client {
    name: String,
    pin: String,
    rub_balance: f64,
    usd_balance: f64,
    history: Vec<String>,
}
impl Client {
    fn change_history(&mut self, change: String) {
        self.history.push(change);
    }

    fn deposit(&mut self) {
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

    fn withdraw(&mut self) {
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

    fn check_balance(&mut self) {
        println!(
            "Ваш баланс {} руб, {} USD",
            self.rub_balance, self.usd_balance
        );
        let ident: String = String::from("Проверка баланса");
        self.change_history(ident);
    }

    fn show_history(&self) {
        for i in &self.history {
            println!("{}", i);
        }
    }
    fn convert(&mut self) {
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
                    2=> {
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
                    _=>{
                        println!("Неверное число операции")
                    }
                }
            }
        }
    


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
    match first_menu() {
        1 => {
            clients.push(registration_client());
            let name: String = authorization_clent();
            let mut authorized_client: Client = chek_authorization_client(name, clients);
            chek_pincode_client(&mut authorized_client);
            loop {
                println!(
                    "{}, выберите действие:\n1 - Проверить баланс\n2 - Пополнить счет\n3 - Снять деньги\n4 - История операций\n5 - Конвертация валют\n6 - Выход",
                    &authorized_client.name
                );
                match menu_selection() {
                    1 => authorized_client.check_balance(),
                    2 => authorized_client.deposit(),
                    3 => authorized_client.withdraw(),
                    4 => authorized_client.show_history(),
                    5 => authorized_client.convert(),
                    6 =>{ break;}
                    _ => {
                        panic!("ОШИБКА")
                    }
                }
            }
        }
        2 => {
            let name: String = authorization_clent();

            if chek_client_bank(name.clone(), &clients) {
                let mut authorized_client: Client = chek_authorization_client(name, clients);
                chek_pincode_client(&mut authorized_client);
                loop {
                    println!(
                        "{}, выберите действие:\n1 - Проверить баланс\n2 - Пополнить счет\n3 - Снять деньги\n4 - История операций\n5 - Конвертация валют\n6 - Выход",
                        &authorized_client.name
                    );
                    match menu_selection() {
                    1 => authorized_client.check_balance(),
                    2 => authorized_client.deposit(),
                    3 => authorized_client.withdraw(),
                    4 => authorized_client.show_history(),
                    5 => authorized_client.convert(),
                    6 =>{ break;}
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

fn first_menu() -> u8 {
    let mut vybor: String = String::new();
    match io::stdin().read_line(&mut vybor) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let num: u8 = vybor.trim().parse().unwrap();
    return num;
}
fn registration_client() -> Client {
    println!("Регистрация в приложении банка {}", BANK_NAME);
    println!("Введите своё имя:");
    let mut client_name: String = String::new();
    match io::stdin().read_line(&mut client_name) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let name: String = client_name.trim().parse().unwrap();
    println!("Задайте PIN:");
    let pincode: String = String::new();
    match io::stdin().read_line(&mut client_name) {
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

fn authorization_clent() -> String {
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

fn chek_client_bank(name: String, base_clients: &Vec<Client>) -> bool {
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

fn chek_authorization_client(name: String, base_clients: Vec<Client>) -> Client {
    for i in base_clients {
        if i.name == name {
            return i;
        } else {
        }
    }
    panic!("ОШИБКА")
}

fn chek_pincode_client(client: &mut Client) {
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

fn menu_selection() -> u8 {
    let mut vybor: String = String::new();
    match io::stdin().read_line(&mut vybor) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let num: u8 = vybor.trim().parse().unwrap();
    return num;
}
