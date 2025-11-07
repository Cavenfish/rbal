use dirs::data_dir;
use eframe::egui;
use std::path::PathBuf;

use crate::utils::get_rows;

pub struct MyApp {
    db_file: PathBuf,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            db_file: data_dir().unwrap().join("rbal/main.db"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let data = get_rows();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("data_grid").striped(true).show(ui, |ui| {
                    ui.heading("ID");
                    ui.heading("Vendor");
                    ui.heading("Date");
                    ui.heading("Coin");
                    ui.heading("Amount");
                    ui.end_row();

                    for row in data.iter() {
                        ui.label(row.id.to_string());
                        ui.label(&row.vendor);
                        ui.label(&row.date);
                        ui.label(&row.coin);
                        ui.label(&row.amount.to_string());
                        ui.end_row();
                    }
                });
            });
        });
    }
}
