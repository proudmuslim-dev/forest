use iced::{
    button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Radio, Row, Rule, Sandbox, Text, TextInput,
};

use crate::{config::ForestConfig, ui::themes::*};

#[derive(Default)]
pub struct Forest {
    steps: Steps,
    next_button: button::State,
    back_button: button::State,
    input: text_input::State,
    input_value: String,
    config: ForestConfig,
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
            Message::ButtonPressed(button) => match button {
                ForestButton::Next => {
                    if !self.input_value.is_empty() && self.steps.current == 0 {
                        self.config.set_api_key(self.input_value.as_str());
                    }

                    self.steps.forward();
                }
                ForestButton::Back => {
                    self.steps.go_back();
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self.steps.steps[self.steps.current] {
            Step::Welcome => self.welcome(),
            Step::Dashboard => self.dashboard(),
        };

        content.into()
    }
}

impl Forest {
    fn welcome(&mut self) -> Container<Message> {
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

        let next_button = Button::new(&mut self.next_button, Text::new("Next"))
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
            .push(Row::new().spacing(10).push(text_input).push(next_button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.config.theme())
    }

    fn dashboard(&mut self) -> Container<Message> {
        let back_button = Button::new(&mut self.back_button, Text::new("Back"))
            .padding(10)
            .on_press(Message::ButtonPressed(ForestButton::Back))
            .style(self.config.theme());

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(
                Row::new().spacing(10).push(
                    Text::new("Under construction...")
                        .width(Length::Fill)
                        .size(50)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(10)
                    .padding(10)
                    .push(back_button),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.config.theme())
    }
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
    Back,
}

pub struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![Step::Welcome, Step::Dashboard],
            current: 0,
        }
    }
    fn forward(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
    }
}

impl Default for Steps {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Step {
    Welcome,
    Dashboard,
}
