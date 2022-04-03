use iced::{
    button,
    canvas::{self, Cache, Cursor, Geometry, Path},
    text_input, window, Align, Button, Canvas, Color, Column, Container, Element, Length, Point,
    Rectangle, Row, Sandbox, Settings, Size, Text, TextInput,
};

pub fn main() -> iced::Result {
    Timer::run(Settings {
        window: window::Settings {
            size: (240, 110),
            min_size: Some((240, 110)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Default)]
struct Timer {
    text: String,
    start: button::State,
    stop: button::State,
    restart: button::State,
    text_input: text_input::State,
    cache: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Stop,
    Restart,
    InputChanged(String),
}

impl Sandbox for Timer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Hourglass")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                // self.value += 1;
            }
            Message::Stop => {
                // self.value -= 1;
            }
            Message::Restart => {
                // self.value -= 1;
            }
            Message::InputChanged(text) => {
                self.text = text;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // let row: Row<Message> = Row::new()
        //     .padding(10)
        //     .align_items(Align::Center)
        //     .push(Button::new(&mut self.start, Text::new("Start")).on_press(Message::Start))
        //     .push(Button::new(&mut self.stop, Text::new("Stop")).on_press(Message::Stop))
        //     .push(Button::new(&mut self.restart, Text::new("Restart")).on_press(Message::Restart));

        // let column = Column::new()
        //     .align_items(Align::Center)
        //     .padding(10)
        //     .push(TextInput::new(
        //         &mut self.text_input,
        //         "Enter time:",
        //         &self.text,
        //         Message::InputChanged,
        //     ))
        //     .push(row);

        let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Timer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.cache.draw(bounds.size(), |frame| {
            let top_left = Point::new(0.0, 0.0);
            let size = Size::new(frame.width(), frame.height());
            let background = Path::rectangle(top_left, size);
            let blue = Color::from_rgb8(54, 101, 179);
            frame.fill(&background, blue);

            let top_left = Point::new(15.0, 15.0);
            let size = Size::new(frame.width() - 30.0, frame.height() - 30.0);
            let background = Path::rectangle(top_left, size);
            frame.fill(&background, Color::from_rgb8(30, 30, 30));
        });

        vec![clock]
    }
}
