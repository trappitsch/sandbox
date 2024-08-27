use rfd::MessageDialog;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

#[derive(Clone, Debug)]
pub struct Settings {
    value: i32,
}

impl Settings {
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let settings_dialog = SettingsDialog::new()?;

    let my_settings = Rc::new(RefCell::new(Settings { value: 42 }));

    // Main page
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_settings_window({
        let sd_handle = settings_dialog.as_weak();
        let my_settings = my_settings.clone();
        move || {
            let sd = sd_handle.unwrap();
            sd.show().unwrap();
            sd.set_settings_value(my_settings.borrow().get_value());
        }
    });

    ui.on_settings_print({
        let my_settings = my_settings.clone();
        move || {
            println!("{:?}", my_settings.borrow());
        }
    });

    // Settings dialog window
    settings_dialog.on_ok_clicked({
        let sd_handle = settings_dialog.as_weak();
        let my_settings = my_settings.clone();
        move || {
            let sd = sd_handle.unwrap();
            sd.hide().unwrap();
            let mut mss = my_settings.borrow_mut();
            mss.set_value(sd.get_settings_value());
            println!("Settings dialog OK clicked");
        }
    });

    settings_dialog.on_cancel_clicked({
        let sd_handle = settings_dialog.as_weak();
        move || {
            sd_handle.unwrap().hide().unwrap();
            println!("Settings dialog Cancel clicked");
        }
    });

    // Dialog page
    ui.on_message_dialog({
        let ui = ui.as_weak().unwrap();
        move || {
            ui.hide();
            let msg = MessageDialog::new()
                .set_title("Hello, world!")
                .set_description("This is a message dialog.")
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
            println!("{:?}", msg == rfd::MessageDialogResult::Ok);
            ui.show();
        }
    });

    ui.on_question_dialog(|| {
        let msg = MessageDialog::new()
            .set_title("Hello, questionable world!")
            .set_description("This is a question dialog.")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();
        println!("{:?}", msg);
    });

    ui.run()
}
