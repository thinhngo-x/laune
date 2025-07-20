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

/// Generate an aggregated summary from multiple article summaries
pub async fn generate_aggregated_summary(
    feed_summaries: &[crate::models::FeedSummaryInfo],
    hours_back: i64,
) -> Result<String, String> {
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
        "Generating aggregated summary for {} feeds over {} hours",
        feed_summaries.len(),
        hours_back
    );

    // Initialize the OpenAI client
    let openai_config = OpenAIConfig::new().with_api_key(settings.openai.api_key);
    let client = Client::with_config(openai_config);

    // Prepare the content for aggregation
    let mut content_parts = Vec::new();
    let total_articles: usize = feed_summaries.iter().map(|fs| fs.articles.len()).sum();

    content_parts.push(format!(
        "I have {} articles from {} feeds from the last {} hours. Here are the individual article summaries grouped by feed:",
        total_articles, feed_summaries.len(), hours_back
    ));

    for feed_summary in feed_summaries {
        content_parts.push(format!(
            "\n## Feed: {} ({} articles)",
            feed_summary.feed_title, feed_summary.article_count
        ));

        for article in &feed_summary.articles {
            if let Some(summary) = &article.summary {
                content_parts.push(format!("\n### {}\n{}", article.title, summary));
            } else {
                content_parts.push(format!("\n### {} (no summary available)", article.title));
            }
        }
    }

    let aggregated_content = content_parts.join("\n");

    // Create the prompt for aggregated summary
    let system_message = ChatCompletionRequestMessage {
        role: Role::System,
        content: Some(
            "You are an AI assistant that creates comprehensive summaries from multiple article summaries. \
            Analyze the provided article summaries and create a cohesive overview that:\
            1. Identifies key themes and trends across all articles\
            2. Highlights the most important news and developments\
            3. Groups related topics together\
            4. Provides a clear, well-structured summary in 3-5 paragraphs\
            5. Mentions which feeds the information comes from when relevant\
            Focus on providing value by synthesizing information rather than just listing articles."
                .to_string(),
        ),
        name: None,
        function_call: None,
    };

    let user_message = ChatCompletionRequestMessage {
        role: Role::User,
        content: Some(aggregated_content),
        name: None,
        function_call: None,
    };

    // Create the chat completion request with a higher token limit for aggregated content
    let request = CreateChatCompletionRequest {
        model: settings.openai.model,
        messages: vec![system_message, user_message],
        temperature: Some(0.7),
        max_tokens: Some((settings.openai.max_tokens * 2).min(4000) as u16), // Increase limit for aggregated summary
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

    info!("Successfully generated aggregated summary");
    Ok(summary)
}
