use std::io; 
pub fn first_menu() -> u8 {
    let mut vybor: String = String::new();
    match io::stdin().read_line(&mut vybor) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let num: u8 = vybor.trim().parse().unwrap();
    return num;
}

pub fn menu_selection() -> u8 {
    let mut vybor: String = String::new();
    match io::stdin().read_line(&mut vybor) {
        Ok(_) => {}
        Err(e) => println!("ОШИБКА ВВОДА - {}", e),
    }
    let num: u8 = vybor.trim().parse().unwrap();
    return num;
}