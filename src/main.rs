use async_openai::{
    error::OpenAIError,
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs},
    Client,
};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    prompt: String,
}

static SYSTEM_MESSAGE : &str = "You are a linux command line assistant. The user will provide a description of the task he wants to perform on the terminal and your goal is to provide a command or commands with detailed explanation";
static MODEL: &str = "gpt-4";

#[tokio::main]
async fn main() -> Result<(), OpenAIError> {
    let args = Cli::parse();

    println!("Asking chatGPT ...");

    let client = Client::new();

    let system_message = ChatCompletionRequestMessage {
        role: async_openai::types::Role::System,
        content: SYSTEM_MESSAGE.to_string(),
        name: None,
    };

    let task_message = ChatCompletionRequestMessage {
        role: async_openai::types::Role::User,
        content: format!("How do I '{}' ? ", args.prompt),
        name: None,
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(MODEL)
        .messages(vec![system_message, task_message])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        println!("{}", choice.message.content)
    }

    Ok(())
}
