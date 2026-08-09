use crate::{application::ai_providers::AiRequest, domain::messages::MessageRole};

pub fn build_json_messages(request: &AiRequest) -> Vec<serde_json::Value> {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    if let Some(ref system_prompt) = request.system_prompt {
        messages.push(serde_json::json!({
            "role": "system",
            "content": system_prompt
        }));
    }

    let screenshot_b64 = request.screenshot_base64.clone();
    let is_multimodal = screenshot_b64.is_some();

    for (i, msg) in request.messages.iter().enumerate() {
        let role: &str = match msg.role {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::System => "system",
        };

        let is_last_user_message = msg.role == MessageRole::User
            && request
                .messages
                .iter()
                .skip(i + 1)
                .all(|m| m.role != MessageRole::User);

        if is_multimodal && is_last_user_message {
            let content = serde_json::json!([
                {"type": "text", "text": msg.content.clone()},
                {"type": "image_url", "image_url": {"url": format!("data:image/png;base64,{}", screenshot_b64.as_ref().unwrap())}}
            ]);
            messages.push(serde_json::json!({
                "role": role,
                "content": content
            }));
        } else {
            messages.push(serde_json::json!({
                "role": role,
                "content": msg.content.clone()
            }));
        }
    }

    messages
}

pub fn truncate_json_body(value: &serde_json::Value, max_len: usize) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) if s.len() > max_len => {
            let truncated: String = s.chars().take(max_len).collect();
            serde_json::Value::String(format!("{}...[{} chars]", truncated, s.len()))
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| truncate_json_body(v, max_len)).collect())
        }
        serde_json::Value::Object(obj) => serde_json::Value::Object(
            obj.iter()
                .map(|(k, v)| (k.clone(), truncate_json_body(v, max_len)))
                .collect(),
        ),
        _ => value.clone(),
    }
}
