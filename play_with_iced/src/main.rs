use iced::{
    event::{self, Event},
    keyboard,
    widget::{button, column, text, Column},
    window,
    Size,
    Subscription,
};

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(Event),
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    count: i64,
}

impl Counter {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::KeyPressed)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Character(character_pressed),
                    ..
                }) = event
                {
                    let cpr = character_pressed.as_ref();
                    if cpr == "k" {
                        self.count += 1;
                    } else if cpr == "j" {
                        self.count -= 1;
                    };
                }
            }
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.count),
            button("-").on_press(Message::Decrement),
        ]
    }
}

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size::new(200., 200.),
        resizable: false,
        ..window::Settings::default()
    };

    iced::application("My Counter", Counter::update, Counter::view)
        .subscription(Counter::subscription)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .window(window_settings)
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::default();
        counter.update(Message::Increment);
        assert_eq!(counter.count, 1);
        counter.update(Message::Increment);
        assert_eq!(counter.count, 2);
        counter.update(Message::Decrement);
        assert_eq!(counter.count, 1);
    }
}
