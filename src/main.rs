use std::{path::PathBuf, fs};
use eframe::egui;
use serde::{Serialize, Deserialize};
use rfd;
use csv;
use outlook_exe::MessageBuilder;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Email Sender",
		native_options,
		Box::new(|cc| Box::new(EmailSenderApp::new(cc)))
	).unwrap();
}

#[derive(Default, Serialize, Deserialize)]
struct EmailSenderApp {
	hide_password_from_cc: bool,
	template: PathBuf,
	user_list: PathBuf,
	email: Email
}

#[derive(Default, Serialize, Deserialize)]
struct Email {
	subject: String,
	cc: String,
	body: String,
}


#[derive(Default, Serialize, Deserialize)]
struct User {
	email: String,
	password: String,
}

impl EmailSenderApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}

	fn send_emails(&mut self) {
		if self.template.as_os_str().is_empty() {
			self.file_save_as();
		} else {
			self.file_save();
		}

 		if let Some(path) = rfd::FileDialog::new().add_filter("csv", &["csv"]).pick_file() {
			self.user_list= path;
		}

		let mut rdr = csv::Reader::from_path(self.user_list.as_path()).unwrap();
		for res in rdr.deserialize() {
			let user: User = res.expect("Not a user record");

		}
	}
	fn send_email(& self, user: User) {
		let username = email_sender::username(user.email.as_str());
		let fullname = email_sender::fullname(user.email.as_str());

		let body = self.email.body
			.replace("{username}", username.as_str())
			.replace("{fullname}", fullname.as_str())
			.replace("{password}", user.password.as_str());

		let subject = self.email.subject
			.replace("{username}", username.as_str())
			.replace("{fullname}", fullname.as_str())
			.replace("{password}", user.password.as_str());
		
		if self.hide_password_from_cc && !self.email.cc.is_empty() {
			MessageBuilder::new()
				.with_recipient(user.email)
				.with_subject(subject.as_str())
				.with_body(body)
				.spawn()
				.unwrap();
			
			let body_for_cc = self.email.body
				.replace("{username}", username.as_str())
				.replace("{password}", "[hidden]");
			MessageBuilder::new()
				.with_recipient(self.email.cc.as_str())
				.with_subject(subject.as_str())
				.with_body(body_for_cc)
				.spawn()
				.unwrap();
		}
		else {
			MessageBuilder::new()
				.with_recipient(user.email)
				.with_recipient_cc(self.email.cc.as_str())
				.with_subject(subject.as_str())
				.with_body(body)
				.spawn()
				.unwrap();
		}
	}
	
	fn file_open(&mut self) {
		if let Some(path) = rfd::FileDialog::new().add_filter("yaml", &["yaml"]).pick_file() {
			self.template = path;
		}
		let yf = fs::read_to_string(self.template.as_path()).unwrap();
		self.email = serde_yaml::from_str(yf.as_str()).expect("not a yaml file");
	}
	fn file_save(& self) {
		let yaml_text = serde_yaml::to_string(&self.email).unwrap();
		fs::write(self.template.as_path(), yaml_text).unwrap();
	}
	fn file_save_as(&mut self) {
		if let Some(path) = rfd::FileDialog::new().add_filter("yaml", &["yaml"]).save_file() {
			self.template = path;
		}
		self.file_save();
	}

	fn show_menu(&mut self, ui: &mut egui::Ui) {
			use egui::{menu, Button};

			menu::bar(ui, |ui| {
					ui.menu_button("File", |ui| {
							if ui.button("🗁 Open").clicked() {self.file_open()}
							if ui.button("🗐 Save").clicked() {self.file_save()}
							if ui.button("🗐 Save as").clicked() {self.file_save_as()}
					}) 
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
				ui.label("subject:");
				ui.text_edit_singleline(&mut self.email.subject);
			});
			ui.horizontal(|ui| {
				ui.label("cc:");
				ui.text_edit_singleline(&mut self.email.cc);
			});
			ui.horizontal(|ui| {
				ui.label("body:");
				ui.text_edit_multiline(&mut self.email.body);
			});
			if ui.button("📤 send emails").clicked() {self.send_emails();}
		});
	}
}

