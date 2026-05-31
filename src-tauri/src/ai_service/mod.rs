pub mod client;
pub mod config;
pub mod conversation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod client_tests;

pub use client::AiClient;
pub use config::AiConfig;
pub use conversation::{
    Conversation, ConversationManager, ConversationMessage, ConversationRole, ConversationStats,
    PromptTemplate,
};
