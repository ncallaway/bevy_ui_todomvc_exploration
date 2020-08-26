use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::Todo;

pub fn build(app: &mut AppBuilder) {
    app.add_system(count_label_system.system());
    // app.add_resource(PriorTodos(false))
    //     .add_system_to_stage(ui_stage::DOMAIN_EVENTS, on_todo_added.system());
}

fn count_label_system(mut q: Query<&Todo>, mut items: Query<(&mut Text, &mut CountLabel)>) {
    let count = q.iter().iter().len();

    for (mut text, mut label_count) in &mut items.iter() {
        let should_update = if let Some(prior_count) = label_count.0 {
            prior_count != count
        } else {
            true
        };

        if should_update {
            if count != 1 {
                text.value = format!("{} items left", count);
            } else {
                text.value = format!("{} item left", count);
            }

            label_count.0 = Some(count);
        }
    }
}
struct CountLabel(Option<usize>);

fn spawn_count_label(ctx: &mut NodeContext) -> Entity {
    let e = text_node(
        ctx,
        TextNode {
            text: "Some items left",
            ..Default::default()
        },
    );

    ctx.cmds.insert_one(e, CountLabel(None));

    e
}

fn spawn_clear_button(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: colors::WHITE,
            ..Default::default()
        },
        |ctx| {
            vec![text_node(
                ctx,
                TextNode {
                    text: "Clear Completed",
                    ..Default::default()
                },
            )]
        },
    )
}

fn spawn_tab_button(ctx: &mut NodeContext, label: &str, last: bool) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: colors::WHITE,
            margin: Some(Rect {
                left: Val::Px(0.0),
                right: if last { sizes::ZERO } else { sizes::SPACER },
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            }),
            ..Default::default()
        },
        |ctx| {
            vec![text_node(
                ctx,
                TextNode {
                    text: label,
                    ..Default::default()
                },
            )]
        },
    )
}

fn spawn_tab_controls(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: colors::WHITE,
            flex_direction: Some(FlexDirection::Row),
            ..Default::default()
        },
        |ctx| {
            vec![
                spawn_tab_button(ctx, "All", false),
                spawn_tab_button(ctx, "Active", false),
                spawn_tab_button(ctx, "Completed", true),
            ]
        },
    )
}

pub fn spawn_todo_footer(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: Color::WHITE,
            flex_direction: Some(FlexDirection::Row),
            justify_content: Some(JustifyContent::SpaceBetween),
            padding: Some(Rect::xy(sizes::SPACER, sizes::SPACER_SM)),
            ..Default::default()
        },
        |ctx| {
            vec![
                spawn_count_label(ctx),
                spawn_tab_controls(ctx),
                spawn_clear_button(ctx),
            ]
        },
    )
}
