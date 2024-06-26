#![windows_subsystem = "windows"]
use csv;
use eframe::egui;
use email_sender as es;
use rfd;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command, str};

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Option::from(egui::Vec2::new(700 as f32, 500 as f32));
    eframe::run_native(
        "Email Sender",
        native_options,
        Box::new(|cc| Box::new(EmailSenderApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct EmailSenderApp {
    hide_password_from_cc: bool,
    template: Option<PathBuf>,
    user_list: Option<PathBuf>,
    attachment: Option<PathBuf>,
    users: Option<Vec<(User, String, String)>>,
    email: EmailTemplate,
    error: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Email {
    to: String,
    cc: String,
    attachment: String,
    subject: String,
    body: String,
}

#[derive(Default, Serialize, Deserialize)]
struct EmailTemplate {
    cc: String,
    subject: String,
    body: String,
}

#[derive(Default, Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
    var1: Option<String>,
    var2: Option<String>,
    var3: Option<String>,
}

#[macro_export]
macro_rules! rfd_filter {
    ($x:expr) => {
        rfd::FileDialog::new().add_filter($x, &[$x])
    };
}

impl EmailSenderApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn send_emails(&mut self) -> BoxResult<()> {
        // autosave template
        self.template_save()?;
        // Exit if user list is not loaded.
        self.user_list
            .as_ref()
            .ok_or(es::AppError::UserListEmptyError)?;

        let emails = self.create_emails()?;

        // run email backend
        let email_json_str = serde_json::to_string(&emails)?;
        let _output = Command::new("powershell")
            .arg("-File")
            .arg("email.ps1")
            .arg(email_json_str)
            .output()
            .expect("failed to send email");

        Ok(())
    }

    fn create_emails(&self) -> BoxResult<Vec<Email>> {
        let mut emails: Vec<Email> = Vec::new();
        for (user, username, fullname) in self.users.as_ref().unwrap() {
            let attachment = self.attachment_as_string();
            let body = self
                .email
                .body
                .replace("{username}", username)
                .replace("{fullname}", fullname)
                .replace("{password}", &user.password)
                .replace("{var1}", &user.var1.as_deref().unwrap_or_default())
                .replace("{var2}", &user.var2.as_deref().unwrap_or_default())
                .replace("{var3}", &user.var3.as_deref().unwrap_or_default());

            let subject = self
                .email
                .subject
                .replace("{username}", username)
                .replace("{fullname}", fullname)
                .replace("{password}", &user.password)
                .replace("{var1}", &user.var1.as_deref().unwrap_or_default())
                .replace("{var2}", &user.var2.as_deref().unwrap_or_default())
                .replace("{var3}", &user.var3.as_deref().unwrap_or_default());

            if self.hide_password_from_cc && !self.email.cc.is_empty() {
                let body_for_cc = self
                    .email
                    .body
                    .replace("{username}", username)
                    .replace("{fullname}", fullname)
                    .replace("{password}", "[hidden]")
                    .replace("{var1}", &user.var1.as_deref().unwrap_or_default())
                    .replace("{var2}", &user.var2.as_deref().unwrap_or_default())
                    .replace("{var3}", &user.var3.as_deref().unwrap_or_default());

                emails.push(Email {
                    to: user.email.clone(),
                    cc: String::new(),
                    attachment: attachment.clone(),
                    subject: subject.clone(),
                    body,
                });
                emails.push(Email {
                    to: self.email.cc.clone(),
                    cc: String::new(),
                    attachment,
                    subject,
                    body: body_for_cc,
                });
            } else {
                emails.push(Email {
                    to: user.email.clone(),
                    cc: self.email.cc.clone(),
                    attachment,
                    subject,
                    body,
                });
            }
        }
        Ok(emails)
    }

    fn attachment_as_string(&self) -> String {
        self.attachment
            .clone()
            .unwrap_or(PathBuf::new())
            .to_string_lossy()
            .to_string()
    }

    // Fail if either path/file does not exist or the yaml file does not match email format
    fn template_open(&mut self) -> BoxResult<()> {
        if let Some(path) = rfd_filter!("yaml").pick_file() {
            self.template = Some(path);
            let yf = fs::read_to_string(self.template.as_deref().unwrap())?;
            self.email = serde_yaml::from_str(yf.as_str())?;
        }
        Ok(())
    }
    /// Saves EmailTemplate to file.
    ///
    /// # Panics
    /// Panics if email template cannot be turned into yaml.
    ///
    /// # Errors
    /// This function will return an error if it could not save to file.
    fn template_save(&mut self) -> BoxResult<()> {
        match self.template.as_deref() {
            Some(tmpl) => {
                let yaml_text = serde_yaml::to_string(&self.email).unwrap();
                fs::write(tmpl, yaml_text)?;
            }
            None => {
                self.template_save_as()?;
            }
        }
        Ok(())
    }
    fn template_save_as(&mut self) -> BoxResult<()> {
        if let Some(file) = rfd_filter!("yaml").save_file() {
            self.template = Some(file);
            self.template_save()?;
        }
        Ok(())
    }
    fn template_export(&self) -> BoxResult<()> {
        if let Some(file) = rfd_filter!("yaml").save_file() {
            let yaml_text = serde_yaml::to_string(&self.email).unwrap();
            fs::write(file.as_path(), yaml_text)?;
        };
        Ok(())
    }

    fn user_list_open(&mut self) -> BoxResult<()> {
        self.user_list = rfd_filter!("csv").pick_file();
        let user_list = self
            .user_list
            .as_deref()
            .ok_or(es::AppError::UserListEmptyError)?;

        let mut rdr = csv::Reader::from_path(user_list)?;
        let mut user_rows = vec![];
        for res in rdr.deserialize() {
            let user: User = res?;
            let username = es::username(&user.email)?;
            let fullname = es::fullname(&user.email)?;
            user_rows.push((user, username, fullname));
        }
        self.users = Some(user_rows);
        Ok(())
    }
    //TODO
    fn attachment_open(&mut self) -> BoxResult<()> {
        self.attachment = rfd::FileDialog::new().pick_file();
        Ok(())
    }

    fn show_menu(&mut self, ui: &mut egui::Ui) {
        use egui::{menu, Button};

        menu::bar(ui, |ui| {
            ui.menu_button("Template", |ui| {
                if ui.button("🗁 Open").clicked() {
                    self.error = es::error_to_string(self.template_open());
                }
                if ui.button("🗐 Save").clicked() {
                    self.error = es::error_to_string(self.template_save());
                }
                if ui.button("🗐 Save as").clicked() {
                    self.error = es::error_to_string(self.template_save_as());
                }
                if ui.button("🗐 Export").clicked() {
                    self.error = es::error_to_string(self.template_export());
                }
            });
            ui.menu_button("User List", |ui| {
                if ui.button("🗁 Open").clicked() {
                    self.error = es::error_to_string(self.user_list_open());
                }
            });
            ui.menu_button("Attachment", |ui| {
                if ui.button("🗁 Select").clicked() {
                    self.error = es::error_to_string(self.attachment_open());
                }
            });
        });
    }

    fn show_user_table(&self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};
        TableBuilder::new(ui)
            .column(Column::initial(200.0).resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::initial(100.0).resizable(true))
            .column(Column::remainder().resizable(true))
            .header(14.0, |mut header| {
                header.col(|ui| {
                    ui.heading("email");
                });
                header.col(|ui| {
                    ui.heading("password");
                });
                header.col(|ui| {
                    ui.heading("username");
                });
                header.col(|ui| {
                    ui.heading("fullname");
                });
            })
            .body(|mut body| {
                for (user, username, fullname) in self.users.as_ref().unwrap() {
                    body.row(10.0, |mut row| {
                        row.col(|ui| {
                            ui.label(&user.email);
                        });
                        row.col(|ui| {
                            ui.label(&user.password);
                        });
                        row.col(|ui| {
                            ui.label(username);
                        });
                        row.col(|ui| {
                            ui.label(fullname);
                        });
                    })
                }
            });
    }
    fn show_gui_table(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};
        TableBuilder::new(ui)
            .column(Column::initial(50.0))
            .column(Column::remainder().resizable(true))
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("subject:");
                    });
                    row.col(|ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut self.email.subject),
                        );
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("cc:");
                    });
                    row.col(|ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut self.email.cc),
                        );
                    });
                });
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("body:");
                    });
                    row.col(|ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.email.body),
                        );
                    });
                });
            });
    }
}

impl eframe::App for EmailSenderApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Email Sender");
            self.show_menu(ui);
            ui.checkbox(&mut self.hide_password_from_cc, "Hide password from cc?");
            ui.horizontal(|ui| {
                ui.label("attachment:\t");
                ui.label(self.attachment_as_string());
            });
            self.show_gui_table(ui);
            if ui.button("📤 send emails").clicked() {
                self.error = es::error_to_string(self.send_emails());
            }
            if self.users.is_some() {
                self.show_user_table(ui);
            }
        });
        if let Some(err_display) = self.error.as_deref() {
            egui::Window::new("error message").show(ctx, |ui| ui.label(err_display));
        }
    }
}
