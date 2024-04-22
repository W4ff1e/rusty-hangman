#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{char, io::stdin};

use eframe::egui;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "HangmanApp",
        options,
        Box::new(|cc| Box::new(HangmanApp::new(cc))),
    );
    println!("Welcome to Hangman!");

    loop {
        println!("Please enter your phrase: ");

        // Declare an empty string for the buffer to fill.
        let mut phrase_to_guess: String = String::new();

        //  Read terminal input
        stdin()
            .read_line(&mut phrase_to_guess)
            .expect("Failed to read terminal input!");

        // Make phrase to guess immutable for this iteration.
        let phrase_to_guess = phrase_to_guess;

        loop {
            println!("Please enter a letter to guess:");
            let mut letter_to_guess: String = String::new();

            stdin()
                .read_line(&mut letter_to_guess)
                .expect("Failed to read line!");

            let letter_to_guess: char = match letter_to_guess.trim().parse() {
                Ok(char) => char,
                Err(_) => continue,
            };

            for character in 0..phrase_to_guess.chars().count() {
                if letter_to_guess == phrase_to_guess.chars().nth(character).unwrap() {
                    println!("Match!")
                } else {
                    println!("No Match!");
                }
            }
        }
    }
    // TODO Needed for Ratatui
    //Ok(())
}

#[derive(Default)]
struct HangmanApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

impl HangmanApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for HangmanApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hangman Game!");
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            egui::Window::new("Are you sure you want to exit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}
