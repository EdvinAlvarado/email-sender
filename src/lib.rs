use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Email input error")]
    EmailInputError,
    #[error("cancelled file open")]
    CancelledFileError,
    #[error("No user list loaded.")]
    UserListEmptyError,
    #[error("Email cannot be trasnformed to username and/or fullname.")]
    UserLoadError,
}

pub type AppResult<T> = Result<T, AppError>;

pub fn username(email: &str) -> AppResult<String> {
    let email_iter = email
        .split("@")
        .next()
        .ok_or(AppError::EmailInputError)?
        .to_lowercase();
    let mut name_iter = email_iter.split(".");

    let first_name = name_iter.next().ok_or(AppError::UserLoadError)?;
    let first_char = first_name.chars().nth(0).ok_or(AppError::UserLoadError)?;
    let last_name: String = name_iter
        .filter(|s| s.len() > 1)
        .filter(|s| !s.contains("contractor"))
        .map(|s| s.replace("-", ""))
        .map(|s| if s == "ki" { "k".to_string() } else { s })
        .collect();
    Ok(first_char.to_string() + last_name.as_str())
}

pub fn fullname(email: &str) -> AppResult<String> {
    Ok(email
        .split("@")
        .next()
        .ok_or(AppError::EmailInputError)?
        .to_lowercase()
        .replace("contractor", "")
        .split(".")
        .map(|s| uppercase_first_letter(s))
        .collect::<Vec<String>>()
        .join(" "))
}

// copied
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[warn(dead_code)]
fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

pub fn error_to_string(res: Result<(), Box<dyn std::error::Error>>) -> Option<String> {
    match res {
        Ok(()) => None,
        Err(err) => Some(err.to_string()),
    }
}
