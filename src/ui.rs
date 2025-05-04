use std::path::PathBuf;

use iced::{
    widget::{ button, column, container, row, text, text::Style as TextStyle },
    Alignment, Border, Color, Element, Length, Padding, Task,
};

use opener;
use chrono::Local;
use native_dialog::DialogBuilder;

use crate::{ types::FileExt, reader::Reader, writer::Writer, worker };

#[derive(Default, PartialEq)]
pub enum ScreenState {
    #[default]
    Start,
    FileSelected,
    ScheduleGenerating,
    ScheduleGenerated,
}

#[derive(Default)]
pub struct Window {
    pub screen_state: ScreenState,
    pub reader: Reader,
    pub writer: Writer,
    pub path: Option<PathBuf>,
    pub file_name: String,
    pub exported: bool,
}

#[derive(Debug, Default, Clone)]
pub enum WindowMessage {
    #[default]
    None,
    ChooseFile,

    /// Generate the schedule
    Generate,

    /// 
    Generating,

    /// Export to file
    Export(FileExt),
    GoToGitHub,
    Reset,
}

impl Window {
    pub fn new() -> Self { Self::default() }

    fn reset(&mut self) {
        self.screen_state = ScreenState::Start;
        self.reader = Reader::default();
        self.writer = Writer::default();
        self.path = None;
        self.file_name = String::default();
    }

    pub fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        match message {
            WindowMessage::None => Task::none(),

            WindowMessage::ChooseFile => {
                let path = DialogBuilder::file()
                    .add_filter("Spreadsheet Files", &[ "xlsx", "csv" ])
                    .open_single_file()
                    .show()
                    .unwrap();

                if let Some(p) = path {
                    self.path = Some(p.clone());

                    if let Some(path_last) = p.iter().last() {
                        if let Some(path_str) = path_last.to_str() {
                            self.file_name = path_str.to_owned();
                        }
                    }

                    self.screen_state = ScreenState::FileSelected;
                    self.reader.read(p.as_path());

                    return Task::done(WindowMessage::None);
                }

                Task::none()
            }

            WindowMessage::Generate => {
                self.screen_state = ScreenState::ScheduleGenerating;

                Task::done(WindowMessage::Generating)
            },

            WindowMessage::Generating => {
                let now = Local::now();
                let output = worker::process(&self.reader.elements, now, 10);

                self.writer.init(10, now, &output);

                println!("{:?}", output);

                self.screen_state = ScreenState::ScheduleGenerated;

                Task::none()
            }

            WindowMessage::Export(file) => {
                self.writer.export(file);

                self.exported = true;
                Task::none()
            },

            WindowMessage::GoToGitHub => {
                let _ = opener::open(
                    "https://github.com/abhinavgunwant/schedulez"
                );

                Task::none()
            },

            WindowMessage::Reset => {
                self.reset();

                Task::done(WindowMessage::None)
            }
        }
    }

    pub fn view(&self) -> Element<WindowMessage> {
        let mut cols = column![];

        if self.screen_state == ScreenState::Start {
            cols = cols.push(
                container(
                    button("Choose File")
                        .on_press(WindowMessage::ChooseFile)
                )
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    
            );

            cols = cols.push(
                container(text("Choose an .xlsx or .csv file"))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );

            cols = cols.push(
                container(row![
                    text("See "),
                    button("github.com/abhinavgunwant/schedulez")
                        .style(|_,_| button::Style {
                            background: None,
                            text_color: iced::color!(100, 100, 255).into(),
                            border: Border { width: 0.0, ..Border::default() },
                            ..button::Style::default()
                        })
                        .padding(Padding::from(0))
                        .on_press(WindowMessage::GoToGitHub)
                ])
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );

            cols = cols.push(
                container(text("for more information"))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );
        } else {
            cols = cols.push(
                container(text(self.file_name.clone()))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );

            cols = cols.push(
                container(row![
                    text("Schedule for month: "),
                    text("May "),
                    text("Year: "),
                    text("2025"),
                ])
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(Padding {
                        bottom: 16.0,
                        ..Padding::default()
                    })
            );
        }

        if self.screen_state == ScreenState::FileSelected {
            cols = cols.push(
                container(
                    button("Generate Schedule")
                        .on_press(WindowMessage::Generate)
                )
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(Padding {
                        bottom: 8.0,
                        ..Padding::default()
                    })
            );

            cols = cols.push(
                container(button("Reset").on_press(WindowMessage::Reset))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );
        }

        if self.screen_state == ScreenState::ScheduleGenerating {
            cols = cols.push(
                container(button("Generating Schedule"))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(Padding {
                        bottom: 8.0,
                        ..Padding::default()
                    })
            );
        }

        if self.screen_state == ScreenState::ScheduleGenerated {
            cols = cols.push(
                container(text("Export as:"))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(Padding {
                        bottom: 8.0,
                        ..Padding::default()
                    })
            );

            cols = cols.push(
                container(row![
                    container(
                        button(".xlsx")
                            .on_press(WindowMessage::Export(FileExt::Xlsx))
                    )
                        .padding(Padding {
                            right: 8.0,
                            ..Padding::default()
                        }),

                    button(".csv (Coming Soon)"),
                ])
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(Padding {
                        bottom: 8.0,
                        ..Padding::default()
                    })
            );

            if self.exported {
                cols = cols.push(
                    container(
                        text("Export Successful!")
                            .style(|_| TextStyle {
                                color: Some(Color::from_rgb(0.5, 1.0, 0.5)),
                            })
                    )
                        .align_x(Alignment::Center)
                        .width(Length::Fill)
                        .padding(Padding {
                            bottom: 8.0,
                            ..Padding::default()
                        })
                );
            }

            cols = cols.push(
                container(button("Reset").on_press(WindowMessage::Reset))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
            );
        }

        container(cols)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}

