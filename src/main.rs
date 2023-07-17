mod sifis_lamp;

use iced::executor;
use iced::widget::{column, container, slider, text, vertical_slider};
use iced::{Application, Element, Length, Settings, Theme};

use sifis_lamp::Lamps;

// Application entry point whose goal consists in running the application
// with the default settings
pub fn main() -> iced::Result {
    LampsViewer::run(Settings::default())
}

// List of messages sent by the application when one or more of
// its widgets changes
#[derive(Debug, Clone)]
pub enum Message {
    // All lamps have been found and their information retrieved
    Lamps,
}

// Lamps viewer data
#[derive(Default)]
pub struct LampsViewer {
    // Lamps
    lamps: Vec<Lamp>,
    // Toggle states
    toggles: Vec<u8>,
    // Sliders values
    sliders: Vec<u8>,
}

// Trait
impl Application for LampsViewer {
    // Type of messages exchanged within the application
    type Message = Message;
    // Default graphic theme used by the application
    type Theme = Theme;
    // Application flags
    type Flags = ();
    // Application executor
    type Executor = executor::Default;

    // Creates a new application.
    //
    // This function retrieves all the lamps in an environment, saves them, and
    // then uses their values to update the graphical interface
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(sifis_lamp::lamps(), |_| Message::Lamps),
        )
    }

    // Application title
    fn title(&self) -> String {
        String::from("Lamps Viewer")
    }

    // Change application state according to the sent messages
    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => {
                self.slider_value = value;
            }
        }
    }

    // Application interface
    fn view(&self) -> Element<Message> {
        let value = self.slider_value;

        let h_slider = container(slider(0..=100, value, Message::SliderChanged)).width(250);

        let v_slider =
            container(vertical_slider(0..=100, value, Message::SliderChanged)).height(200);

        let text = text(format!("{value}"));

        container(
            column![
                container(v_slider).width(Length::Fill).center_x(),
                container(h_slider).width(Length::Fill).center_x(),
                container(text).width(Length::Fill).center_x(),
            ]
            .spacing(25),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
