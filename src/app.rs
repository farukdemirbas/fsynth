use egui::{
    Button, CentralPanel, Color32, CornerRadius, Frame, Margin, MenuBar, Panel, Slider, Stroke, Ui, ViewportCommand, widgets::global_theme_preference_buttons
};

use crate::audio::engine::AudioEngine;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FSynthApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    audio_engine: AudioEngine,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for FSynthApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            audio_engine: AudioEngine::new(),
        }
    }
}

impl FSynthApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for FSynthApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        Panel::top("top_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                }

                global_theme_preference_buttons(ui);
            });
        });

        Panel::left("left_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                Frame::default()
                    .inner_margin(Margin::same(12))
                    .show(ui, |_ui| {});
            });

        Panel::right("right_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                Frame::default()
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.label("right panel");
                    });
                });

        CentralPanel::default().show_inside(ui, |ui| {

            // These will become shared params. This is dummy vals.
            let mut name = "Instrument 1".to_string();
            let mut pan = 0.0;
            let mut volume = 0.1;
            let mut frequency = 440.0;
            
            Frame::default()
                .inner_margin(Margin::same(12))
                .outer_margin(Margin::same(12))
                .stroke(Stroke::new(0.4, Color32::WHITE))
                .corner_radius(CornerRadius::same(2))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(&name);
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Panning");
                            ui.add(egui::Slider::new(&mut pan, -1.0..=1.0));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Volume");
                            ui.add(egui::Slider::new(&mut volume, -0.0..=1.0));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Frequency");
                            ui.add(egui::Slider::new(&mut frequency, 80.0..=4000.0).suffix("Hz"));
                        });
                    })
                });
        });
    }
}
