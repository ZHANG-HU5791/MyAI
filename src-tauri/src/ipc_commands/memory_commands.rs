use crate::core_engine::memory::MasterSpec;
use crate::event_bus::{AppEvent, EventBus};
use crate::storage::Database;

#[tauri::command]
pub async fn get_master_spec(
    db: tauri::State<'_, Database>,
    session_id: Option<String>,
) -> Result<MasterSpec, String> {
    let spec = crate::storage::master_spec_repo::load_latest(&db, session_id.as_deref())
        .await
        .map_err(|e| e.to_string())?;
    Ok(spec.unwrap_or_else(MasterSpec::new))
}

#[tauri::command]
pub async fn update_master_spec(
    db: tauri::State<'_, Database>,
    event_bus: tauri::State<'_, EventBus>,
    session_id: Option<String>,
    spec: MasterSpec,
) -> Result<(), String> {
    crate::storage::master_spec_repo::save(&db, session_id.as_deref(), &spec)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(sid) = session_id {
        let _ = event_bus.publish(AppEvent::MasterSpecUpdated { session_id: sid });
    }
    let _ = event_bus.publish(AppEvent::MemoryStateInvalidated {
        reason: "Master spec updated via IPC".to_string(),
    });

    Ok(())
}

#[tauri::command]
pub async fn update_active_goal(
    db: tauri::State<'_, Database>,
    event_bus: tauri::State<'_, EventBus>,
    session_id: Option<String>,
    goal: String,
) -> Result<(), String> {
    let mut spec = crate::storage::master_spec_repo::load_latest(&db, session_id.as_deref())
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or_else(MasterSpec::new);

    spec.global_constraints.target_goals.push(goal);

    crate::storage::master_spec_repo::save(&db, session_id.as_deref(), &spec)
        .await
        .map_err(|e| e.to_string())?;

    let _ = event_bus.publish(AppEvent::MemoryStateInvalidated {
        reason: "Active goal updated".to_string(),
    });

    Ok(())
}

#[tauri::command]
pub async fn add_constraint(
    db: tauri::State<'_, Database>,
    event_bus: tauri::State<'_, EventBus>,
    session_id: Option<String>,
    constraint: String,
) -> Result<(), String> {
    let mut spec = crate::storage::master_spec_repo::load_latest(&db, session_id.as_deref())
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or_else(MasterSpec::new);

    spec.global_constraints
        .blacklisted_approaches
        .push(constraint);

    crate::storage::master_spec_repo::save(&db, session_id.as_deref(), &spec)
        .await
        .map_err(|e| e.to_string())?;

    let _ = event_bus.publish(AppEvent::MemoryStateInvalidated {
        reason: "Constraint added".to_string(),
    });

    Ok(())
}
