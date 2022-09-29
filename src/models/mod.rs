pub mod gpt3;

use reqwest::Result;
use crate::GPT3;

pub trait Completion {
    fn completion(&self, input_text: String) -> Result<Option<String>>;
}

impl Completion for GPT3 {
    fn completion(&self, input_text: String) -> Result<Option<String>> {
        Ok(Some((*self.api_key).to_string()))
    }
}