use dotenv::dotenv;
use notify_rust::Notification;
//https://github.com/obv-mikhail/InputBot for global keyboard inputs
use crate::models::Completion;
use crate::models::gpt3::{
    GPT3, 
    GPT3Model
};

pub mod models;
fn main() {
    dotenv().ok();

    let mut ai = GPT3::init(GPT3Model::Davinci);
    let result = match ai
        .tokens(64)
        .temperature(0.25)
        .completion(String::from("The quick brown fox")) {
        Ok(res) => res.unwrap(),
        Err(e) => panic!("Error: {:?}", e)
    };
    
    Notification::new()
        .appname("GPT3")
        .body(&result)
        .show();
}