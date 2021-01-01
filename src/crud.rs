use iced::{button, Button, Row, Element, Sandbox, Settings, Text, scrollable, Scrollable,
           HorizontalAlignment, Length, Column, Container, text_input, TextInput, Align, Radio};


const DEFAULT_SIZE: (u32, u32) = (450, 250);
const MIN_SIZE: (u32, u32) = (400, 200);
const PAD: u16 = 10;
const SPACING: u16 = 5;
const PAD_SMALL: u16 = 2;
const TEXTINPUT_WIDTH: u16 = 100;


pub fn main() -> iced::Result{
    // Set the window properties
    let mut settings = Settings::default();
    settings.window.size = DEFAULT_SIZE;
    settings.window.min_size = Some(MIN_SIZE);

    // run the application
    CRUD::run(settings)
}


struct ListItem {
    index: usize,
    label: String,
}

impl ListItem {
    fn new(index: usize, label: &str) -> Self {
        ListItem {
            index,
            label: label.to_string()
        }
    }
}


struct List {
    items: Vec<ListItem>,
    selected: Option<usize>,
    scroll: scrollable::State,
}

impl Default for List {
    fn default() -> List {
        let v = vec![
            "Emil, Hans",
            "Mustermann, Max",
            "Tisch, Roman",
        ];

        List {
            items: v.iter()
                    .enumerate()
                    .map(
                        |(i, s)| ListItem::new(i, s)
                    ).collect(),
            selected: None,
            scroll: scrollable::State::default()
        }
    }
}

impl List {
    fn view(&mut self, filter: &str) -> Container<Message> {

        let iterlist = self.items
            .iter()
            .filter(|i| i.label.to_lowercase().contains(&filter.to_lowercase()));

        let content = iterlist.fold(
            Column::new()
                .padding(PAD)
                .spacing(1),
            |column, item| {
                column.push(Radio::new(
                    item.index,
                    &item.label,
                    self.selected,
                    Message::SelectionChanged))
                // column.push(Text::new(item))
            }
        );



        let content = Scrollable::new(&mut self.scroll)
            .push(content.width(Length::Fill));

        Container::new(content).style(style::ListView)
    }

    fn push(&mut self, label: &str) {
        let index = self.items.len();
        let item = ListItem::new(index, label);
        self.items.push(item);
    }

    fn remove(&mut self) {
        if let Some(i) = self.selected {
            self.items.remove(i);
            self.items = self.items.iter()
                .enumerate()
                .map(|(j, item)| ListItem::new(j, &item.label))
                .collect();
            self.selected = None;
        }
    }

    fn replace(&mut self, label: &str) {
        if let Some(i) = self.selected {
            let item = ListItem::new(i, label);
            self.items[i] = item;
        }
    }
}


#[derive(Debug, Clone)]
enum Message {
    CreatePressed,
    UpdatePressed,
    DeletePressed,
    FilterUpdated(String),
    NameUpdated(String),
    SurnameUpdated(String),
    SelectionChanged(usize),
}


// To simplify the data model fields
#[derive(Default)]
struct InputData {
    value: String,
    state: text_input::State
}


#[derive(Default)]
struct CRUD {
    names: List,
    input_filter: InputData,
    input_name: InputData,
    input_surname: InputData,
    btn_create: button::State,
    btn_update: button::State,
    btn_delete: button::State,
}

impl CRUD {
    fn make_name(&self) -> String {
        format!("{}, {}", self.input_surname.value, self.input_name.value)
    }
}

impl Sandbox for CRUD {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("C.R.U.D.")
    }

    fn update(&mut self, message:Message) {
        match message {
            Message::FilterUpdated(s) => self.input_filter.value = s,
            Message::NameUpdated(s) => self.input_name.value = s,
            Message::SurnameUpdated(s) => self.input_surname.value = s,
            Message::SelectionChanged(i) => self.names.selected = Some(i),
            Message::CreatePressed => self.names.push(&self.make_name()),
            Message::UpdatePressed => self.names.replace(&self.make_name()),
            Message::DeletePressed =>  self.names.remove(),
        }
    }

    fn view(&mut self) -> Element<Message> {

        let in_filter = TextInput::new(
                &mut self.input_filter.state,
                "Filter",
                &self.input_filter.value,
                Message::FilterUpdated
            )
                .padding(PAD_SMALL)
                .width(Length::Units(TEXTINPUT_WIDTH));

        let header = Row::new()
            .align_items(Align::Center)
            .spacing(SPACING)
            .height(Length::Shrink)
            .push(Text::new("Filter Prefix:".to_string()))
            .push(in_filter);

        let btn_create = Button::new(&mut self.btn_create,
                                 Text::new("Create".to_string())
                                         .horizontal_alignment(HorizontalAlignment::Center)
                                         .width(Length::Fill)
        ).on_press(Message::CreatePressed);

        let btn_update = Button::new(&mut self.btn_update,
                                 Text::new("Update".to_string())
                                         .horizontal_alignment(HorizontalAlignment::Center)
                                         .width(Length::Fill)
        ).on_press(Message::UpdatePressed);

        let btn_delete = Button::new(&mut self.btn_delete,
                                 Text::new("Delete".to_string())
                                         .horizontal_alignment(HorizontalAlignment::Center)
                                         .width(Length::Fill)
        ).on_press(Message::DeletePressed);

        let footer = Row::new()
            .spacing(SPACING)
            .push(btn_create)
            .push(btn_update)
            .push(btn_delete)
            .height(Length::Shrink);

        let left = self.names.view(&self.input_filter.value)
            .width(Length::Fill)
            .height(Length::Fill);

        let in_name = TextInput::new(
                &mut self.input_name.state,
                "Name",
                &self.input_name.value,
                Message::NameUpdated
            )
                .padding(2)
                .width(Length::Units(TEXTINPUT_WIDTH));

        let in_surname = TextInput::new(
                &mut self.input_surname.state,
                "Surname",
                &self.input_surname.value,
                Message::SurnameUpdated
            )
                .padding(PAD_SMALL)
                .width(Length::Units(TEXTINPUT_WIDTH));

        let right1 = Row::new()
            .push(Text::new("Name: "))
            .push(in_name);

        let right2 = Row::new()
            .push(Text::new("Surname: "))
            .push(in_surname);

        let right = Column::new()
            .spacing(SPACING)
            .align_items(Align::End)
            .push(right1)
            .push(right2);

        let body = Row::new()
            .spacing(PAD)
            .push(left)
            .push(right)
            .height(Length::Fill);

        Column::new()
            .padding(PAD)
            .spacing(SPACING)
            .push(header)
            .push(body)
            .push(footer)
            .into()
    }
}


mod style {

    pub struct ListView;

    use iced::{container, Background, Color};

    impl container::StyleSheet for ListView {
        fn style(&self) -> container::Style {
            container::Style {
                border_color: Color::BLACK,
                border_width: 2.,
                border_radius: 5.,
                background: Some(Background::from(Color::from_rgb(0.95, 0.95, 0.95))),
                ..container::Style::default()
            }
        }
    }
}

