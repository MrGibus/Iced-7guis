#![allow(non_snake_case)]

mod counter;
mod temperature;
mod flights;
mod timer;
mod crud;

use iced::{button, Settings, Button, Column, Sandbox, Element, Text, HorizontalAlignment, Length};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (200, 400);
    settings.window.resizable = false;
    Landing::run(settings)
}


#[derive(Default)]
struct Landing{
    counter: button::State,
    temperature: button::State,
    flights: button::State,
    timer: button::State,
    crud: button::State,
    circle: button::State,
    cells: button::State,
}


#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Message{
    Counter,
    Temperature,
    Flights,
    Timer,
    CRUD,
    Circle,
    Cells,
}

impl Sandbox for Landing {
    type Message = Message;

    fn new() -> Self {Self::default()}
    fn title(&self) -> String {"7Guis - Iced".to_string()}

    fn update(&mut self, message: Message) {
         match message {
            Message::Counter => counter::main(),
            Message::Temperature => temperature::main(),
            Message:: Flights => flights::main(),
            Message::Timer => timer::main(),
            Message::CRUD => crud::main(),
            _ => Ok(())
        }.unwrap_or(());
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(10)
            .spacing(5)
            .push(Text::new("7GUIs\nfor\nIced")
                .width(Length::Fill)
                .size(40)
                .color([0.1, 0.1, 0.6])
                .horizontal_alignment(HorizontalAlignment::Center)
            )
            .push(
                Button::new(&mut self.counter, Text::new("Counter")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill).on_press(Message::Counter))
            .push(
                Button::new(&mut self.temperature, Text::new("Temperature")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill).on_press(Message::Temperature))
            .push(
                Button::new(&mut self.flights, Text::new("Flights")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill).on_press(Message::Flights))
            .push(
                Button::new(&mut self.timer, Text::new("Timer")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill).on_press(Message::Timer))
            .push(
                Button::new(&mut self.crud, Text::new("CRUD")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill).on_press(Message::CRUD))
            .push(
                Button::new(&mut self.circle, Text::new("Circles")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill))
            .push(
                Button::new(&mut self.cells, Text::new("Cells")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)).width(Length::Fill))
            .into()
    }
}