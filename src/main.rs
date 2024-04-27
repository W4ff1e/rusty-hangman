#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use std::{char, io::stdin};
mod hangmangame;
use std::default;

use eframe::egui::{self};
use hangmangame::HangmanGameState;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // Define the options for the native application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "HangmanApp",
        options,
        Box::new(|cc| Box::new(HangmanApp::new(cc))),
    )
    .expect("Failed to run native application! Panic!");
}

#[derive(Default)]
/// Represents a Hangman application.
struct HangmanApp {
    game_state: HangmanGameState,   // The current state of the Hangman game
    show_confirmation_dialog: bool, // Flag indicating whether the exit confirmation dialog should be shown
    allowed_to_close: bool,         // Flag indicating whether the application is allowed to close
    input_text: String,             // The text entered by the user as input for guessing
    submitted_text: String,         // The text submitted by the user/program as a guess
}

impl HangmanApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default();
        Self {
            game_state: HangmanGameState::new("".to_string().to_uppercase()),
            show_confirmation_dialog: false,
            allowed_to_close: false,
            input_text: String::new(),
            submitted_text: String::new(),
        }
    }
}
const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
/// Implementation of the `eframe::App` trait for the `HangmanApp` struct.
impl eframe::App for HangmanApp {
    /// Updates the application state and renders the user interface.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The `egui::Context` used for rendering the UI.
    /// * `frame` - The `eframe::Frame` used for displaying the UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hangman Game!");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                        // TODO: Display the phrase to guess with underscores for each letter.
                        ui.label("Guesses:");
                        ui.horizontal(|ui| {
                            for letter in ALPHABET.iter() {
                                if self.game_state.chars_to_guess.contains(letter)
                                    && self.game_state.guessed_letters.contains(letter)
                                {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(0, 255, 0),
                                        format!("{}", *letter),
                                    );
                                } else if self.game_state.guessed_letters.contains(letter) {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(255, 0, 0),
                                        format!("{}", *letter),
                                    );
                                } else {
                                    if ui.link(format!("{}", *letter)).clicked() {
                                        self.submitted_text = letter.to_string();
                                    }
                                }
                            }
                        });
                    });
                });
                // ui.add_space(ui.available_size_before_wrap().x * 0.50);
                ui.vertical(|ui| {
                    egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                        let max_rect = ui.max_rect();
                        // Allocate a painter
                        // Allocate a painter
                        let (response, painter) =
                            ui.allocate_painter(max_rect.size(), egui::Sense::hover());

                        // Head
                        painter.circle_filled(
                            response.rect.center() + egui::Vec2::new(0.0, -70.0),
                            20.0,
                            egui::Color32::WHITE,
                        );

                        // Body
                        painter.line_segment(
                            [
                                response.rect.center() + egui::Vec2::new(0.0, -50.0),
                                response.rect.center() + egui::Vec2::new(0.0, -10.0),
                            ],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );

                        // Arms
                        painter.line_segment(
                            [
                                response.rect.center() + egui::Vec2::new(-20.0, -30.0),
                                response.rect.center() + egui::Vec2::new(20.0, -30.0),
                            ],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );

                        // Legs
                        painter.line_segment(
                            [
                                response.rect.center() + egui::Vec2::new(-10.0, -10.0),
                                response.rect.center() + egui::Vec2::new(-10.0, 10.0),
                            ],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );
                        painter.line_segment(
                            [
                                response.rect.center() + egui::Vec2::new(10.0, -10.0),
                                response.rect.center() + egui::Vec2::new(10.0, 10.0),
                            ],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );
                        // ui.add_space(450.0);
                    });
                });
            });
            ui.add_space(ui.available_size_before_wrap().y * 0.10);

            ui.horizontal(|ui| {
                ui.label("Enter a letter:");
                ui.add_sized(
                    egui::Vec2::new(30.0, 25.0),
                    egui::TextEdit::singleline(&mut self.input_text),
                );
                if self.input_text.len() > 1 {
                    println!("Input text is too long, truncating...");
                    self.input_text.truncate(1);
                    println!("Input text after truncation: {}", self.input_text);
                }
                if ui.button("Guess").clicked() || ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Some(letter) = self.input_text.chars().next() {
                        self.submitted_text = letter.to_string().to_uppercase();
                        self.input_text.clear();
                    }
                }
            });

            if !self.submitted_text.is_empty() {
                ui.label(format!("Guessed letter: {}", self.submitted_text));
                hangmangame::HangmanGameState::guess_letter(
                    &mut self.game_state,
                    self.submitted_text
                        .to_uppercase()
                        .chars()
                        .next()
                        .expect("Invalid character guessed!"),
                );
                self.submitted_text.clear();
            }
            if self.game_state.difficulty == 0 {
                ui.horizontal(|ui| {
                    ui.label("Select difficulty:");
                    if ui.button("Very Easy").clicked() {
                        self.game_state.difficulty = 10;
                    }
                    if ui.button("Easy").clicked() {
                        self.game_state.difficulty = 8;
                    }
                    if ui.button("Normal").clicked() {
                        self.game_state.difficulty = 6;
                    }
                    if ui.button("Hard").clicked() {
                        self.game_state.difficulty = 4;
                    }
                });
            }
            // ! DEBUG CODE AHEAD!!!!
            ui.vertical(|ui| {
                ui.label("Debug:");
                ui.label(format!(
                    "Phrase to guess: {}",
                    self.game_state.phrase_to_guess
                ));
                ui.label(format!(
                    "Chars to guess: {:?}",
                    self.game_state.chars_to_guess
                ));
                ui.label(format!(
                    "Obfuscated phrase: {}",
                    self.game_state.obfuscated_phrase
                ));
                ui.label(format!("Submitted Text: {}", self.submitted_text));
                ui.label(format!("Input Text: {}", self.input_text));
                ui.label(format!(
                    "Guessed letters: {:?}",
                    self.game_state.guessed_letters
                ));
                ui.label(format!(
                    "Incorrect guess count: {}",
                    self.game_state.incorrect_guess_count
                ));
                ui.label(format!(
                    "Guesses Left: {}",
                    self.game_state.difficulty - self.game_state.incorrect_guess_count
                ));
                ui.label(format!("Difficulty: {}", self.game_state.difficulty));
                ui.label(format!("Game over: {}", self.game_state.game_over));
                ui.label(format!("Win: {}", self.game_state.win));
            });

            // ! END DEBUG CODE!!!!
            if self.game_state.incorrect_guess_count >= self.game_state.difficulty
                && self.game_state.difficulty != 0
                && self.game_state.phrase_to_guess != ""
            {
                self.game_state.game_over = true;
                self.game_state.win = false;
            }
            if self
                .game_state
                .chars_to_guess
                .iter()
                .all(|c| self.game_state.guessed_letters.contains(c))
                && self.game_state.phrase_to_guess != ""
            {
                self.game_state.game_over = true;
                self.game_state.win = true;
            }
        });

        if self.game_state.game_over {
            egui::Window::new("Game Over!")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if self.game_state.win {
                            ui.label("Congratulations! You won!");
                            ui.label(format!(
                                "The phrase was: {}",
                                self.game_state.phrase_to_guess
                            ));
                        } else {
                            ui.label("Game Over! You lost!");
                            ui.label(format!(
                                "The phrase was: {}",
                                self.game_state.phrase_to_guess
                            ));
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Restart?").clicked() {
                            self.game_state = default::Default::default();
                        }
                        if ui.button("Quit?").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }

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
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}
