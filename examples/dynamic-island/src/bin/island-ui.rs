#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = dynamic_island::run_dynamic_island_ui_preview_standalone() {
        eprintln!("failed to run island UI preview: {error}");
        std::process::exit(1);
    }
}
