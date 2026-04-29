use iced::widget::{Column, button, column, text};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            column![
                text("HARBOR v0.1").size(50),
                text("Manage your system ports in one place"),
                text("by Codemesh"),
            ],
            column![
                button("+").on_press(Message::Increment),
                text(self.value),
                button("-").on_press(Message::Decrement),
            ]
        ]
    }
}

fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

#[test]
fn test_counter() {
    let mut counter = Counter { value: 0 };
    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);
    assert_eq!(counter.value, 1);
}
