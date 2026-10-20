use localsend_rpc::{CommandResult, DaemonClient, ServiceCommand, ServiceSnapshot};
use tauri::{Emitter, State};

struct DesktopState {
    runtime: tokio::runtime::Runtime,
    client: DaemonClient,
}

#[tauri::command]
fn get_snapshot(state: State<'_, DesktopState>) -> Option<ServiceSnapshot> {
    state.client.snapshot()
}

#[tauri::command]
async fn command(
    state: State<'_, DesktopState>,
    command: ServiceCommand,
) -> Result<CommandResult, String> {
    state.client.command(command).await
}

#[tauri::command]
fn runtime_handle(state: State<'_, DesktopState>) -> Result<(), String> {
    let _ = state.runtime.handle();
    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio runtime");
    let client = runtime
        .block_on(DaemonClient::connect_interactive())
        .expect("connect to localsendd");
    let event_client = client.clone();
    let state = DesktopState { runtime, client };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            command,
            runtime_handle
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            let mut events = event_client.subscribe();
            tauri::async_runtime::spawn(async move {
                while let Ok(event) = events.recv().await {
                    let _ = handle.emit("service-event", event);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running LocalSend PC prototype");
}
