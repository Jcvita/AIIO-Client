use std::env; //environment variables 

enum GPT3Model {
    text_davinci_002,
    text_curie_001,
    text_babbage_001,
    text_ada_001,
    code_davinci_002,
    code_cushman_001,
}

struct GPT3 {
    model: GPT3Model,
    temperature: f32,
    tokens: u16,
    stop_sequences: Vec<String>,
    top_p: f32,
    freq_penalty: f32,
    presence_penalty: f32,
    best_of: u8,
    start_text: String,
    restart_text: String
}

impl GPT3 {
    fn init(model: GPT3Model) -> GPT3 {
        GPT3 {
            model,
            temperature: 0.7,
            tokens: 256,
            stop_sequences: Vec![],
            top_p: 1,
            freq_penalty: 0,
            presence_penalty: 0,
            best_of: 1,
            start_text: "",
            restart_text: ""
        }
    }
}