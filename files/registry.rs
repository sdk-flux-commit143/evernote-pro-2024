//! Windows registry read helpers for Evernote Pro 2024.

pub const APP_KEY: &str = "Software\\Evernote\\Evernote Pro 2024";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Evernote Pro 2024";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Evernote Pro 2024") },
        RegValue { name: "Version",     value: String::from("v3.7.1") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Evernote Pro 2024") },
        RegValue { name: "Publisher",       value: String::from("Evernote") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Evernote Pro 2024\\uninstall.exe\"") },
    ]
}
