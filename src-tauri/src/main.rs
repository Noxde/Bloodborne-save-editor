#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Returning the error makes a failed start print it and exit with a non-zero code
fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_lib::run()
}
