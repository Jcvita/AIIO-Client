pub mod gpt3;
pub mod openai;
// uncomment the below when async traits work with reqwest

// use reqwest::Result;
// use async_trait::async_trait;
// use serde_json::{json, Value};
// use crate::GPT3;

// #[async_trait]
// pub trait Completion {
//     async fn completion(&self, input_text: String);
// }

// //'r lifetime because of async trait
// #[async_trait]
// impl Completion for GPT3 {
//     async fn completion(& self, input_text: String) {
//         let mut stop_sequences: Vec<Value> = vec![];
//         let mut data = json!({
//             "model": self.model.to_string(),
//             "temperature": self.temperature,
//             "max_tokens": self.tokens,
//             "stop_sequence": self.stop_sequences,
//             "top_p": self.top_p,
//             "frequency_penalty": self.freq_penalty,
//             "presence_penalty": self.presence_penalty,
//             "best_of": self.best_of,
//             "n": 1,
//         });

//         let client = reqwest::Client::new();
//         let res = client.post("https://api.openai.com/v1/completions")
//             .header("Authorization", format!("Bearer {}", (*self.api_key).to_string()))
//             .header("Content-Type", "application/json")
//             .json(&data)
//             .send()
//             .await;
//     }
// }