use crate::event_bus::{AppEvent, EventBus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasContent {
    pub content_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasModification {
    pub line_range: Option<(usize, usize)>,
    pub instruction: String,
}

#[tauri::command]
pub async fn update_canvas_content(
    event_bus: tauri::State<'_, EventBus>,
    session_id: String,
    content_type: String,
    data: serde_json::Value,
) -> Result<(), String> {
    let _ = event_bus.publish(AppEvent::CanvasUpdated {
        session_id,
        content_type,
    });
    Ok(())
}

#[tauri::command]
pub async fn apply_canvas_modification(
    event_bus: tauri::State<'_, EventBus>,
    session_id: String,
    modification: CanvasModification,
) -> Result<(), String> {
    // Phase 4.5: Send modification to LLM for refinement
    let _ = event_bus.publish(AppEvent::CanvasUpdated {
        session_id,
        content_type: "diff".to_string(),
    });
    Ok(())
}
