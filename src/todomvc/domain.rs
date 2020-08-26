use bevy::prelude::*;
use rand::seq::SliceRandom;

pub fn build(app: &mut AppBuilder) {
    app.init_resource::<Filter>();
}

#[derive(PartialEq, Copy, Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

pub struct Todo {
    pub label: String,
    pub completed: bool,
    pub ordinal: u16,
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            label: label,
            completed: false,
            ordinal: 0, // 0 is our special ordinal to indicate that the compactor should move it to the end of the list
        }
    }

    pub fn random_message() -> String {
        let msgs = [
            "Lorem ipsum dolor sit amet",
            "consectetur adipiscing elit",
            "Proin vel eros dolor",
            "Cras luctus vehicula ex",
            "Proin vel eros dolor",
            "at dapibus massa viverra id",
            "Vestibulum nec tempor lacus",
            "eget lobortis ligula",
            "Sed vel gravida neque",
            "ac sollicitudin purus",
            "Aenean aliquet odio quis nulla varius",
            "Phasellus vitae nibh leo",
            "Maecenas lobortis porttitor consectetur",
            "Sed congue, ex a blandit congue",
            "erat erat ullamcorper orci",
            "vitae euismod eros lacus ut eros",
            "Ut molestie metus leo",
            "eget posuere tellus maximus a",
            "Nulla porttitor faucibus ullamcorper",
            "Phasellus feugiat felis at odio",
            "consectetur lacinia. Nullam fermentum",
            "malesuada consequat",
        ];

        return msgs.choose(&mut rand::thread_rng()).unwrap().to_string();
    }
}
