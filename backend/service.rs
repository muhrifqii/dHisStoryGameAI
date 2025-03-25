pub mod prompts {
    use crate::ollama::{chat, ChatMessage, Role};

    const SYSTEM_PROMPT: &'static str = "
    You are an interactive storyteller for a historical event or story, a game with an immersive, branching narrative. Your task is to continue the story based on previous context and the user's decision.

    Your response must be guided byt the following instructions:
    - Narative Generation:
        - Once the user provides their description, use it as the foundation for the story.
        - Generate a richly detailed narrative that is immersive and uses vivid language to portray the historical setting.
        - Incorporate factual historical elements of the chosen location while introducing imaginative twists based on the user's input.
        - Structure narrative branches as bullet lists. For example:
            - * Explore the ancient marketplace.
            - * Visit the royal palace.
        - Balance historical accuracy with creative storytelling. Let historical facts form the backbone of the narrative while allowing creative twists based on user input.
    - If the user's input is not relevant to the game (for example, if it strays from the historical adventure narrative), respond with a message explaining that you cannot process such input because it is outside the scope of the game.
    - Always conclude your responses with a set of bullet-pointed 2-3 choices.
    - Branching and Decisions:
        - Present the available narrative choices as a bullet list. Do not rely solely on numbered options.
        - Allow the user the freedom to type in an additional choice if it makes sense, ensuring creativity while discouraging irrelevant or nonsensical input.
    - Tone and Style:
        - Use a friendly, engaging, and slightly dramatic tone that captures the grandeur of historical events while remaining accessible.
        - Provide clear, vivid descriptions of settings, characters, and events.
    ";

    const HISTORY_THRESHOLD: usize = 7;

    fn system_message_prompt() -> ChatMessage {
        ChatMessage {
            role: Role::system,
            content: SYSTEM_PROMPT.to_string(),
        }
    }

    fn history_too_long(history: &Vec<ChatMessage>) -> bool {
        history.len() > HISTORY_THRESHOLD
    }

    pub fn build_initial_prompt() -> Vec<ChatMessage> {
        let intro_instruction = ChatMessage {
            role: Role::assistant,
            content: r#"
            You should say this sentence

            Welcome to the Historical Adventure Game, dHisStoryGame.AI! In this journey, history meets creativity. Before we begin, please tell me: Are you a real person or a character in a historical event? And which country or city would you prefer to be in?
            "#.to_string(),
        };

        vec![system_message_prompt(), intro_instruction]
    }

    fn prepare_summarization_prompt(history_chat_messages: &Vec<ChatMessage>) -> Vec<ChatMessage> {
        let combined = history_chat_messages
            .iter()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect::<Vec<_>>()
            .join("\n");
        let prompt = format!(
            "Summarize the folllowing conversation in a concise manner, capturing the key decisions and context for an interactive historical adventure game:\n\n{}",
            combined);
        vec![
            ChatMessage {
                role: Role::system,
                content: "You are a summarizer. Your task is to provide a concise summary of a conversation, preserving key narrative decisions and context.".to_string()
            },
            ChatMessage {
                role: Role::user,
                content: prompt,
            }
        ]
    }

    async fn summarize(messages: &Vec<ChatMessage>) -> String {
        chat(prepare_summarization_prompt(messages)).await
    }

    fn build_conversation_context(history: &Vec<ChatMessage>, new_input: &str) -> Vec<ChatMessage> {
        let mut messages = Vec::new();
        messages.push(system_message_prompt());
        messages.extend(history.clone());
        messages.push(ChatMessage {
            role: Role::user,
            content: new_input.to_string(),
        });
        messages
    }

    pub async fn continue_conversation(
        mut history: Vec<ChatMessage>,
        user_input: &str,
    ) -> Option<String> {
        if history_too_long(&history) {
            let summary = summarize(&history).await;
            history.clear();
            history.push(ChatMessage {
                role: Role::system,
                content: format!("Summary of previous context: {}", summary),
            });
        }
        let messages = build_conversation_context(&history, user_input);
        let response = chat(messages).await;
        history.push(ChatMessage {
            role: Role::user,
            content: user_input.to_string(),
        });
        history.push(ChatMessage {
            role: Role::user,
            content: response.clone(),
        });
        Some(response)
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            ollama::{ChatMessage, Role},
            service::prompts::history_too_long,
        };

        use super::{
            build_conversation_context, build_initial_prompt, prepare_summarization_prompt,
            system_message_prompt, SYSTEM_PROMPT,
        };

        #[test]
        fn system_prompt_message_check() {
            let msg = system_message_prompt();
            assert_eq!(msg.content, SYSTEM_PROMPT);
            assert!(matches!(msg.role, Role::system));
        }

        #[test]
        fn validate_summarization_criteria() {
            let msgs = vec![
                ChatMessage {
                    role: Role::system,
                    content: "Hello".to_string(),
                },
                ChatMessage {
                    role: Role::assistant,
                    content: "World!".to_string(),
                },
                ChatMessage {
                    role: Role::user,
                    content: "wohoo!".to_string(),
                },
            ];
            assert!(!history_too_long(&msgs));
        }

        #[test]
        fn initial_prompt_message_check() {
            let msg = build_initial_prompt();
            assert_eq!(msg.len(), 2);
            assert_eq!(msg.get(0).unwrap().content, SYSTEM_PROMPT);
        }

        #[test]
        fn summarization_preparation_should_returns_correct_propmpts() {
            let msgs = vec![
                ChatMessage {
                    role: Role::system,
                    content: "Hello".to_string(),
                },
                ChatMessage {
                    role: Role::assistant,
                    content: "World!".to_string(),
                },
                ChatMessage {
                    role: Role::user,
                    content: "wohoo!".to_string(),
                },
            ];
            let summarizer_prompts = prepare_summarization_prompt(&msgs);
            assert_eq!(summarizer_prompts.len(), 2);
            assert_eq!(summarizer_prompts.get(1).unwrap().content, "Summarize the folllowing conversation in a concise manner, capturing the key decisions and context for an interactive historical adventure game:\n\nsystem: Hello\nassistant: World!\nuser: wohoo!");
        }

        #[test]
        fn verify_conversation_context() {
            let history = vec![ChatMessage {
                role: Role::assistant,
                content: "Hello World!".to_string(),
            }];
            let messages = build_conversation_context(&history, "Hello");
            assert_eq!(messages.len(), 3);
            assert_eq!(
                messages.last().unwrap().role.to_string(),
                Role::user.to_string()
            );
            assert_eq!(messages.last().unwrap().content, "Hello");
            assert_eq!(messages.get(1).unwrap().content, "Hello World!");
        }
    }
}

pub mod conversations {
    use std::{cell::RefCell, collections::HashMap};

    use candid::Principal;

    use crate::ollama::ChatMessage;

    thread_local! {
        static CONVERSATION_HISTORIES: RefCell<HashMap<Principal, Vec<ChatMessage>>> = RefCell::new(HashMap::new());
    }

    fn get_history(user: Principal) -> Vec<ChatMessage> {
        CONVERSATION_HISTORIES
            .with_borrow(|histories| histories.get(&user).cloned().unwrap_or_default())
    }

    fn update_history(user: Principal, history: Vec<ChatMessage>) {
        CONVERSATION_HISTORIES.with_borrow_mut(|histories| {
            histories.insert(user, history);
        })
    }

    #[cfg(test)]
    mod tests {
        use candid::Principal;

        use crate::ollama::{ChatMessage, Role};

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
                vec![ChatMessage {
                    role: Role::user,
                    content: "yes".to_string(),
                }],
            );
            let history = get_history(user.clone());
            assert!(history.is_empty());
            let history = get_history(Principal::anonymous());
            assert_eq!(history.get(0).unwrap().content, "yes");

            clear_history();
        }
    }
}
