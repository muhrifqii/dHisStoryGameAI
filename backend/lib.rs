pub mod prompts {
    use candid::Principal;

    use crate::utils::{combine_conversation, SingleConversation};

    use super::conversations::{get_history, update_history};
    use ic_llm::{chat, ChatMessage, Role};

    const SYSTEM_PROMPT: &'static str =
    "You are an interactive storyteller for a historical event or story, a game with an immersive, branching narrative. Your task is to continue the story based on previous context and the user's decision.\n\nYour response must be guided by the following instructions:
    1. Narative Generation:
        - Once the user provides their description, use it as the foundation for the story.
        - Generate a richly detailed narrative that is immersive and uses vivid language to portray the historical setting.
        - Always response with only 1 paragraph with maximum of 500 characters.
        - Incorporate factual historical elements of the chosen location while introducing imaginative twists based on the user's input.
        - Structure narrative branches as bullet lists. For example:
            - * Explore the ancient marketplace.
            - * Visit the royal palace.
        - Balance historical accuracy with creative storytelling. Let historical facts form the backbone of the narrative while allowing creative twists based on user input.
    2. If the user's input is not relevant to the game (for example, if it strays from the historical adventure narrative), respond with a message explaining that you cannot process such input because it is outside the scope of the game.
    3. Branching and Decisions:
        - Present the available narrative choices as a bullet list.
        - Allow the user the freedom to type in an additional choice if it makes sense, ensuring creativity while discouraging irrelevant or nonsensical input.
    4. Tone and Style:
        - Use a friendly, engaging, and slightly dramatic tone that captures the grandeur of historical events while remaining accessible.
        - Provide clear, vivid descriptions of settings, characters, and events.
    ---
    ";

    const START_GAME_PHRASE: &'static str = "I want to start the game";

    const HISTORY_THRESHOLD: usize = 5;

    /// system message prompt that allow a context to be appended
    fn system_message_prompt(added_context: Option<&str>) -> ChatMessage {
        let mut sys_prompt = SYSTEM_PROMPT.to_string();
        if let Some(context) = added_context {
            sys_prompt = format!("{}\n\n{}", sys_prompt, context);
        }
        ChatMessage {
            role: Role::System,
            content: sys_prompt,
        }
    }

    /// gives a system message that should give an intro, and prompt as a user that want to start the game
    fn build_initial_prompt() -> Vec<ChatMessage> {
        let context = "When the user want to start the game. You should say a sentence like this one, that match with your tone:

            Welcome to the Historical Adventure Game, dHisStoryGame.AI! In this journey, history meets creativity. Before we begin, please tell me: Are you a real character in the historical event or not? And which country or city would you prefer to be in?";
        let user_intro = ChatMessage {
            role: Role::User,
            content: START_GAME_PHRASE.to_string(),
        };

        vec![system_message_prompt(Some(context)), user_intro]
    }

    /// make a summary from existing history of conversation
    fn prepare_summarization_prompt(history: &Vec<SingleConversation>) -> Vec<ChatMessage> {
        let combined = combine_conversation(history);
        let prompt = format!(
            "Summarize the folllowing conversation in a concise manner, capturing the key decisions and context for an interactive historical adventure game:\n\n{}",
            combined);
        vec![
            ChatMessage {
                role: Role::System,
                content: "You are a summarizer. Your task is to provide a concise summary of a conversation, preserving key narrative decisions and context.".to_string()
            },
            ChatMessage {
                role: Role::User,
                content: prompt,
            }
        ]
    }

    /// shortcut for making a message prompt using llama3.1 model
    async fn llama_31_chat(messages: Vec<ChatMessage>) -> String {
        chat(ic_llm::Model::Llama3_1_8B, messages).await
    }

    /// prompt to do summarize the conversations
    async fn summarize(conversation: &Vec<SingleConversation>) -> String {
        llama_31_chat(prepare_summarization_prompt(conversation)).await
    }

    /// make the history of conversation as a context of system message with the given user prompt
    fn build_conversation_context(
        history: &Vec<SingleConversation>,
        new_input: &str,
    ) -> Vec<ChatMessage> {
        let clue = "The current conversation context that consist of either a summary of previous context, user prompt, assistant prompt, or the combination of them:";
        let combined_history = combine_conversation(history);
        let context = format!("{}\n\n{}", clue, combined_history);

        vec![
            system_message_prompt(Some(&context)),
            ChatMessage {
                role: Role::User,
                content: new_input.to_string(),
            },
        ]
    }

    /// process the user prompt when having a given history of conversation
    async fn continue_conversation(
        history: &mut Vec<SingleConversation>,
        user_input: &str,
    ) -> String {
        if history.len() > HISTORY_THRESHOLD {
            let summary = summarize(&history).await;
            history.clear();
            history.push(SingleConversation::new_summary(&summary));
        }
        let messages = build_conversation_context(&history, user_input);
        let response = llama_31_chat(messages).await;
        history.push(SingleConversation::new(user_input, &response));
        response
    }

    /// Responding to a specific command
    async fn response_for_command(input: &str) -> (Option<String>, bool) {
        match input {
            "/start" => (Some(llama_31_chat(build_initial_prompt()).await), true),

            "/about" => (Some("This is an interactive game where historical settings are blended with creative twists, that depends on you to decide. Build on ICP, and it is available at https://github.com/muhrifqii/dHisStoryGameAI".to_string()), false),

            _ => (None, false),
        }
    }

    /// the main user prompt handler
    pub async fn user_prompt(user: Principal, input: &str) -> String {
        let mut history = get_history(&user);
        if let (Some(res), is_starting) = response_for_command(input).await {
            if is_starting && history.is_empty() {
                update_history(
                    user.clone(),
                    vec![SingleConversation::new(START_GAME_PHRASE, &res)],
                );
            }
            return res;
        }
        let response = continue_conversation(&mut history, input).await;
        update_history(user, history);
        response
    }

    #[cfg(test)]
    mod tests {
        use ic_llm::{ChatMessage, Role};

        use crate::utils::SingleConversation;

        use super::{
            build_conversation_context, build_initial_prompt, prepare_summarization_prompt,
            system_message_prompt, SYSTEM_PROMPT,
        };

        #[test]
        fn system_prompt_message_check() {
            let msg = system_message_prompt(None);
            assert_eq!(msg.content, SYSTEM_PROMPT);
            assert!(matches!(msg.role, Role::System));
            let msg = system_message_prompt(Some("additional context here"));
            assert_ne!(msg.content, SYSTEM_PROMPT);
        }

        #[test]
        fn initial_prompt_message_check() {
            let msg = build_initial_prompt();
            assert_eq!(msg.len(), 2);
            assert!(matches!(msg.get(0).unwrap().role, Role::System));
            assert!(matches!(msg.get(1).unwrap().role, Role::User));
        }

        #[test]
        fn summarization_preparation_should_returns_correct_propmpts() {
            let conv = vec![
                SingleConversation::new("Hello", "World!"),
                SingleConversation::new("Foo", "Bar!"),
            ];
            let summarizer_prompts = prepare_summarization_prompt(&conv);
            assert_eq!(summarizer_prompts.len(), 2);
            assert_eq!(summarizer_prompts.get(1).unwrap().content, "Summarize the folllowing conversation in a concise manner, capturing the key decisions and context for an interactive historical adventure game:\n\nuser: Hello\nassistant: World!\nuser: Foo\nassistant: Bar!");
        }

        #[test]
        fn verify_conversation_context() {
            let conv = vec![
                SingleConversation::new("Hello", "World!"),
                SingleConversation::new("Foo", "Bar!"),
            ];
            let messages = build_conversation_context(&conv, "Oh hello there");
            assert_eq!(messages.len(), 2);
            assert!(matches!(messages.last().unwrap().role, Role::User));
            assert_eq!(messages.last().unwrap().content, "Oh hello there");
            assert_eq!(
                messages.get(0).unwrap().content,
                format!("{}\n\nThe current conversation context that consist of either a summary of previous context, user prompt, assistant prompt, or the combination of them:\n\nuser: Hello\nassistant: World!\nuser: Foo\nassistant: Bar!", SYSTEM_PROMPT)
            );
        }
    }
}

pub mod utils {
    use std::fmt::Display;

    #[derive(Clone, Debug)]
    pub struct SingleConversation {
        pub assistant: Option<String>,
        pub user: Option<String>,
        pub summary: Option<String>,
    }

    impl Display for SingleConversation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let summary = match &self.summary {
                Some(s) => format!("Summary of previous context: {}", s),
                None => "".to_string(),
            };
            let user = match &self.user {
                Some(u) => format!("user: {}\n", u),
                None => "".to_string(),
            };
            let assistant = match &self.assistant {
                Some(a) => format!("assistant: {}", a),
                None => "".to_string(),
            };
            write!(f, "{}{}{}", summary, user, assistant)
        }
    }

    impl SingleConversation {
        pub fn new(user: &str, assistant: &str) -> Self {
            Self {
                assistant: Some(assistant.to_string()),
                user: Some(user.to_string()),
                summary: None,
            }
        }

        pub fn new_summary(summary: &str) -> Self {
            Self {
                assistant: None,
                user: None,
                summary: Some(summary.to_string()),
            }
        }
    }

    pub fn combine_conversation(conversation: &Vec<SingleConversation>) -> String {
        conversation
            .iter()
            .map(|conv| conv.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(test)]
    mod tests {
        use super::{combine_conversation, SingleConversation};

        #[test]
        fn init_and_display_test() {
            let conv = SingleConversation::new("hello", "oh hello there");
            assert_eq!(conv.to_string(), "user: hello\nassistant: oh hello there");
            let conv = SingleConversation::new_summary("this is a summary");
            assert_eq!(
                conv.to_string(),
                "Summary of previous context: this is a summary"
            );
        }

        #[test]
        fn combine_conversation_test() {
            let conversations = vec![
                SingleConversation::new_summary("this is a summary"),
                SingleConversation::new("hello", "oh hello there"),
                SingleConversation::new("who is joe?", "joe mama"),
            ];
            let combined = combine_conversation(&conversations);
            assert_eq!(combined, "Summary of previous context: this is a summary\nuser: hello\nassistant: oh hello there\nuser: who is joe?\nassistant: joe mama");
        }
    }
}

pub mod conversations {
    use std::{cell::RefCell, collections::HashMap};

    use candid::Principal;

    use crate::utils::SingleConversation;

    thread_local! {

        /// History of message of conversation unit. Used as a workaround for a missing assistant role in ic_llm
        static CONVERSATION_HISTORIES: RefCell<HashMap<Principal, Vec<SingleConversation>>> = RefCell::new(HashMap::new());
    }

    pub fn get_history(user: &Principal) -> Vec<SingleConversation> {
        CONVERSATION_HISTORIES
            .with_borrow(|histories| histories.get(user).cloned().unwrap_or_default())
    }

    pub fn update_history(user: Principal, history: Vec<SingleConversation>) {
        CONVERSATION_HISTORIES.with_borrow_mut(|histories| {
            histories.insert(user, history);
        })
    }

    #[cfg(test)]
    mod tests {
        use candid::Principal;

        use crate::utils::SingleConversation;

        use super::{get_history, update_history, CONVERSATION_HISTORIES};

        fn clear_history() {
            CONVERSATION_HISTORIES.with_borrow_mut(|histories| histories.clear());
        }

        #[test]
        fn get_and_update_history_check() {
            let user = Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c".to_string()).unwrap();
            update_history(user.clone(), vec![]);
            update_history(
                Principal::anonymous(),
                vec![SingleConversation::new("hi", "oh hello there")],
            );
            let history = get_history(&user);
            assert!(history.is_empty());
            let history = get_history(&Principal::anonymous());
            assert_eq!(
                history.get(0).unwrap().to_string(),
                "user: hi\nassistant: oh hello there"
            );

            clear_history();
        }
    }
}

use ic_cdk::{caller, update};

#[update]
async fn prompt(prompt_str: String) -> String {
    let user = caller();
    prompts::user_prompt(user, &prompt_str).await
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
