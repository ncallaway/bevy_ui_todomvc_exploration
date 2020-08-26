use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    app.add_system(on_complete_todo_click.system());
    // app.add_system(count_label_system.system())
    //     .add_system_to_stage(ui_stage::USER_EVENTS, on_filter_tab_button_click.system())
    //     .add_system(set_filter_tab_active_system.system());
}

fn on_complete_todo_click(
    mut click_query: Query<(Entity, &CompleteTodoButton, &Parent, Mutated<Interaction>)>,
    mut row_query: Query<(Entity, &TodoRow)>,
) {
    for (btn, _, btn_parent, i) in &mut click_query.iter() {
        if *i == Interaction::Clicked {
            println!("Clicked a complete button, finding the parent...");
            for (todo_row, row) in &mut row_query.iter() {
                if todo_row == btn_parent.0 {
                    println!(
                        "FOUND A PARENT, WE CAN COMPLETE THE TODO NOW, AMAZING: {}",
                        row.0
                    );
                }
            }
        }
    }
}

struct TodoRow(String);
struct CompleteTodoButton;

fn spawn_complete_todo_button(ctx: &mut NodeContext) -> Entity {
    let e = text_button_node(
        ctx,
        TextButtonNode {
            label: TextNode {
                text: "   ",
                ..Default::default()
            },
            padding: Some(Rect::all(sizes::SPACER_XS)),
            ..Default::default()
        },
    );

    ctx.cmds.insert_one(e, CompleteTodoButton);

    e
}

pub fn spawn_todo_row(ctx: &mut NodeContext) -> Entity {
    let string = Todo::random_message();

    let e = div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
            flex_direction: Some(FlexDirection::Row),
            padding: Some(Rect::all(sizes::SPACER_SM)),
            ..Default::default()
        },
        |ctx| {
            vec![
                spawn_complete_todo_button(ctx),
                text_node(
                    ctx,
                    TextNode {
                        text: string.as_str(),
                        font_size: Some(sizes::FONT_LARGE),
                        margin: Some(Rect::left(sizes::SPACER_LG)),
                        ..Default::default()
                    },
                ),
            ]
        },
    );

    ctx.cmds.insert_one(e, TodoRow(string));

    e
}
