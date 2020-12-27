use iced::{button, Align, Button, Row, Element, Sandbox, Settings, Text, HorizontalAlignment, Length, TextInput, Space};
use iced::{text_input};

// the window size is an unsigned 32 bit integer, the padding is unsigned 16 bit integer
const SIZE: (u32, u32) = (300, 80);
const PAD: u16 = 25;

pub fn main() -> iced::Result{
    // Set the window properties
    let mut settings = Settings::default();
    settings.window.size = SIZE;
    settings.window.resizable = false;

    Temperature::run(settings)
}

#[derive(Default)]
struct Temperature{
    c_state: text_input::State,
    f_state: text_input::State,
    c_value: String,
    f_value: String,
}


#[derive(Debug, Clone)]
enum Message{
    CUpdated(String),
    FUpdated(String),
}

impl Sandbox for Temperature {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Temperature Converter")
    }

    fn update(&mut self, message:Message) {
        match message {
            Message::CUpdated(s) => {
                self.f_value = c_to_f(s.clone());
                self.c_value = s;
            },
            Message::FUpdated(s) => {
                self.c_value = f_to_c(s.clone());
                self.f_value = s;
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let c_in = TextInput::new(
            &mut self.c_state,
            "",
            &self.c_value,
                Message::CUpdated
        );

        let f_in = TextInput::new(
            &mut self.f_state,
            "",
            &self.f_value,
            Message::FUpdated
        );

        Row::new()
            .padding(PAD)
            .align_items(Align::Center)
            .push(c_in)
            .push(
                Text::new(" C".to_string())
                    .horizontal_alignment(HorizontalAlignment::Left),
            )
            .push(Space::with_width(Length::Fill))
            .push(f_in)
            .push(
                Text::new(" F".to_string())
                    .horizontal_alignment(HorizontalAlignment::Left),
            )
            .into()
    }
}

fn c_to_f(s: String) -> String {
    return if let Ok(c) = s.parse::<f64>() {
        let f = c * ( 9. /  5.) + 32.;
        format!("{:.1}", f)
    } else {
        "err".to_string()
    }
}

fn f_to_c(s: String) -> String {
    return if let Ok(f) = s.parse::<f64>() {
        let c = (f - 32.) * (5. / 9.);
        format!("{:.1}", c)
    } else {
        "err".to_string()
    }
}