use crate::models::openai::*;

use std::fmt;
use std::env; use reqwest::Client;
//environment variables 
use reqwest;
use serde_json::{json, Value, from_value};

//TODO fetch available models from GET https://api.openai.com/v1/models
#[derive(Debug, Clone, Copy)]
pub enum GPT3Model {
    Davinci,
    Curie,
    Babbage,
    Ada,
    CodexAccurate,
    CodexFast
}

// allows GPT3Model::<model> to have the .to_string(); function 
impl fmt::Display for GPT3Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GPT3Model::Davinci => write!(f, "text_davinci_002"),
            GPT3Model::Curie => write!(f, "text_curie_001"),
            GPT3Model::Babbage => write!(f, "text_babbage_001"),
            GPT3Model::Ada => write!(f, "text_ada_001"),
            GPT3Model::CodexAccurate => write!(f, "code_davinci_002"),
            GPT3Model::CodexFast => write!(f, "code_cushman_001")
        }
    }
}
pub struct GPT3 {
    pub model: GPT3Model,
    pub temperature: f32,
    pub tokens: u16,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub freq_penalty: f32,
    pub presence_penalty: f32,
    pub best_of: u8,
    pub api_key: String,
    rest_client: Client
}

impl GPT3 {
    // Initializes GPT3 with defaults on the playground
    pub fn init(model: GPT3Model) -> GPT3 {
        let api_key = match env::var_os("OPENAI_API_KEY") {
            Some(key) => key.into_string().unwrap(),
            None => panic!("$OPENAI_API_KEY is not set")
        };

        let rest_client = reqwest::Client::new();

        GPT3 {
            model,
            temperature: 0.7,
            tokens: 256,
            stop_sequences: vec![],
            top_p: 1.0,
            freq_penalty: 0.0,
            presence_penalty: 0.0,
            best_of: 1,
            api_key,
            rest_client
        }
    }

    pub fn temperature(&mut self, temp: f32) -> &mut GPT3 {
        self.temperature = temp;
        self
    }
    pub fn tokens(&mut self, num_tokens: u16) -> &mut GPT3 {
        self.tokens = num_tokens;
        self
    }
    pub fn stop_sequences(&mut self, stop_sequences: &Vec<String>) -> &mut GPT3 {
        self.stop_sequences = stop_sequences.to_vec();
        self
    }
    pub fn top_p(&mut self, top_p: f32) -> &mut GPT3 {
        self.top_p = top_p;
        self
    }
    pub fn freq_penalty(&mut self, penalty: f32) -> &mut GPT3 {
        self.freq_penalty = penalty;
        self
    }
    pub fn presence_penalty(&mut self, penalty: f32) -> &mut GPT3 {
        self.presence_penalty = penalty;
        self
    }
    pub fn best_of(&mut self, num_completions: u8) -> &mut GPT3 {
        self.best_of = num_completions;
        self
    }

    pub async fn completion(&self, prompt: &str) -> reqwest::Result<String> {
        let client = &self.rest_client;

        let data = json!({
                "model": self.model.to_string(),
                "prompt": prompt,
                "temperature": self.temperature,
                "max_tokens": self.tokens,
                "stop_sequence": self.stop_sequences,
                "top_p": self.top_p,
                "frequency_penalty": self.freq_penalty,
                "presence_penalty": self.presence_penalty,
                "best_of": self.best_of,
                "n": 1,
        });
        let res = client.post("https://api.openai.com/v1/completions")
            .header("Authorization", format!("Bearer {}", (*self.api_key).to_string()))
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?
            .json::<Value>()
            .await?;
        // let output = from_value(res).unwrap();
        
        //TODO classify res value and conver to corresponding object. make this process easier to define structs as output

        Ok(String::from(""))
        
    }
}