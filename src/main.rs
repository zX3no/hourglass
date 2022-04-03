use iced::{
    button,
    canvas::{self, Cache, Cursor, Geometry, Path, Program},
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
        //     .push(
        //         Button::new(&mut self.start, Text::new("Start"))
        //             .on_press(Message::Start)
        //             .style(style::Button::Primary),
        //     )
        //     .push(
        //         Button::new(&mut self.stop, Text::new("Stop"))
        //             .on_press(Message::Stop)
        //             .style(style::Button::Primary),
        //     )
        //     .push(
        //         Button::new(&mut self.restart, Text::new("Restart"))
        //             .on_press(Message::Restart)
        //             .style(style::Button::Primary),
        //     );

        // Column::new()
        //     .align_items(Align::Center)
        //     .padding(10)
        //     .push(TextInput::new(
        //         &mut self.text_input,
        //         "Enter time:",
        //         &self.text,
        //         Message::InputChanged,
        //     ))
        //     .push(row)
        //     .into()

        let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Program<Message> for Timer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let time = self.cache.draw(bounds.size(), |frame| {
            let top_left = Point::new(0.0, 0.0);
            let size = Size::new(frame.width(), frame.height());
            frame.fill_rectangle(top_left, size, Color::from_rgb8(54, 101, 179));

            let top_left = Point::new(15.0, 15.0);
            let size = Size::new(frame.width() - 30.0, frame.height() - 30.0);
            frame.fill_rectangle(top_left, size, Color::from_rgb8(30, 30, 30));
        });

        vec![time]
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Hover,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Hover => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 6.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
