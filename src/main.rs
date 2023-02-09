#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, TextEdit};
use yabf_rs::*;

const CODE: &str = ",.,+.";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800., 600.)),
        ..Default::default()
    };
    let app = App::from(Program::from(CODE));
    eframe::run_native("yabf", options, Box::new(|_cc| Box::new(app)))
}

struct App {
    bf: BfInstance,
    input_buf: String,
    out_buf: String,
    last_frame_status: ProgramStatus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bf: Default::default(),
            input_buf: Default::default(),
            out_buf: Default::default(),
            last_frame_status: ProgramStatus::Run,
        }
    }
}

impl From<Program> for App {
    fn from(p: Program) -> Self {
        Self {
            bf: BfInstance::from(p),
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
                font_id.size *= 1.65;
            }

            ui.heading(format!("Current program: {CODE}"));
            let mut need_input = false;

            let status = self.bf.step(
                || {
                    need_input = true;
                    self.input_buf.pop()
                },
                |out| {
                    while let Some(c) = out.pop() {
                        self.out_buf.push(c);
                    }
                    Ok(())
                },
            );
            if let (&ProgramStatus::Run, &ProgramStatus::Exit) = (&self.last_frame_status, &status)
            {
                self.bf
                    .io_buf
                    .flush(|out| {
                        while let Some(c) = out.pop() {
                            self.out_buf.push(c);
                        }
                        Ok(())
                    })
                    .unwrap()
            }
            self.last_frame_status = status;

            let text_edit = TextEdit::singleline(&mut self.input_buf).hint_text(match need_input {
                true => "need input",
                false => "don't need input",
            });
            let _ = text_edit.show(ui);

            ui.heading(format!("Program Output: {}", self.out_buf));
        });
    }
}
