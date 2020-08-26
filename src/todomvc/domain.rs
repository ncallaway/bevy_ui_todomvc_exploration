use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::todomvc::ui::ui_stage;

pub fn build(app: &mut AppBuilder) {
    app.init_resource::<Filter>()
        .add_system_to_stage(ui_stage::DOMAIN_SYNC, todo_ordering_system.system());
}

fn todo_ordering_system(mut todos: Query<&mut Todo>) {
    let mut i = todos.iter();
    let mut sorted_todos: Vec<Mut<Todo>> = i.into_iter().collect();

    sorted_todos.sort_by(|a, b| a.ordinal.cmp(&b.ordinal));

    let mut next = 0;
    let mut needs_compact = false;

    // Do we need to compact?
    for todo in &sorted_todos {
        if todo.ordinal != next {
            needs_compact = true;
        }
        next = next + 1;
    }

    // If so, do it.
    if needs_compact {
        let mut next = 0;
        for todo in &mut sorted_todos {
            todo.ordinal = next;
            next = next + 1;
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
            ordinal: u16::MAX,
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
