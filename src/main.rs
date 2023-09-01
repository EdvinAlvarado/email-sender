use std::{path::PathBuf, fs};
use eframe::egui;
use serde::{Serialize, Deserialize};
use rfd;


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
	email: Email
}

#[derive(Default, Serialize, Deserialize)]
struct Email {
	subject: String,
	cc: String,
	body: String,	
}

impl EmailSenderApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}
	fn send_emails(& self) {
		todo!()
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
}

impl eframe::App for EmailSenderApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Email Sender");
			show_menu(ui, self);
			ui.checkbox(&mut self.hide_password_from_cc, "Hide password from cc?");
			ui.text_edit_singleline(&mut self.email.subject);
			ui.text_edit_singleline(&mut self.email.cc);
			ui.text_edit_multiline(&mut self.email.body);
			if ui.button("send emails 📤").clicked() {self.send_emails();}
		});
	}
}

fn show_menu(ui: &mut egui::Ui, app: &mut EmailSenderApp) {
		use egui::{menu, Button};

		menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
						if ui.button("🗁 Open").clicked() {app.file_open()}
						if ui.button("🗐 Save").clicked() {app.file_save()}
						if ui.button("🗐 Save as").clicked() {app.file_save_as()}
				}) 
		});
}

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
