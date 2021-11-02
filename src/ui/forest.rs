use iced::{
    button, executor, text_input, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Radio, Row, Rule, Text, TextInput,
};
use iced_aw::{modal, Card, Modal};

use crate::{config::ForestConfig, ui::themes::*};

#[derive(Default)]
pub struct Forest {
    config: ForestConfig,
    steps: Steps,
    state: State,
}

impl Application for Forest {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Forest".to_owned()
    }

    // TODO: Migrate update logic to `State` struct
    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => self.config.set_theme(theme),
            Message::InputChanged(value) => self.state.input_value = value,
            Message::ButtonPressed(button) => match button {
                ForestButton::Next => {
                    if !self.state.input_value.is_empty() && self.steps.current == 0 {
                        self.config.set_api_key(self.state.input_value.as_str());
                    }

                    // Quick & dirty check
                    if self.steps.steps[self.steps.current] == Step::Welcome
                        && self.state.input_value.is_empty()
                        && self.config.api_key().is_empty()
                    {
                        self.state.modal_state.show(true);
                    } else {
                        self.steps.forward();
                    }
                }
                ForestButton::Back => {
                    self.steps.go_back();
                }
            },
            Message::OpenModal => {
                self.state.modal_state.show(true);
            }
            Message::CloseModal => {
                self.state.modal_state.show(false);
            }
        }

        Command::none()
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

        // TODO: API key validation via RegEx
        let text_input = TextInput::new(
            &mut self.state.input,
            "Enter API key...",
            &self.state.input_value,
            Message::InputChanged,
        )
        .size(20)
        .padding(10)
        .style(self.config.theme())
        .on_submit(Message::ButtonPressed(ForestButton::Next))
        .password();

        let next_button = Button::new(&mut self.state.next_button, Text::new("Next"))
            .padding(10)
            .on_press(Message::ButtonPressed(ForestButton::Next))
            .style(self.config.theme());

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(
                Row::new().align_items(Align::Center).spacing(10).push(
                    Text::new("Welcome to Forest!")
                        .width(Length::Fill)
                        .size(40)
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

        let theme = self.config.theme();

        // TODO: Figure out why it's impossible to align this correctly
        let modal = Modal::new(&mut self.state.modal_state, content, move |state| {
            Card::new(Text::new("Please enter an API Key."), Text::new(""))
                .foot(
                    Row::new().spacing(10).padding(5).width(Length::Fill).push(
                        Button::new(
                            &mut state.ok_state,
                            Text::new("Ok").horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Fill)
                        .style(theme)
                        .on_press(Message::CloseModal),
                    ),
                )
                .max_width(300)
                .on_close(Message::CloseModal)
                .style(theme)
                .into()
        })
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal)
        .style(self.config.theme());

        Container::new(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.config.theme())
    }

    fn dashboard(&mut self) -> Container<Message> {
        let back_button = Button::new(&mut self.state.back_button, Text::new("Back"))
            .padding(10)
            .on_press(Message::ButtonPressed(ForestButton::Back))
            .style(self.config.theme());

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Row::new().spacing(10).padding(10).push(back_button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.config.theme())
    }
}

#[derive(Default)]
pub struct State {
    modal_state: modal::State<ModalState>,
    next_button: button::State,
    back_button: button::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    InputChanged(String),
    ButtonPressed(ForestButton),
    OpenModal,
    CloseModal,
}

#[derive(Default)]
struct ModalState {
    ok_state: button::State,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Eq, PartialEq)]
pub enum Step {
    Welcome,
    Dashboard,
}
