#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

  use tauriapp::AppBuilder;
  
  pub fn main() {
    AppBuilder::new().run();
  }
  
  
  