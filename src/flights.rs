// # An application to demonstrate data-validation

use iced::{Align, Button, Element, Sandbox, Settings, Text, HorizontalAlignment, Length, TextInput, Space, PickList, Column};
use iced::{text_input, pick_list, button};
use chrono::{Local, NaiveDate};

// ## Constants
const SIZE: (u32, u32) = (250, 250);
const PAD: u16 = 25;

// ## Entry Point
pub fn main() -> iced::Result{
    // Set the window properties
    let mut settings = Settings::default();
    settings.window.size = SIZE;
    settings.window.resizable = false;

    Flights::run(settings)
}

// ## Primary data model
#[derive(Default)]
struct Flights{
    book: button::State,
    valid: bool,
    dropdown: pick_list::State<FlightType>,
    type_selected: FlightType,
    input_outbound: TextInputData,
    input_inbound: TextInputData,
}

#[derive(Default, Clone)]
struct TextInputData {
    state: text_input::State,
    value: String,
    style: style::TextInput,
}

// ## An enum to define potential events (Messages)
#[derive(Debug, Clone)]
enum Message{
    TypeSelected(FlightType),
    OutboundUpdated(String),
    InboundUpdated(String),
    BookRequest,
}

// ## Dropdown List definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlightType {
    OneWay,
    Return,
}

impl FlightType {
    const ALL: [FlightType; 2] = [
        FlightType::OneWay,
        FlightType::Return,
    ];
}

impl Default for FlightType {
    fn default() -> FlightType {
        FlightType::OneWay
    }
}

impl std::fmt::Display for FlightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlightType::OneWay => "One-way Flight",
                FlightType::Return => "Return Flight",
            }
        )
    }
}

// ## This trait controls the application
impl Sandbox for Flights {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    // Sets the window title
    fn title(&self) -> String {
        String::from("Flight Booker")
    }

    // updates the model based on a given message
    fn update(&mut self, message:Message) {
        match message {
            Message::TypeSelected(t) => {
                self.type_selected = t;
                if self.type_selected == FlightType::Return {
                    self.input_inbound.style = style::TextInput::Enabled;
                    self.valid = true;
                } else {
                    self.input_inbound.style = style::TextInput::Disabled;
                }
            },
            Message::OutboundUpdated(s) => {
                self.input_outbound.value = s;
            },
            Message::InboundUpdated(s) => {
                if self.type_selected == FlightType::Return {
                    self.input_inbound.value = s;
                }
            },
            Message::BookRequest => self.book_flight(),
        }
        self.update_validity();
    }

    // Defines the layout of the application (the view)
    fn view(&mut self) -> Element<Message> {
        let dropdown = PickList::new(
            &mut self.dropdown,
            &FlightType::ALL[..],
            Some(self.type_selected),
            Message::TypeSelected,
        )
            .width(Length::Fill);

        let tbox_outbound = TextInput::new(
            &mut self.input_outbound.state,
            &today(),
            &self.input_outbound.value,
            Message::OutboundUpdated
        )
            .padding(5)
            .style(self.input_outbound.style);

        let tbox_inbound = TextInput::new(
            &mut self.input_inbound.state,
            &today(),
            &self.input_inbound.value,
            Message::InboundUpdated
        )
            .padding(5)
            .style(self.input_inbound.style);

        let btn_book = Button::new(
            &mut self.book,
            Text::new("Book")
                .horizontal_alignment(HorizontalAlignment::Center)
                .width(Length::Fill)
        )
            .width(Length::Fill);

        Column::new()
            .padding(PAD)
            .spacing(PAD/2)
            .align_items(Align::Center)
            .push(dropdown)
            .push(tbox_outbound)
            .push(tbox_inbound)
            .push(Space::with_height(Length::Fill))
            .push(if self.valid {
                btn_book.on_press(Message::BookRequest)
            } else {
                btn_book // when no on_press method is provided so the button is disabled
            })
            .into()
    }
}

impl Flights {
    fn update_validity(&mut self) {
        // 2 inputs need to be checked for validity, initilise as neither being valid
        let mut valid_outbox = false;
        let mut valid_inbox = false;

        let date_outbound = NaiveDate::parse_from_str(
            &self.input_outbound.value,
            "%d-%m-%Y");

        let date_inbound = NaiveDate::parse_from_str(
            &self.input_inbound.value,
            "%d-%m-%Y");

        // Check outbox validity
        if date_outbound.is_ok() {
            valid_outbox = true;
            self.input_outbound.style = style::TextInput::Enabled
        } else {
            self.input_outbound.style = style::TextInput::Invalid
        }

        // check inbox validity
        if self.type_selected == FlightType::Return {
            if let Ok(indate) = date_inbound {
                // the date will parse but may not be valid as it may be before the outbound date
                if let Ok(outdate) = date_outbound {
                    let duration = indate.signed_duration_since(outdate).num_days();
                    // check at least one day has passed
                    if duration > 0 {
                        valid_inbox = true;
                        self.input_inbound.style = style::TextInput::Enabled
                    } else {
                        self.input_inbound.style = style::TextInput::Invalid
                    }
                }
            } else {
                self.input_inbound.style = style::TextInput::Invalid
            }
        } else {
            // if it's a one way flight the inflight is automatically valid
            valid_inbox = true;
            self.input_inbound.style = style::TextInput::Disabled
        }

        self.valid = valid_outbox && valid_inbox;
    }

    fn book_flight(&self) {
        println!("\nA {} has been booked", self.type_selected);
        println!("\nDeparting on: {}", self.input_outbound.value);
        if self.type_selected==FlightType::Return {
            println!("\nReturning on: {}", self.input_inbound.value);
        };
    }
}

fn today() -> String {
    Local::today().format("%d-%m-%Y").to_string()
}

// used to set the style of the text inputs
mod style {
    use iced::{text_input, Background, Color};

    // recycleable colours to use between widgets
    const BACKGROUND_CLR: Background = Background::Color(Color::WHITE);
    const ACCENT_CLR: Color = Color::from_rgb(0.7, 0.7, 0.7);
    const TEXT_CLR: Color = Color::from_rgb(0.2, 0.2 ,0.2);
    const TEXT_CLR_DISABLED: Color = Color::from_rgb(0.8, 0.8 ,0.8);
    const TEXT_CLR_INVALID: Color = Color::from_rgb(0.8, 0.2 ,0.2);
    const HIGHLIGHT_CLR: Color = Color::from_rgb(0.8, 0.8, 0.8);

    const BORDER_WIDTH: f32 = 1.;
    const BORDER_RADIUS: f32 = 5.;

    const DEFAULT_TEXT_INPUT_STYLE: text_input::Style = text_input::Style{
        background: BACKGROUND_CLR,
        border_radius: BORDER_RADIUS,
        border_width: BORDER_WIDTH,
        border_color: ACCENT_CLR,
    };


    // Defines 3 styles
    #[derive(Clone, Copy)]
    pub enum TextInput {
        Enabled,
        Disabled,
        Invalid,
    }

    impl Default for TextInput {
        fn default() -> Self {
            TextInput::Enabled
        }
    }

    // sets style based on above enum
    impl text_input::StyleSheet for TextInput {

        fn active(&self) -> text_input::Style {
            DEFAULT_TEXT_INPUT_STYLE
        }

        fn focused(&self) -> text_input::Style {
            match self {
                TextInput::Disabled => DEFAULT_TEXT_INPUT_STYLE,
                _ => {
                    text_input::Style {
                        border_color: Color::BLACK,
                        ..DEFAULT_TEXT_INPUT_STYLE
                    }
                },
            }
        }

        fn placeholder_color(&self) -> Color {
            TEXT_CLR_DISABLED
        }

        fn value_color(&self) -> Color {
            match self {
                TextInput::Enabled => TEXT_CLR,
                TextInput::Disabled => TEXT_CLR_DISABLED,
                TextInput::Invalid => TEXT_CLR_INVALID,
            }
        }

        fn selection_color(&self) -> Color {
            match self {
                TextInput::Disabled => Color::TRANSPARENT,
                _ => HIGHLIGHT_CLR,
            }

        }
    }
}
