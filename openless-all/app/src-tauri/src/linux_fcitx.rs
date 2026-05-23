//! Linux fcitx5 DBus client.
//!
//! The local-lite build keeps only the dictation path: committing text,
//! syncing the main dictation hotkey, and listening for dictation edge events.

use std::time::Duration;

use dbus::blocking::BlockingSender;

const DEST: &str = "org.fcitx.Fcitx5";
const PATH: &str = "/openless";
const IFACE: &str = "org.fcitx.Fcitx.OpenLess1";
const TIMEOUT: Duration = Duration::from_secs(3);

pub fn commit_text(text: &str) -> Result<(), String> {
    let conn = dbus::blocking::Connection::new_session()
        .map_err(|e| format!("dbus session: {e}"))?;
    let msg = dbus::Message::new_method_call(DEST, PATH, IFACE, "CommitText")
        .map_err(|e| format!("build msg: {e}"))?
        .append1(text);
    conn.send_with_reply_and_block(msg, TIMEOUT)
        .map_err(|e| format!("CommitText: {e}"))?;
    Ok(())
}

pub fn set_hotkey(keys: &[&str]) -> Result<(), String> {
    let conn = dbus::blocking::Connection::new_session()
        .map_err(|e| format!("dbus session: {e}"))?;
    let list: Vec<String> = keys.iter().map(|s| s.to_string()).collect();
    let msg = dbus::Message::new_method_call(DEST, PATH, IFACE, "SetHotkey")
        .map_err(|e| format!("build msg: {e}"))?
        .append1(list);
    conn.send_with_reply_and_block(msg, TIMEOUT)
        .map_err(|e| format!("SetHotkey: {e}"))?;
    Ok(())
}

pub fn set_hotkey_raw(sym: u32, states: u32) -> Result<(), String> {
    let conn = dbus::blocking::Connection::new_session()
        .map_err(|e| format!("dbus session: {e}"))?;
    let msg = dbus::Message::new_method_call(DEST, PATH, IFACE, "SetHotkeyRaw")
        .map_err(|e| format!("build msg: {e}"))?
        .append2(sym, states);
    conn.send_with_reply_and_block(msg, TIMEOUT)
        .map_err(|e| format!("SetHotkeyRaw: {e}"))?;
    Ok(())
}

const KEYSYM_CONTROL_R: u32 = 0xffe4;
const KEYSYM_CONTROL_L: u32 = 0xffe3;
const KEYSYM_ALT_R: u32 = 0xffea;
const KEYSYM_ALT_L: u32 = 0xffe9;
const KEYSYM_SUPER_R: u32 = 0xffec;

fn trigger_to_keysym(trigger: crate::types::HotkeyTrigger) -> u32 {
    match trigger {
        crate::types::HotkeyTrigger::RightControl => KEYSYM_CONTROL_R,
        crate::types::HotkeyTrigger::LeftControl => KEYSYM_CONTROL_L,
        crate::types::HotkeyTrigger::RightOption | crate::types::HotkeyTrigger::RightAlt => {
            KEYSYM_ALT_R
        }
        crate::types::HotkeyTrigger::LeftOption => KEYSYM_ALT_L,
        crate::types::HotkeyTrigger::RightCommand => KEYSYM_SUPER_R,
        crate::types::HotkeyTrigger::Fn => KEYSYM_CONTROL_R,
        crate::types::HotkeyTrigger::Custom => unreachable!(),
    }
}

fn trigger_name(trigger: crate::types::HotkeyTrigger) -> &'static str {
    match trigger {
        crate::types::HotkeyTrigger::RightControl => "Control_R",
        crate::types::HotkeyTrigger::LeftControl => "Control_L",
        crate::types::HotkeyTrigger::RightOption | crate::types::HotkeyTrigger::RightAlt => {
            "Alt_R"
        }
        crate::types::HotkeyTrigger::LeftOption => "Alt_L",
        crate::types::HotkeyTrigger::RightCommand => "Super_R",
        crate::types::HotkeyTrigger::Fn => "Control_R",
        crate::types::HotkeyTrigger::Custom => unreachable!(),
    }
}

pub fn sync_binding_to_plugin(binding: &crate::types::HotkeyBinding) {
    if binding.trigger == crate::types::HotkeyTrigger::Custom {
        return;
    }
    let sym = trigger_to_keysym(binding.trigger);
    let name = trigger_name(binding.trigger);
    match set_hotkey_raw(sym, 0) {
        Ok(()) => log::info!("[fcitx] Synced hotkey {name} (sym={sym}) to plugin"),
        Err(e) => log::warn!("[fcitx] Failed to sync hotkey to plugin: {e}"),
    }
}

pub fn binding_to_fcitx_key_string(binding: &crate::types::ShortcutBinding) -> String {
    let mut parts: Vec<String> = Vec::new();
    for m in &binding.modifiers {
        let lower = m.to_lowercase();
        let normalized = match lower.as_str() {
            "ctrl" | "control" => "Control",
            "alt" | "option" | "opt" => "Alt",
            "shift" => "Shift",
            "super" | "meta" | "cmd" | "win" | "command" => "Super",
            other => other,
        };
        let normalized = normalized.to_string();
        if !parts.contains(&normalized) {
            parts.push(normalized);
        }
    }

    let primary = binding.primary.trim();
    let primary = if let Some(stripped) = primary.strip_prefix("Key") {
        stripped.to_lowercase()
    } else {
        primary.to_lowercase()
    };
    if primary.is_empty() {
        return String::new();
    }
    if parts.is_empty() {
        primary
    } else {
        format!("{}+{}", parts.join("+"), primary)
    }
}

pub fn set_custom_dictation_trigger(key_string: &str) -> Result<(), String> {
    let conn = dbus::blocking::Connection::new_session()
        .map_err(|e| format!("dbus session: {e}"))?;
    let msg = dbus::Message::new_method_call(DEST, PATH, IFACE, "SetCustomDictationTrigger")
        .map_err(|e| format!("build msg: {e}"))?
        .append1(key_string);
    conn.send_with_reply_and_block(msg, TIMEOUT)
        .map_err(|e| format!("SetCustomDictationTrigger: {e}"))?;
    Ok(())
}

pub fn available() -> bool {
    let conn = match dbus::blocking::Connection::new_session() {
        Ok(c) => c,
        Err(_) => return false,
    };
    let msg = match dbus::Message::new_method_call(DEST, PATH, "org.freedesktop.DBus.Peer", "Ping")
    {
        Ok(m) => m,
        Err(_) => return false,
    };
    conn.send_with_reply_and_block(msg, TIMEOUT).is_ok()
}

#[cfg(target_os = "linux")]
pub fn start_dictation_signal_listener(
    tx: std::sync::mpsc::Sender<crate::hotkey::HotkeyEvent>,
) {
    std::thread::Builder::new()
        .name("openless-fcitx-signal".into())
        .spawn(move || {
            let conn = match dbus::blocking::SyncConnection::new_session() {
                Ok(c) => c,
                Err(e) => {
                    log::warn!("[fcitx-hotkey] DBus session failed: {e}");
                    return;
                }
            };

            let rule = match dbus::message::MatchRule::parse(
                "type='signal',\
                 interface='org.fcitx.Fcitx.OpenLess1'",
            ) {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("[fcitx-hotkey] Invalid match rule: {e}");
                    return;
                }
            };

            let _match = match conn.add_match(rule, move |args: (u32, u32, bool), _conn, msg| {
                let (sym, states, is_press) = args;
                let member = msg.member();
                let member_str: String =
                    member.as_ref().map(|m| m.to_string()).unwrap_or_default();
                log::debug!(
                    "[fcitx-hotkey] Signal {}: sym={}, states={}, isPress={}",
                    member_str,
                    sym,
                    states,
                    is_press,
                );
                if member.as_deref() == Some("DictationKeyEvent") {
                    let event = if is_press {
                        crate::hotkey::HotkeyEvent::Pressed
                    } else {
                        crate::hotkey::HotkeyEvent::Released
                    };
                    let _ = tx.send(event);
                }
                true
            }) {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("[fcitx-hotkey] Failed to add match: {e}");
                    return;
                }
            };

            log::info!("[fcitx-hotkey] Listening for OpenLess1 dictation signals");
            loop {
                if let Err(e) = conn.process(Duration::from_millis(500)) {
                    log::warn!("[fcitx-hotkey] DBus process error: {e}");
                    break;
                }
            }
        })
        .ok();
}
