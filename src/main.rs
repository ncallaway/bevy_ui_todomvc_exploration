use bevy::prelude::*;

mod todomvc;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(todomvc::TodoPlugin)
        .run();
}
