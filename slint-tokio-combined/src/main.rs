// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    error::Error,
    sync::{Arc, Mutex},
};

mod my_async;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    let task_running = Arc::new(Mutex::new(false));

    // Here we can toggle the running state from the UI
    ui.on_start_async({
        let ui_handle = ui.as_weak();
        move || {
            let task_running = Arc::clone(&task_running);
            let ui = ui_handle.unwrap();
            let future1 = my_async::async_function(ui, task_running);
            slint::spawn_local(async_compat::Compat::new(future1)).unwrap();
        }
    });

    ui.on_stop_async({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_is_running(false);
        }
    });

    // This one just spawns and runs until it returns
    let ui_handle = ui.as_weak();
    let future = my_async::async_function2(ui_handle);
    slint::spawn_local(async_compat::Compat::new(future)).unwrap();

    ui.run()?;

    Ok(())
}
