// SPDX-License-Identifier: GPL-3.0-only

use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::widget::{self, icon, nav_bar};
use cosmic::{Application, ApplicationExt, Apply, Element};

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct YourApp {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// A model that contains all of the pages assigned to the nav bar panel.
    nav: nav_bar::Model,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {}

/// Identifies a page in the application.
pub enum Page {
    Page1,
    Page2,
    Page3,
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the executor that will be used to run your application.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for YourApp {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    // Instructs the cosmic runtime to use this model as the nav bar model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text("Page 1")
            .data::<Page>(Page::Page1)
            .icon(icon::from_name("applications-science-symbolic"))
            .activate();

        nav.insert()
            .text("Page 2")
            .data::<Page>(Page::Page2)
            .icon(icon::from_name("applications-system-symbolic"));

        nav.insert()
            .text("Page 3")
            .data::<Page>(Page::Page3)
            .icon(icon::from_name("applications-games-symbolic"));

        let mut app = YourApp { core, nav };

        let command = app.update_titles();

        (app, command)
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        widget::text::title1(fl!("welcome"))
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        self.update_titles()
    }
}

impl YourApp {
    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<Message> {
        let mut window_title = fl!("app-title");
        let mut header_title = String::new();

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
            header_title.push_str(page);
        }

        self.set_header_title(header_title);
        self.set_window_title(window_title)
    }
}
