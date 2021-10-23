use crate::{config::ForestConfig, ui::themes::*};
use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Radio,
    Row, Rule, Sandbox, Text, TextInput,
};

#[derive(Default)]
pub struct Forest {
    input: text_input::State,
    input_value: String,
    button: button::State,
    config: ForestConfig,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    InputChanged(String),
    ButtonPressed(ForestButton),
}

#[derive(Debug, Clone)]
pub enum ForestButton {
    Next,
}

impl Sandbox for Forest {
    type Message = Message;

    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        "Forest".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ThemeChanged(theme) => self.config.set_theme(theme),
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed(button) => {
                match button {
                    ForestButton::Next => {
                        self.config.set_api_key(self.input_value.as_str());
                    }
                }
            }
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
                        Some(self.config.theme()),
                        Message::ThemeChanged,
                    )
                    .style(self.config.theme()),
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
        .style(self.config.theme())
        .password();

        let button = Button::new(&mut self.button, Text::new("Next"))
            .padding(10)
            .on_press(Message::ButtonPressed(ForestButton::Next))
            .style(self.config.theme());

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
            .push(Rule::horizontal(38).style(self.config.theme()))
            .push(choose_theme)
            .push(Rule::horizontal(38).style(self.config.theme()))
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
            .style(self.config.theme())
            .into()
    }
}
