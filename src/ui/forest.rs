use crate::ui::themes::*;
use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Radio,
    Row, Rule, Sandbox, Text, TextInput,
};

#[derive(Default)]
pub struct Forest {
    theme: style::Theme,
    input: text_input::State,
    input_value: String,
    button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    InputChanged(String),
    ButtonPressed,
}

impl Sandbox for Forest {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Forest".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::InputChanged(value) => self.input_value = value,
            // TODO: Store API key to config file on click
            Message::ButtonPressed => (),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        &format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        let text_input = TextInput::new(
            &mut self.input,
            "Enter API key...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let button = Button::new(&mut self.button, Text::new("Next"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(
                Row::new().spacing(10).push(
                    Text::new("Welcome to Forest!")
                        .width(Length::Fill)
                        .size(50)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
            )
            .push(Rule::horizontal(38).style(self.theme))
            .push(choose_theme)
            .push(Rule::horizontal(38).style(self.theme))
            .push(
                Row::new()
                    .spacing(10)
                    .push(Text::new("Enter your API key to continue:")),
            )
            .push(Row::new().spacing(10).push(text_input).push(button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
