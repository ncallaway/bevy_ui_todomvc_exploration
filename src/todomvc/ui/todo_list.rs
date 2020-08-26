use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    // app.add_system(count_label_system.system())
    //     .add_system_to_stage(ui_stage::USER_EVENTS, on_filter_tab_button_click.system())
    //     .add_system(set_filter_tab_active_system.system());
}
// fn spawn_todo_list(ctx: &mut NodeContext) -> Entity {
//     div_node(
//         ctx,
//         DivNode {
//             ..Default::default()
//         },
//         |ctx| vec![todo_footer::spawn_todo_footer(ctx)],
//     )
// }

pub fn spawn_todo_list(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        |ctx| vec![todo_row::spawn_todo_row(ctx), todo_row::spawn_todo_row(ctx)],
    )
}
