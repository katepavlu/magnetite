/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize,PartialEq)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MagnetiteGUI {
    // Example stuff:
    // this how you opt-out of serialization of a member
    res: f64,
    last_plot_min: [f64; 2],
    last_plot_max: [f64; 2],
}

impl Default for MagnetiteGUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            res: 10.0,
            last_plot_min: [0.0, 0.0],
            last_plot_max: [10.0, 10.0],
        }
    }
}

impl MagnetiteGUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

use egui::plot::{Arrows, PlotPoints, Plot};
use egui::*;

impl eframe::App for MagnetiteGUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { res, last_plot_max, last_plot_min } = self;


        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    _frame.close();
                };
                if ui.button("Load file").clicked() {
                    _frame.close();
                };
                if ui.button("Save to file").clicked() {
                    _frame.close();
                }; 
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            
            ui.heading("Settings");

            ui.add(egui::Slider::new(res, 10.0..=100.0)
                   .logarithmic(true)
                   .text("Plot Resolution"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Plot");
            let pnt_cnt = res.powi(2);

            let plot = Plot::new("items_demo").data_aspect(1.0);


            let eval_points = PlotPoints::from_parametric_callback(
                    |t| (t%*res+last_plot_min[0], (t - t%*res) / *res + last_plot_min[1] ),
                    0.0..pnt_cnt,
                    pnt_cnt as usize,
                );

            let arrow_tips = PlotPoints::from_parametric_callback(
                    |t| (t%*res + 0.5+last_plot_min[0], (t - t%*res) / *res + 0.5 +last_plot_min[1]),
                    0.0..pnt_cnt,
                    pnt_cnt as usize,
                );

            let arrows = {
                Arrows::new(eval_points, arrow_tips)
            };
            
            let InnerResponse {
                response,
                inner: (bounds,_),
            } = plot.show(ui, |plot_ui| {(
                plot_ui.plot_bounds(),
                plot_ui.arrows(arrows),
            )});

            *last_plot_min = bounds.min();
            *last_plot_max = bounds.max();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
