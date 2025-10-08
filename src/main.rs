use std::io;
const BANK_NAME:&str="Банк Солбиф";
const MAX_PIN_ATTEMPTS: i8 =3;
const RUB_TO_USD: f64 = 83.30;
const RUB_TO_EUR: f64 = 96.40;

struct Client {
    name: String,
    pin: String,
    rub_balance: f64,
    usd_balance: f64,
    history: Vec<String>,
}


fn main() {
    let mut clients = vec![
    ("Анна".to_string(), "1234".to_string(), 1500.0, 20.0, 5.0),
    ("Иван".to_string(), "5678".to_string(), 800.0, 100.0, 0.0),
    ];
    println!("Авторизация в приложение банка {}",BANK_NAME);
    println!("Введите своё имя:");
    let mut client_name : String = String::new();
    match io::stdin().read_line(&mut client_name) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let mut name:String = client_name.trim().parse().unwrap();
    let mut chek_try = MAX_PIN_ATTEMPTS;


    if chek_client_bank(name.clone(),clients.clone()){
    loop {  
    println!("Введите PIN:");
    let mut pincode : String = String::new();
    match io::stdin().read_line(&mut pincode) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let pin:String = pincode.trim().parse().unwrap();
    if chek_try>0 {if pin==clients[0].1{
        break;
    } else {
        chek_try-=1;
        println!("PIN CODE неверный, попробуйте еще раз. Количество оствшихся попыток{}",chek_try);
        if chek_try==0{
            println!("В доступе отказано, попытки ввода исчерпаны");
            break;
        } 
    }} 
    }

    let mut historic_operations:Vec<String> = Vec::new();
    loop {
        println!("{}, выберите действие:\n1 - Проверить баланс\n2 - Пополнить счет\n3 - Снять деньги\n4 - История операций\n5 - Выйти",&name);
        let mut vybor : String = String::new();
        match io::stdin().read_line(&mut vybor) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА - {}", e),
        }
        let  num: i8 = vybor.trim().parse().unwrap();
        match num {
            1 => {check_balance(clients[0].clone(),&mut historic_operations)}
            2 => {
                deposit(clients[0].clone(),&mut historic_operations);

            }
            3 => {
                withdraw(clients[0].clone(),&mut historic_operations);
            }
            4 => {
                show_history(&mut historic_operations);

            }
            5 =>{
                break;
            }
            _ => {

            }

        }
    }} else {
        println!("Досвидос")
    }


}
     
fn deposit(mut client:(String,String,f64,f64,f64),history_schet:&mut Vec<String>)-> ( String,String,f64,f64,f64 )  {
    println!("Укажите сумму пополнения:");
    let mut change_schet : String = String::new();
    match io::stdin().read_line(&mut change_schet){
    Ok(_) => {}
    Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let up_schet: f64 = change_schet.trim().parse().unwrap();
    client.2+=up_schet;
    println!("Счет пополнен, баланс увеличен на {}",up_schet);
    let ident: String = String::from("Пополнение:");
    history_schet.push(ident);
    history_schet.push(change_schet);
    return client

}
fn withdraw(mut client:(String,String,f64,f64,f64),history_schet:&mut Vec<String>)-> ( String,String,f64,f64,f64 ) {
   println!("Укажите сумму которую хотите снять:");
    let mut change_schet : String = String::new();
    match io::stdin().read_line(&mut change_schet){
    Ok(_) => {}
    Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let up_schet: f64 = change_schet.trim().parse().unwrap();
    client.2-=up_schet;
    println!("Сумма {} списана с вашего счета",up_schet);
    let ident: String = String::from("Снятие:");
    history_schet.push(ident);
    history_schet.push(change_schet);
    return client 
}
fn show_history (history_schet:&mut Vec<String>) {
    for i in history_schet{
        println!("{}",i);
    }}
fn chek_client_bank (name: String, base_clients:Vec<(String,String,f64,f64,f64)> ) -> bool{
    let mut serch_name_in_base: i8=0;
    for i in base_clients {
        if i.0==name{
            serch_name_in_base+=1;
            println!("{},добро пожаловать в приложение банка{}",name,BANK_NAME);
            return true
        } else{}}
    if serch_name_in_base==0  {
        println!("Вы не являетесь клиентом банка");
        return false
    } else{
        println!("Ошибка");
        return false}
}
fn check_balance(mut client:(String,String,f64,f64,f64),history_schet:&mut Vec<String>){
    println!("Ваш баланс {} руб, {} USD, {} EUR",client.2,client.3,client.4);
    let ident: String = String::from("Проверка баланса");
    history_schet.push(ident);

}

