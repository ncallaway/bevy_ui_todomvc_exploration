use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    app.add_system(count_label_system.system())
        .add_system_to_stage(ui_stage::USER_EVENTS, on_filter_tab_button_click.system())
        .add_system(set_filter_tab_active_system.system());
}

fn on_filter_tab_button_click(
    mut filter: ResMut<Filter>,
    mut click_query: Query<(&FilterTabButton, Mutated<Interaction>)>,
) {
    for (ft, i) in &mut click_query.iter() {
        if *i == Interaction::Clicked {
            *filter = ft.0;
        }
    }
}

fn set_filter_tab_active_system(
    filter: Res<Filter>,
    mut button_query: Query<(&FilterTabButton, &mut ButtonBehavior)>,
) {
    for (ft, mut button) in &mut button_query.iter() {
        button.is_active = ft.0 == *filter;
    }
}

// fn on_add_button_clicked(
//     mut commands: Commands,
//     mut click_query: Query<(&AddTodoButton, Mutated<Interaction>)>,
// ) {
//     for (_, interaction) in &mut click_query.iter() {
//         if *interaction == Interaction::Clicked {
//             println!("spawning a new todo...");
//             commands.spawn((Todo::new(Todo::random_message()),));
//         }
//     }
// }

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
            background: ctx.colors.white.into(),
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

struct FilterTabButton(Filter);

fn spawn_tab_button(ctx: &mut NodeContext, filter: Filter, label: &str, last: bool) -> Entity {
    let e = text_button_node(
        ctx,
        TextButtonNode {
            label: TextNode {
                text: label,
                ..Default::default()
            },
            color_normal: ctx.colors.white.into(),
            color_hover: ctx.colors.background_hover_red.into(),
            color_active: ctx.colors.background_active_red.into(),
            color_pressed: ctx.colors.background_pressed_red.into(),
            margin: Some(Rect {
                left: Val::Px(0.0),
                right: if last { sizes::ZERO } else { sizes::SPACER },
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            }),
            ..Default::default()
        },
    );

    ctx.cmds.insert_one(e, FilterTabButton(filter));
    e
}

fn spawn_tab_controls(ctx: &mut NodeContext) -> Entity {
    let e = div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
            flex_direction: Some(FlexDirection::Row),
            ..Default::default()
        },
        |ctx| {
            vec![
                spawn_tab_button(ctx, Filter::All, "All", false),
                spawn_tab_button(ctx, Filter::Active, "Active", false),
                spawn_tab_button(ctx, Filter::Completed, "Completed", true),
            ]
        },
    );

    e
}

pub fn spawn_todo_footer(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
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
