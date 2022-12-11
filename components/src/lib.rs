use iced::{Application, Theme, Command, Element, Renderer};
use iced::executor;


#[derive(Debug, Default)]
pub struct IgneousApp;


impl Application for IgneousApp {
    type Executor = executor::Default;
    type Theme = Theme;
    type Message = ();
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        "Igneous".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        "Igneous".into()
    }
}