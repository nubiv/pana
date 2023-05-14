use llm_chain::{executor, parameters, prompt};

#[tauri::command]
pub async fn test() -> Result<String, String> {
    let result = "test";
    Ok(result.into())
}

#[tauri::command]
pub fn converse() {}
