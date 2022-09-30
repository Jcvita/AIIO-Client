use dotenv::dotenv;
use notify_rust::Notification;
use clipboard::{ClipboardProvider, ClipboardContext};
use std::{thread, time};
//https://github.com/obv-mikhail/InputBot for global keyboard inputs
use crate::models::gpt3::{
    GPT3, 
    GPT3Model
};

pub mod models;

//takes in any model that has the .completion()
fn generate_code_comments(model: GPT3, input_code: String, input_language: Option<String>) -> String {
    "".to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    //Initialize clipboard access
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut ai: GPT3 = GPT3::init(GPT3Model::Davinci);
    let result: String = match ai
        .tokens(64)
        .temperature(0.25)
        .completion("The quick brown fox").await {
            Ok(out) => out,
            Err(e) => panic!("Completion failed: {:?}", e)
        };
    
    match Notification::new()
        .appname("GPT3")
        .body(&result)
        .show() {
            Ok(_) => {},
            Err(e) => panic!("Notification failed to fire: {:?}", e)
        };
}