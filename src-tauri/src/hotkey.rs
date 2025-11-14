use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::state::AppState;

pub fn bind_hotkey(
    app_handle: &AppHandle,
    state: &AppState,
    hotkey: &str,
    window_label: &str,
) -> Result<(), String> {
    if hotkey.trim().is_empty() {
        return Err("快捷键不能为空".into());
    }

    let mut current_hotkey = state
        .registered_hotkey
        .lock()
        .map_err(|_| "无法获取快捷键状态".to_string())?;

    if let Some(previous) = current_hotkey.as_deref() {
        if let Err(err) = app_handle.global_shortcut().unregister(previous) {
            log::warn!("failed to unregister previous hotkey {previous}: {err}");
        }
    }

    let hotkey_string = hotkey.trim().to_string();
    let shortcut_literal = hotkey_string.clone();
    let window_label_string = window_label.to_string();
    app_handle
        .global_shortcut()
        .on_shortcut(shortcut_literal.as_str(), {
            let window_label = window_label_string;
            move |app_handle, _, event| {
                if event.state == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window(&window_label) {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .map_err(|err| err.to_string())?;

    *current_hotkey = Some(hotkey_string);
    Ok(())
}
