pub mod error;
pub mod event_bus;
pub mod storage;
pub mod core_engine;
pub mod mcp_client;
pub mod ipc_commands;

use event_bus::EventBus;
use mcp_client::McpRegistry;
use storage::Database;
use tauri::Manager;

/// Entry point for the Tauri application.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Resolve app data directory — no hardcoded paths
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            // Create event bus and start forwarding to frontend
            let event_bus = EventBus::new();
            event_bus.setup_event_forwarding(app_handle.clone());

            // Resolve MCP config path inside app data dir
            let mcp_config_path = app_data_dir.join("mcp_config.json");
            if !mcp_config_path.exists() {
                // Copy default config if none exists
                let default_config = include_str!("../mcp_config.json");
                std::fs::write(&mcp_config_path, default_config)
                    .expect("Failed to write default MCP config");
            }

            // Create MCP registry
            let mcp_registry = McpRegistry::new(mcp_config_path, event_bus.clone());

            // Spawn database initialization in a background task
            let db_data_dir = app_data_dir.clone();
            let event_bus_clone = event_bus.clone();
            let app_handle_clone = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                match Database::new(&db_data_dir).await {
                    Ok(db) => {
                        log::info!("Database initialized at {:?}", db_data_dir);

                        // Load MCP config
                        if let Err(e) = mcp_registry.load_config().await {
                            log::error!("Failed to load MCP config: {e}");
                        }

                        // Register all state
                        app_handle_clone.manage(db);
                        app_handle_clone.manage(event_bus_clone);
                        app_handle_clone.manage(mcp_registry);

                        log::info!("Omni-Creator Hub initialized successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to initialize database: {e}");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Chat commands
            ipc_commands::chat_commands::create_session,
            ipc_commands::chat_commands::list_sessions,
            ipc_commands::chat_commands::delete_session,
            ipc_commands::chat_commands::get_messages,
            ipc_commands::chat_commands::send_message,
            // Canvas commands
            ipc_commands::canvas_commands::update_canvas_content,
            ipc_commands::canvas_commands::apply_canvas_modification,
            // MCP commands
            ipc_commands::mcp_commands::list_mcp_servers,
            ipc_commands::mcp_commands::start_mcp_server,
            ipc_commands::mcp_commands::stop_mcp_server,
            ipc_commands::mcp_commands::reload_mcp_config,
            ipc_commands::mcp_commands::list_mcp_tools,
            ipc_commands::mcp_commands::call_mcp_tool,
            // Memory commands
            ipc_commands::memory_commands::get_master_spec,
            ipc_commands::memory_commands::update_master_spec,
            ipc_commands::memory_commands::update_active_goal,
            ipc_commands::memory_commands::add_constraint,
            // Settings commands
            ipc_commands::settings_commands::get_api_keys,
            ipc_commands::settings_commands::set_api_key,
            ipc_commands::settings_commands::delete_api_key,
            ipc_commands::settings_commands::get_app_settings,
            ipc_commands::settings_commands::update_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Omni-Creator Hub");
}
