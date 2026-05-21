use crate::event_bus::{AppEvent, EventBus};
use crate::storage::Database;
use crate::core_engine::memory::MasterSpec;
use super::master_spec::ActiveWorkspace;

/// After the main chat stream completes, spawn a background task that uses a
/// cheap model to update the Master Spec asynchronously. This keeps the main
/// chat stream blazing fast.
pub fn spawn_refiner(
    db: Database,
    event_bus: EventBus,
    session_id: String,
    user_content: String,
    assistant_content: String,
) {
    tokio::spawn(async move {
        // Load current master spec
        let mut spec = match crate::storage::master_spec_repo::load_latest(&db, Some(&session_id))
            .await
        {
            Ok(Some(s)) => s,
            Ok(None) => MasterSpec::new(),
            Err(e) => {
                log::error!("RefinerWorker: failed to load master spec: {e}");
                return;
            }
        };

        // Add the latest turn to the workspace
        spec.active_workspace
            .add_turn("user", &summarize(&user_content));
        spec.active_workspace
            .add_turn("assistant", &summarize(&assistant_content));

        // Save updated spec
        if let Err(e) =
            crate::storage::master_spec_repo::save(&db, Some(&session_id), &spec).await
        {
            log::error!("RefinerWorker: failed to save master spec: {e}");
            return;
        }

        // Notify frontend
        let _ = event_bus.publish(AppEvent::MasterSpecUpdated {
            session_id: session_id.clone(),
        });
        let _ = event_bus.publish(AppEvent::MemoryStateInvalidated {
            reason: format!("RefinerWorker updated spec for session {session_id}"),
        });

        log::info!("RefinerWorker: master spec updated for session {session_id}");
    });
}

/// Summarize a message to its first 200 chars for workspace storage.
fn summarize(text: &str) -> String {
    if text.len() <= 200 {
        text.to_string()
    } else {
        format!("{}...", &text[..200])
    }
}
