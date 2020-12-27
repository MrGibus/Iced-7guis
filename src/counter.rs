use iced::{button, Align, Button, Row, Element, Sandbox, Settings, Text, HorizontalAlignment, Length};

pub fn main() -> iced::Result{
    // Set the window properties
    let mut settings = Settings::default();
    settings.window.size = SIZE;
    settings.window.resizable = false;

    // run the application
    Counter::run(settings)
}

// the window size is an unsigned 32 bit integer, the padding is unsigned 16 bit integer
const SIZE: (u32, u32) = (250, 80);
const PAD: u16 = 25;

#[derive(Default)]
pub struct Counter {
    // counter value
    value: i32,
    btn: button::State,
}

// define the possible interactions of the application
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ButtonPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    // linking the enum to the logic
    fn update(&mut self, message:Message) {
        match message {
            Message::ButtonPressed  => {
                self.value +=1;
            }
        }
    }

    // the view defining the layout, linking the widgets to the interaction 'message' enum
    fn view(&mut self) -> Element<Message> {
        let btn_txt = Text::new("Count")
            .horizontal_alignment(HorizontalAlignment::Center);

        Row::new()
            .padding(PAD)
            .spacing(PAD*2)
            .align_items(Align::Center)
            .push(
                Text::new(&self.value.to_string())
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::FillPortion(2)),
            )
            .push(
                Button::new(&mut self.btn, btn_txt)
                    .width(Length::FillPortion(2))
                    .on_press(Message::ButtonPressed),
            )
            .into()
    }

}
