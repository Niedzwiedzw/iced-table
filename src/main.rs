use iced::{container::Style, Application, Column, Command, Container, Element, Row, Text};

pub struct Header<T> {
    pub name: String,
    pub value: Box<dyn Fn(&T) -> String>,
    pub sort_value: Box<dyn Fn(&T) -> String>,
}

impl<T> std::fmt::Debug for Header<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HeaderConfig<{}> {{ name: {} }}",
            std::any::type_name::<T>(),
            self.name
        )
    }
}

#[derive(Debug)]
pub struct Message;

#[derive(Debug)]
pub struct Table<'a, T> {
    pub headers: Vec<Header<T>>,
    pub items: Vec<&'a T>,
}

impl<'a, T> Table<'a, T> {
    fn cell<'element>(text: &str) -> Element<'element, Message> {
        Container::new(Text::new(text.to_string()))
            .style(style::Cell)
            .width(iced::Length::Fill)
            .into()
    }
    pub fn view<'element>(&self) -> Element<'element, Message> {
        let headers = self
            .headers
            .iter()
            .fold(Row::new(), |row, header| row.push(Self::cell(&header.name)))
            .spacing(10);
        let rows = self.items.iter().fold(Column::new(), |column, item| {
            column.push(
                self.headers
                    .iter()
                    .fold(Row::new(), |row, Header { value, .. }| {
                        row.push(Self::cell(&value(item)))
                    })
                    .spacing(10),
            )
        });
        let table = Column::new().push(headers).push(rows);
        Container::new(table).into()
    }
}

struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

pub struct DisplayStuff {
    people: Vec<Person>,
}

impl Application for DisplayStuff {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                people: vec![
                    Person {
                        first_name: "Michael".into(),
                        last_name: "Michaelowski iiiii".into(),
                        age: 32,
                    },
                    Person {
                        first_name: "Michael".into(),
                        last_name: "Michaelowski".into(),
                        age: 33,
                    },
                    Person {
                        first_name: "Michael".into(),
                        last_name: "Michaelowski".into(),
                        age: 34,
                    },
                    Person {
                        first_name: "Michael".into(),
                        last_name: "Michaelowski".into(),
                        age: 35,
                    },
                ],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "nothing special".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let table = Table::<Person> {
            headers: vec![
                Header {
                    name: "First name".to_string(),
                    value: Box::new(|person| person.first_name.clone()),
                    sort_value: Box::new(|person| person.first_name.clone()),
                },
                Header {
                    name: "Last name".to_string(),
                    value: Box::new(|person| person.last_name.clone()),
                    sort_value: Box::new(|person| person.last_name.clone()),
                },
                Header {
                    name: "Full name".to_string(),
                    value: Box::new(
                        |Person {
                             first_name,
                             last_name,
                             ..
                         }| format!("{first_name} {last_name}"),
                    ),
                    sort_value: Box::new(
                        |Person {
                             first_name,
                             last_name,
                             ..
                         }| format!("{first_name} {last_name}"),
                    ),
                },
                Header {
                    name: "Age".to_string(),
                    value: Box::new(|person| person.age.to_string()),
                    sort_value: Box::new(|person| person.age.to_string()),
                },
            ],
            items: self.people.iter().collect::<Vec<_>>(),
        };
        Container::new(Column::new().push(table.view()))
            .padding(10)
            .into()
    }
}

fn main() -> iced::Result {
    DisplayStuff::run(iced::Settings::default())
}

mod style {
    use iced::Color;

    use super::*;
    pub struct Cell;
    impl iced::container::StyleSheet for Cell {
        fn style(&self) -> Style {
            Style {
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color::BLACK,
                ..Default::default()
            }
        }
    }
}
