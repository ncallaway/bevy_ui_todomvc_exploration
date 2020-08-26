use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    app.add_system(on_complete_todo_click.system())
        .add_system_to_stage(ui_stage::VISUAL_SYNC, sync_row_system.system());
}

fn sync_row_system(
    mut row_query: Query<(Entity, &TodoRow)>,
    mut todos: Query<(Entity, Changed<Todo>)>,
    mut row_labels: Query<(&Parent, &RowLabel, &mut Text)>,
    mut complete_buttons: Query<(Entity, &Parent, &CompleteTodoButton)>,
    mut complete_button_labels: Query<(&Parent, &TextButtonLabel, &mut Text)>,
) {
    // for (e, row) in &mut row_query.iter() {
    // }
    for (todo_e, todo) in &mut todos.iter() {
        for (row_e, row) in &mut row_query.iter() {
            if todo_e == row.0 {
                sync_row(
                    row_e,
                    &(*todo),
                    &mut row_labels,
                    &mut complete_buttons,
                    &mut complete_button_labels,
                );
            }
        }
    }
}

fn sync_row(
    row: Entity,
    todo: &Todo,
    row_labels: &mut Query<(&Parent, &RowLabel, &mut Text)>,
    complete_buttons: &mut Query<(Entity, &Parent, &CompleteTodoButton)>,
    complete_button_labels: &mut Query<(&Parent, &TextButtonLabel, &mut Text)>,
) {
    // apply the text update...
    for (parent, _, mut text) in &mut row_labels.iter() {
        if row == parent.0 {
            text.value = todo.label.clone();
            break;
        }
    }

    // apply the btn update...
    for (btn, parent, _) in &mut complete_buttons.iter() {
        if row == parent.0 {
            for (parent, _, mut text) in &mut complete_button_labels.iter() {
                if btn == parent.0 {
                    text.value = if todo.completed { " X " } else { "   " }.to_string()
                    //" ✓ "
                }
            }
        }
    }
}

fn on_complete_todo_click(
    mut click_query: Query<(Entity, &CompleteTodoButton, &Parent, Mutated<Interaction>)>,
    mut row_query: Query<(Entity, &TodoRow)>,
    mut todos: Query<(Entity, &mut Todo)>,
) {
    for (btn, _, btn_parent, i) in &mut click_query.iter() {
        if *i == Interaction::Clicked {
            for (todo_row, row) in &mut row_query.iter() {
                if todo_row == btn_parent.0 {
                    for (todo_e, mut todo) in &mut todos.iter() {
                        if todo_e == row.0 {
                            todo.completed = !todo.completed;
                        }
                    }

                    // this produces queryerror: cannot write archetype
                    // match todos.get_mut::<Todo>(row.0) {
                    //     Ok(mut todo) => {
                    //         println!("Found a row AND a matching todo...");
                    //         todo.completed = !todo.completed;
                    //     }
                    //     Err(e) => {
                    //         println!("Query Error when trying to get todo: {:?}", e);
                    //     }
                    // }
                }
            }
        }
    }
}

/// TodoRow holds a reference to the Entity that it represents.
#[derive(Debug)]
pub struct TodoRow(pub Entity);
struct CompleteTodoButton;

fn spawn_complete_todo_button(ctx: &mut NodeContext) -> Entity {
    let e = text_button_node(
        ctx,
        TextButtonNode {
            label: TextNode {
                text: "   ",
                color: Some(colors::TEXT_LIGHT),
                ..Default::default()
            },
            padding: Some(Rect::all(sizes::SPACER_XS)),
            size: Some(Size::new(Val::Px(50.0), Val::Auto)),
            ..Default::default()
        },
    );

    ctx.cmds.insert_one(e, CompleteTodoButton);

    e
}

struct RowLabel;

pub fn spawn_todo_row(ctx: &mut NodeContext, todo: Entity) -> Entity {
    let e = div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
            flex_direction: Some(FlexDirection::Row),
            padding: Some(Rect::all(sizes::SPACER_SM)),
            ..Default::default()
        },
        |ctx| {
            let btn = spawn_complete_todo_button(ctx);
            let txt = text_node(
                ctx,
                TextNode {
                    text: "",
                    font_size: Some(sizes::FONT_LARGE),
                    margin: Some(Rect::left(sizes::SPACER_LG)),
                    ..Default::default()
                },
            );

            ctx.cmds.insert_one(txt, RowLabel);

            vec![btn, txt]
        },
    );

    ctx.cmds.insert_one(e, TodoRow(todo));

    e
}
