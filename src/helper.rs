pub fn is_ui_dev() -> bool {
    if let Some(value) = option_env!("UI_DEV") {
        return value.to_lowercase() == "true";
    }
    
    false
}