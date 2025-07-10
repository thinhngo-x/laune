use crate::config;
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role},
    Client,
};
use tracing::info;

/// Generate a summary of an article using the OpenAI API
pub async fn generate_summary(title: &str, content: &str) -> Result<String, String> {
    // Load the OpenAI configuration from config file
    let settings = match config::Settings::new() {
        Ok(settings) => settings,
        Err(e) => return Err(format!("Failed to load config: {}", e)),
    };

    // Check if API key is valid
    if settings.openai.api_key.is_empty() || settings.openai.api_key == "your-api-key-here" {
        return Err(
            "OpenAI API key not configured. Please set a valid API key in config/default.json"
                .to_string(),
        );
    }

    info!(
        "Generating summary for article: {} using model: {}",
        title, settings.openai.model
    );

    // Initialize the OpenAI client
    let openai_config = OpenAIConfig::new().with_api_key(settings.openai.api_key);
    let client = Client::with_config(openai_config);

    // Create the prompt for the summary
    let system_message = ChatCompletionRequestMessage {
        role: Role::System,
        content: Some(
            "You are an AI assistant that summarizes articles. \
            Provide a concise and informative summary in 2-3 paragraphs. \
            Focus on the key points and main takeaways."
                .to_string(),
        ),
        name: None,
        function_call: None,
    };

    let prompt = format!("Title: {}\n\nContent: {}", title, content);
    let user_message = ChatCompletionRequestMessage {
        role: Role::User,
        content: Some(prompt),
        name: None,
        function_call: None,
    };

    // Create the chat completion request
    let request = CreateChatCompletionRequest {
        model: settings.openai.model,
        messages: vec![system_message, user_message],
        temperature: Some(0.5),
        max_tokens: Some(settings.openai.max_tokens as u16),
        ..Default::default()
    };

    // Send the request to the OpenAI API
    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| format!("OpenAI API error: {}", e))?;

    // Extract the summary from the response
    let summary = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .ok_or_else(|| "No response content from OpenAI".to_string())?;

    info!("Successfully generated summary");
    Ok(summary)
}
