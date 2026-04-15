#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app_lib;

fn main() {
  app_lib::run();
}