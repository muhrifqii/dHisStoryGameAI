//! An extended functionality for making requests to the LLM canister on the Internet Computer that is not covered on ic_llm, specific to project use case.

use std::fmt::Display;

use candid::{ser::TypeSerialize, CandidType, Principal};
#[cfg(any(not(test), rust_analyzer))]
use ic_cdk::call;
use ic_llm::Model;
#[cfg(all(test, not(rust_analyzer)))]
use mock_ic::call;

const LLM_CANISTER: &'static str = "w36hm-eqaaa-aaaal-qr76a-cai";

/// Options for model based on Ollama docs, specific to project use case
#[derive(CandidType, Clone, Debug)]
struct ModelOptions {
    temperature: f32,
    top_p: f32,
}

impl Default for ModelOptions {
    /// fine tuned model options
    fn default() -> Self {
        Self {
            temperature: 0.9,
            top_p: 0.9,
        }
    }
}

/// The role of a `ChatMessage`, that is not covered on ic_llm v0.3.0 :(
#[derive(CandidType, Clone, Debug)]
pub enum Role {
    system,
    user,
    assistant,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Role::assistant => "assistant",
                Role::user => "user",
                Role::system => "system",
            }
        )
    }
}

/// A message in a chat.
#[derive(CandidType, Clone, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// Wrapper to communicate with llm_canister
#[derive(CandidType, Debug)]
struct Request {
    model: String,
    messages: Vec<ChatMessage>,
    options: ModelOptions,
}

impl Request {
    /// short-hand for initializing "Request"
    fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            model: Model::Llama3_1_8B.to_string(),
            messages,
            options: ModelOptions::default(),
        }
    }
}

/// Sends messages
pub async fn chat(messages: Vec<ChatMessage>) -> String {
    let llm_canister = Principal::from_text(LLM_CANISTER).expect("invalid canister id");

    let res: (String,) = call(llm_canister, "v0_chat", (Request::new(messages),))
        .await
        .unwrap();
    res.0
}

#[cfg(test)]
mod mock_ic {
    use std::future::Future;

    use candid::Principal;
    use ic_cdk::api::call::{CallResult, RejectionCode};

    use super::Request;

    /// mock ic_cdk::call specific to ic_llm v0.3.0-v0.4.0(current)
    pub fn call(
        _id: Principal,
        _method: &str,
        args: (Request,),
    ) -> impl Future<Output = CallResult<(String,)>> + Send + Sync {
        let msgs = args.0.messages;
        async move {
            msgs.last()
                .map(|z| (z.content.clone(),))
                .ok_or((RejectionCode::NoError, String::from("Empty!")))
        }
    }
}

#[cfg(test)]
mod tests {
    use ic_llm::Model;

    use super::Request;

    #[test]
    fn generate_default_request_check() {
        let req = Request::new(vec![]);
        assert!(req.messages.is_empty());
        assert_eq!(req.model, Model::Llama3_1_8B.to_string());
        assert_eq!(req.options.temperature, 0.9);
        assert_eq!(req.options.top_p, 0.9);
    }

    // // need to add tokio Cargo.toml :(
    // #[tokio::test]
    // async fn chat_should_return_expected_string() {
    //     chat(vec![ChatMessage {
    //         role: Role::User,
    //         content: "Hello World!".to_string(),
    //     }])
    //     .await;
    // }
}
