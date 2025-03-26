use ic_cdk::{caller, update};

mod ollama;
pub use ollama::*;

mod service;
pub use service::conversations::*;
pub use service::prompts::*;

#[update]
async fn process_prompt(prompt: String) -> String {
    let user = caller();
    service::prompts::user_prompt(user, prompt.as_str()).await
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
