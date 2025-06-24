use std::sync::{Arc, Mutex};

use slint::Weak;

use crate::AppWindow;

pub async fn async_function(ui: AppWindow, is_running: Arc<Mutex<bool>>) {
    {
        let mut a = is_running.lock().unwrap();
        if *a {
            println!("Async function already running, returning...");
            return;
        } else {
            *a = true;
        }
    }
    ui.set_is_running(!ui.get_is_running());
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        if !ui.get_is_running() {
            println!("Breaking endless loop in async function...");
            {
                let mut a = is_running.lock().unwrap();
                *a = false;
                println!("status a: {:?}", *a);
            }
            break;
        }
        ui.invoke_request_increase_value();
        // println!("Plus async function...");
    }
}

pub async fn async_function2(uiw: Weak<AppWindow>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(900)).await;
        let ui = uiw.unwrap();
        ui.set_counter(ui.get_counter() - 1);
        // println!("Minus async function...");
    }
}
