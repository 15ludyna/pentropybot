use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn make_keyboard() -> KeyboardMarkup {
    let keyboard = KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("Entropy")
        ],
        vec![
            KeyboardButton::new("Info")
        ]
    ])
    .resize_keyboard();

    keyboard
}
