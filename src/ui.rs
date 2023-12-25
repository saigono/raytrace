use eframe::egui;

#[derive(Default)]
pub struct RaytraceUI {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for RaytraceUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Scene file: ");
                if ui.button("Open…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(".json", &["json"])
                        .set_directory("./")
                        .pick_file()
                    {
                        self.picked_path = Some(path.display().to_string());
                    }
                }

                // Show dropped files (if any):
                if !self.dropped_files.is_empty() {
                    ui.group(|ui| {
                        ui.label("Dropped files:");

                        for file in &self.dropped_files {
                            let mut info = if let Some(path) = &file.path {
                                path.display().to_string()
                            } else if !file.name.is_empty() {
                                file.name.clone()
                            } else {
                                "???".to_owned()
                            };

                            let mut additional_info = vec![];
                            if !file.mime.is_empty() {
                                additional_info.push(format!("type: {}", file.mime));
                            }
                            if let Some(bytes) = &file.bytes {
                                additional_info.push(format!("{} bytes", bytes.len()));
                            }
                            if !additional_info.is_empty() {
                                info += &format!(" ({})", additional_info.join(", "));
                            }

                            ui.label(info);
                        }
                    });
                }
            });

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
        });
    }
}
