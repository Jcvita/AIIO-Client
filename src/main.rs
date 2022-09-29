use dotenv::dotenv;
use notify_rust::Notification;
//https://github.com/obv-mikhail/InputBot for global keyboard inputs

use crate::models::gpt3::{
    GPT3, 
    GPT3Model
};

pub mod models;
fn main() {
    dotenv().ok();

    let mut ai = GPT3::init(GPT3Model::Davinci);
    let result = match ai
        .tokens(&64)
        .completion(String::from("The quick brown fox")) {
        Ok(res) => res.unwrap(),
        Err(e) => panic!("Error: {:?}", e)
    };
    
    Notification::new()
        .appname("GPT3")
        .body(&result)
        .show();
}

//Caesar cipher example code 
//caesar cipher
// println!("Enter the text to be encrypted");
// let mut text = String::new();
// io::stdin().read_line(&mut text).expect("Failed to read line");
// let text = text.trim();
// println!("Enter the key");
// let mut key = String::new();
// io::stdin().read_line(&mut key).expect("Failed to read line");
// let key: u8 = key.trim().parse().expect("Please type a number!");
// let mut encrypted_text = String::new();
// for c in text.chars() {
//     let mut c = c as u8;
//     if c >= 65 && c <= 90 {
//         c = ((c - 65 + key) % 26) + 65;
//     } else if c >= 97 && c <= 122 {
//         c = ((c - 97 + key) % 26) + 97;
//     }
//     encrypted_text.push(c as char);
// }
// println!("Encrypted text: {}", encrypted_text);