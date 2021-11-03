use iced::{
    button, executor, text_input, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Radio, Row, Rule, Text, TextInput,
};
use iced_aw::{modal, Card, Modal};

use crate::{config::ForestConfig, ui::themes::*};

pub enum Forest {
    LoginScreen {
        config: ForestConfig,
        state: LoginState,
    },
    Dashboard {
        config: ForestConfig,
        state: DashboardState,
    },
}

impl Application for Forest {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Forest::LoginScreen {
                config: ForestConfig::default(),
                state: LoginState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            Forest::LoginScreen { .. } => "Login",
            Forest::Dashboard { .. } => "Dashboard",
        };

        format!("Forest - {}", subtitle)
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => match self {
                Forest::LoginScreen { config, .. } | Forest::Dashboard { config, .. } => {
                    config.set_theme(theme);
                }
            },
            Message::InputChanged(value) => {
                if let Forest::LoginScreen { state, .. } = self {
                    state.input_value = value;
                }
            }
            Message::ButtonPressed(button) => match button {
                ForestButton::Next => {
                    if let Forest::LoginScreen { config, state } = self {
                        config.set_api_key(state.input_value.as_str());
                        *self = Forest::Dashboard {
                            config: ForestConfig::default(),
                            state: DashboardState::default(),
                        };
                    }
                }
                ForestButton::Back => {
                    if let Forest::Dashboard { .. } = self {
                        *self = Forest::LoginScreen {
                            state: LoginState::default(),
                            config: ForestConfig::default(),
                        };
                    }
                }
            },
            Message::CloseModal => {
                if let Forest::LoginScreen { state, .. } = self {
                    state.modal_state.show(false);
                }
            }
            Message::OpenModal => {
                if let Forest::LoginScreen { state, .. } = self {
                    state.modal_state.show(true);
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match self {
            Forest::LoginScreen { .. } => self.welcome(),
            Forest::Dashboard { .. } => self.dashboard(),
        };

        content.into()
    }
}

impl Forest {
    fn welcome(&mut self) -> Container<Message> {
        if let Forest::LoginScreen { state, config } = self {
            let choose_theme = style::Theme::ALL.iter().fold(
                Column::new().spacing(10).push(Text::new("Choose a theme:")),
                |column, theme| {
                    column.push(
                        Radio::new(
                            *theme,
                            &format!("{:?}", theme),
                            Some(config.theme()),
                            Message::ThemeChanged,
                        )
                        .style(config.theme()),
                    )
                },
            );

            // TODO: API key validation via RegEx
            let text_input = TextInput::new(
                &mut state.input,
                "Enter API key...",
                &state.input_value,
                Message::InputChanged,
            )
            .size(20)
            .padding(10)
            .style(config.theme())
            .on_submit(Message::ButtonPressed(ForestButton::Next))
            .password();

            let next_button = Button::new(&mut state.next_button, Text::new("Next"))
                .padding(10)
                .on_press(Message::ButtonPressed(ForestButton::Next))
                .style(config.theme());

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
                .push(Rule::horizontal(38).style(config.theme()))
                .push(choose_theme)
                .push(Rule::horizontal(38).style(config.theme()))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Enter your API key to continue:")),
                )
                .push(Row::new().spacing(10).push(text_input).push(next_button));

            let theme = config.theme();

            // TODO: Figure out why it's impossible to align this correctly
            let modal = Modal::new(&mut state.modal_state, content, move |modal_state| {
                Card::new(Text::new("Please enter an API Key."), Text::new(""))
                    .foot(
                        Row::new().spacing(10).padding(5).width(Length::Fill).push(
                            Button::new(
                                &mut modal_state.ok_state,
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
            .style(config.theme());

            return Container::new(modal)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(config.theme());
        }

        unimplemented!("This is unreachable")
    }

    fn dashboard(&mut self) -> Container<Message> {
        if let Forest::Dashboard { config, state } = self {
            let back_button = Button::new(&mut state.back_button, Text::new("Back"))
                .padding(10)
                .on_press(Message::ButtonPressed(ForestButton::Back))
                .style(config.theme());

            let content = Column::new()
                .spacing(20)
                .padding(20)
                .max_width(600)
                .push(Row::new().spacing(10).padding(10).push(back_button));

            return Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(config.theme());
        }

        unimplemented!("This is unreachable")
    }
}

#[derive(Default)]
pub struct LoginState {
    modal_state: modal::State<ModalState>,
    next_button: button::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Default)]
pub struct DashboardState {
    back_button: button::State,
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
pub struct ModalState {
    ok_state: button::State,
}

#[derive(Debug, Copy, Clone)]
pub enum ForestButton {
    Next,
    Back,
}
