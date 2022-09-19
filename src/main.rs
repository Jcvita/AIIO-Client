use dotenv::dotenv;

fn main() {
    dotenv().ok();
    // get openai token from .env file
    // let openai_api_token = std::env::var("OPENAI_API_KEY").expect("OpenAI api token was not set.");
}
