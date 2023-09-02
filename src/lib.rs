
pub fn username(email: &str) -> Result<String, Box<dyn Error>> {
	let mut name_iter = email.split("@").next()?.to_lowercase().split(".");
	
	let first_name = name_iter.next()?;
	let mut last_name: String = name_iter
		.filter(|s| s.len() > 1)
		.filter(|s| !s.contains("contractor"))
		.map(|s| s.replace("-", ""))
		.map(|s| if s == "ki" {"k"} else {s.as_str()})
		.collect();
	
	Ok(first_name[0]+last_name)
}

pub fn fullname(email: &str) -> String {
	 email
		.split("@")
		.next()?
		.to_lowercase()
		.replace("contractor", "")
		.split(".")
		.map(|s| uppercase_first_letter(s))
		.collect::<&[&str]>()
		.join(" ")
}

// copied
fn uppercase_first_letter(s: &str) -> String {
	let mut c = s.chars();
	match c.next() {
		None => String::new(),
		Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
	}
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
