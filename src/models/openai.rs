use serde;
use serde_json::from_value;

// #[derive(serde::Deserialize, Debug)]
#[derive(Debug)]

pub struct OpenAIError {
    message: String,
    _type: String,
    param: i32,
    code: i32
}

// #[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
pub struct ErrorResponse {
    error: OpenAIError
}

// #[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
pub struct Choice {
    text: String,
    index: i32,
    lobprobs: i32,
    finish_reason: String
}

// #[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

// #[derive(serde::Deserialize, Debug)]
#[derive(Debug)]
pub struct Completion {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<Choice>,
    usage: Usage
}

// #[derive(Deserialize, Debug)]
#[derive(Debug)]
// #[serde(untagged)]
pub enum Output {
    Completion(Completion),
    ErrorResponse(ErrorResponse)
}

// pub trait Deserialize<'de>: Sized {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>;
// }

// impl<'de> Deserialize<'de> for Output {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: serde::Deserializer<'de>
//     {
//         let helper : Value = serde::Deserialize::deserialize(deserializer)?;

//         use self::Output::*;

//         let env = if helper.find("error") != None {
//             ErrorResponse(from_value(helper).unwrap())
//         } else if helper.find("id") != None {
//             Completion(from_value(helper).unwrap())
//         } else {
//             Other(from_value(helper).unwrap())
//         };
        
//     }
// }