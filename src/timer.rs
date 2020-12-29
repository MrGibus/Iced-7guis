use iced::{button, executor, Align, Button, Row, Element, Settings, Text, HorizontalAlignment,
           Length, Column, ProgressBar, Slider, slider, time, Application, Command, Subscription};
use std::time::{Duration, Instant};

// the window size is an unsigned 32 bit integer, the padding is unsigned 16 bit integer
const SIZE: (u32, u32) = (350, 180);
const PAD: u16 = 25;

pub fn main() -> iced::Result{
    // Set the window properties
    let mut settings = Settings::default();
    settings.window.size = SIZE;
    settings.window.resizable = false;

    // run the application
    Timer::run(settings)
}


struct Timer {
    elapsed: f32,
    max_time: f32,
    btn: button::State,
    slider: slider::State,
    start_time: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            elapsed: 0.,
            max_time: 30.,
            btn: button::State::default(),
            slider: slider::State::default(),
            start_time: Instant::now(),
        }
    }
}

// define the possible interactions of the application
#[derive(Debug, Clone, Copy)]
enum Message {
    ResetPressed,
    SliderChange(f32),
    Tic(Instant),
}

// Can't use the simplified 'Sandbox' for asyncronous applications
impl Application for Timer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    // in an Application flags can be provided to the creation of the model
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none()
            )
    }

    fn title(&self) -> String {
        String::from("Timer")
    }

    fn update(&mut self, message:Message) -> Command<Message> {
        match message {
            Message::ResetPressed => {
                self.start_time = Instant::now();
                self.elapsed = 0.;
            }
            Message::SliderChange(v) => (self.max_time = v),
            Message::Tic(_) => {
                let now = Instant::now();
                let elapsed = (now - self.start_time).as_secs_f32();
                if elapsed <= self.max_time {
                    self.elapsed = elapsed
                }
            },
        };
        Command::none()
    }

    // produces messages for update to handle as long as it is running
    fn subscription(&self) -> Subscription<Message> {
        // emits a Tic Message every 16 ms
        time::every(Duration::from_millis(16)).map(Message::Tic)
    }

    fn view(&mut self) -> Element<Message> {

        let progbar = ProgressBar::new(0.0..=self.max_time, self.elapsed);

        let slider_time = Slider::new(&mut self.slider,
                                      1.0..=60.,
                                      self.max_time,
                                      Message::SliderChange
        ).step(0.1);

        let top_row = Row::new()
            .push(
                Text::new("Elapsed Time: ")
            )
            .push(progbar);

        let bot_row = Row::new()
            .push(
                Text::new("Duration: ")
            )
            .push(slider_time);


        Column::new()
            .padding(PAD)
            .spacing(10)
            .align_items(Align::Center)
            .push(top_row)
            .push(
                Text::new(format!("{:.1}s", &self.elapsed))
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(Length::FillPortion(2)),
            )
            .push(bot_row)
            .push(
                Button::new(&mut self.btn,
                            Text::new("Reset")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)
                )
                    .width(Length::Fill)
                    .on_press(Message::ResetPressed),
            )
            .into()
    }
}
