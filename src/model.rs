use models::GPT3;

enum KnownModel {
    GPT3: GPT3::GPT3
}

struct Model {
    model: KnownModel,
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

impl Model {
    fn init(model: Model) -> Model {
        Model {
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