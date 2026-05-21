use crate::event_bus::{AppEvent, EventBus};
use crate::storage::message_repo;
use crate::storage::Database;

#[tauri::command]
pub async fn create_session(
    db: tauri::State<'_, Database>,
    title: String,
) -> Result<String, String> {
    let session = crate::storage::session_repo::create_session(&db, &title)
        .await
        .map_err(|e| e.to_string())?;
    Ok(session.id)
}

#[tauri::command]
pub async fn list_sessions(
    db: tauri::State<'_, Database>,
) -> Result<Vec<crate::storage::session_repo::SessionRow>, String> {
    crate::storage::session_repo::list_sessions(&db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_session(
    db: tauri::State<'_, Database>,
    session_id: String,
) -> Result<(), String> {
    crate::storage::session_repo::delete_session(&db, &session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages(
    db: tauri::State<'_, Database>,
    session_id: String,
) -> Result<Vec<message_repo::MessageRow>, String> {
    message_repo::get_messages(&db, &session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_message(
    db: tauri::State<'_, Database>,
    event_bus: tauri::State<'_, EventBus>,
    session_id: String,
    content: String,
) -> Result<(), String> {
    // Phase 4.4: Full orchestration (query transform -> cache check -> context assembly -> LLM -> streaming -> refiner)
    // For now, store the user message and emit a placeholder response.

    let user_msg = message_repo::new_message(&session_id, "user", &content, None);
    message_repo::insert_message(&db, &user_msg)
        .await
        .map_err(|e| e.to_string())?;

    // Placeholder: emit completion event
    let _ = event_bus.publish(AppEvent::ChatComplete {
        session_id,
        message_id: user_msg.id,
    });

    Ok(())
}
