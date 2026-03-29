use crate::ai_providers::AiRequest;

pub fn build_json_messages(request: &AiRequest) -> Vec<serde_json::Value> {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    if let Some(ref system_prompt) = request.system_prompt {
        messages.push(serde_json::json!({
            "role": "system",
            "content": system_prompt
        }));
    }

    let screenshot_b64 = request.screenshot_base64.clone();

    for msg in &request.messages {
        let role: &str = match msg.role {
            crate::models::MessageRole::User => "user",
            crate::models::MessageRole::Assistant => "assistant",
            crate::models::MessageRole::System => "system",
        };

        if let Some(ref ss) = screenshot_b64 {
            messages.push(serde_json::json!({
                "role": role,
                "content": msg.content.clone(),
                "images": [ss]
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
            serde_json::Value::String(format!("{}...[{} chars]", &s[..max_len], s.len()))
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
