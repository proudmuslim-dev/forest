use iced::{
    button, executor, text_input, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, Font, HorizontalAlignment, Length, Radio, Row, Rule, Svg, Text, TextInput,
};
use iced_aw::{modal, Card, Modal};
use std::collections::HashMap;

use crate::{
    api::{
        model::private::{Balances, OrderHistory},
        requests::util::*,
    },
    config::ForestConfig,
    ui::{themes::*, util},
};

// TODO: Clean up the absolute mess I created figuring this out
pub enum Forest {
    LoginScreen {
        config: ForestConfig,
        state: LoginState,
    },
    DashboardLoading {
        config: ForestConfig,
        state: DashboardLoadingState,
    },
    DashboardLoaded {
        state: DashboardState,
        config: ForestConfig,
        balances: Balances,
        orders: Vec<OrderHistory>,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadingComplete((Result<Balances, Error>, Result<Vec<OrderHistory>, Error>)),
    ButtonPressed(ForestButton),
    ThemeChanged(style::Theme),
    InputChanged(String),
    OpenModal,
    CloseModal,
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
            Forest::DashboardLoading { .. } | Forest::DashboardLoaded { .. } => "Dashboard",
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
                Forest::LoginScreen { config, .. }
                | Forest::DashboardLoading { config, .. }
                | Forest::DashboardLoaded { config, .. } => {
                    config.set_theme(theme);
                }
            },
            Message::InputChanged(value) => {
                if let Forest::LoginScreen { state, .. } = self {
                    state.input_value = value;
                }
            }
            Message::ButtonPressed(button) => match button {
                ForestButton::Next => match self {
                    Forest::LoginScreen { config, state } => {
                        if !state.input_value.is_empty() {
                            config.set_api_key(state.input_value.as_str());
                        }

                        *self = Forest::DashboardLoading {
                            config: ForestConfig::default(),
                            state: DashboardLoadingState::default(),
                        };
                        if let Forest::DashboardLoading { config, .. } = self {
                            return Command::perform(
                                util::load_dashboard(config.api_key()),
                                Message::LoadingComplete,
                            );
                        }
                    }
                    Forest::DashboardLoading { config, .. } => {
                        return Command::perform(
                            util::load_dashboard(config.api_key()),
                            Message::LoadingComplete,
                        )
                    }
                    _ => {}
                },
                ForestButton::Back => {
                    if let Forest::DashboardLoading { .. } = self {
                        *self = Forest::LoginScreen {
                            state: LoginState::default(),
                            config: ForestConfig::default(),
                        };
                    }
                }
            },
            Message::LoadingComplete((balances, orders)) => {
                // TODO: Use match statement on `orders` & `balances` for error screen
                if let Forest::DashboardLoading { .. } = self {
                    let balances = balances.unwrap();
                    let orders = orders.unwrap();
                    *self = Forest::DashboardLoaded {
                        balances,
                        orders,
                        config: ForestConfig::default(),
                        state: DashboardState::default(),
                    }
                }
            }
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
            Forest::DashboardLoading { .. } | Forest::DashboardLoaded { .. } => self.dashboard(),
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

        unimplemented!("Unreachable")
    }

    fn dashboard(&mut self) -> Container<Message> {
        fn section_title(title: &str) -> Row<Message> {
            Row::new().push(
                Text::new(title)
                    .width(Length::Fill)
                    .size(40)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
        }

        match self {
            Forest::DashboardLoading { config, state } => {
                let back_button = Button::new(&mut state.back_button, Text::new("Back"))
                    .padding(10)
                    .on_press(Message::ButtonPressed(ForestButton::Back))
                    .style(config.theme());

                let next_button = Button::new(
                    &mut state.force_next_button,
                    Text::new("Taking too long? Retry"),
                )
                .padding(10)
                .on_press(Message::ButtonPressed(ForestButton::Next))
                .style(config.theme());

                let content = Column::new()
                    .spacing(20)
                    .padding(20)
                    .max_width(600)
                    .push(section_title("Loading..."))
                    .push(Rule::horizontal(38).style(config.theme()))
                    .push(
                        Row::new()
                            .spacing(10)
                            .padding(10)
                            .push(back_button)
                            .push(next_button),
                    );

                return Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(config.theme());
            }
            Forest::DashboardLoaded {
                config,
                state,
                balances,
                orders,
            } => {
                const FIRA_CODE: Font = Font::External {
                    name: "Fira Code",
                    bytes: include_bytes!("../../assets/fonts/FiraCode-Regular.ttf"),
                };

                fn display_balance(
                    currency: Currency,
                    balances: &HashMap<String, f64>,
                ) -> Row<Message> {
                    fn get_icon_path(currency: &str) -> String {
                        format!("assets/icons/{}.svg", currency)
                    }

                    let (icon_path, currency) = match currency {
                        Currency::BTC => (get_icon_path("bitcoin"), "BTC"),
                        Currency::XMR => (get_icon_path("monero"), "XMR"),
                    };

                    Row::new()
                        .spacing(10)
                        .push(
                            Svg::from_path(icon_path)
                                .width(Length::Units(28))
                                .height(Length::Units(28)),
                        )
                        .push(
                            Text::new(format!("{}: {}", currency, balances.get(currency).unwrap()))
                                .horizontal_alignment(HorizontalAlignment::Left)
                                .font(FIRA_CODE),
                        )
                }

                fn display_orders(
                    mut order_history: Vec<OrderHistory>,
                ) -> Column<'static, Message> {
                    fn display_order(order: OrderHistory) -> Row<'static, Message> {
                        Row::new().spacing(10).push(Text::new(order.market))
                    }

                    let mut history = Column::new().spacing(10);

                    if order_history.is_empty() {
                        return history.push(Text::new("No active orders!"));
                    }

                    for _ in 0..order_history.len() {
                        history = history.push(display_order(order_history.pop().unwrap()))
                    }

                    history
                }

                let back_button = Button::new(&mut state.back_button, Text::new("Back"))
                    .padding(10)
                    .on_press(Message::ButtonPressed(ForestButton::Back))
                    .style(config.theme());

                let balances = Column::new()
                    .spacing(20)
                    .padding(20)
                    .max_width(300)
                    .push(section_title("Balances"))
                    .push(Rule::horizontal(38).style(config.theme()))
                    .push(display_balance(Currency::BTC, balances.get_all()))
                    .push(display_balance(Currency::XMR, balances.get_all()));

                let active_orders = Column::new()
                    .spacing(20)
                    .padding(20)
                    .max_width(300)
                    .push(section_title("Orders"))
                    .push(Rule::horizontal(38).style(config.theme()))
                    .push(display_orders(orders.clone()));

                let row_one = Row::new().push(balances).push(active_orders);

                let content = Column::new()
                    .push(row_one)
                    .push(back_button)
                    .align_items(Align::Center);

                return Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(config.theme());
            }
            _ => unimplemented!("Unreachable"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ForestButton {
    Next,
    Back,
}

#[derive(Default)]
pub struct LoginState {
    modal_state: modal::State<ModalState>,
    next_button: button::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Default)]
pub struct ModalState {
    ok_state: button::State,
}

#[derive(Default)]
pub struct DashboardLoadingState {
    back_button: button::State,
    force_next_button: button::State,
}

#[derive(Default)]
pub struct DashboardState {
    back_button: button::State,
}
