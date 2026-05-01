use egui::{
    Button, CentralPanel, Frame, MenuBar, Panel, Ui,
    ViewportCommand, widgets::global_theme_preference_buttons, Margin
};

use crate::audio::engine::AudioEngine;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FSynthApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    audio: AudioEngine,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for FSynthApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            audio: AudioEngine::new(),
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

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        Panel::top("top_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
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
            Frame::default()
                .inner_margin(Margin::same(12))
                .show(ui, |ui| {
                    let btn_play = ui.add_sized([62.0, 62.0], Button::new("Play"));
                    if btn_play.clicked() {
                        println!("btn_play clicked.");
                    }
                });
        });
    }
}
