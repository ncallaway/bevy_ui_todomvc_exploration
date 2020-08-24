use bevy::prelude::*;
pub struct TodoPlugin;

mod ui;

impl Plugin for TodoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        ui::build(app);
    }
}
