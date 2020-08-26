use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    app.add_system(count_label_system.system())
        .add_system_to_stage(ui_stage::USER_EVENTS, on_filter_tab_button_click.system())
        .add_system_to_stage(ui_stage::USER_EVENTS, on_clear_completed_click.system())
        .add_system(set_filter_tab_active_system.system());
}

fn on_clear_completed_click(
    mut commands: Commands,
    mut click_query: Query<(&ClearCompletedButton, Mutated<Interaction>)>,
    mut todo_query: Query<(Entity, &Todo)>,
) {
    for (_, i) in &mut click_query.iter() {
        if *i == Interaction::Clicked {
            println!("Clearing completed todos");
            for (e, todo) in &mut todo_query.iter() {
                if todo.completed {
                    commands.despawn_recursive(e);
                }
            }
        }
    }
}

fn on_filter_tab_button_click(
    mut filter: ResMut<Filter>,
    mut click_query: Query<(&FilterTabButton, Mutated<Interaction>)>,
) {
    for (ft, i) in &mut click_query.iter() {
        if *i == Interaction::Clicked {
            *filter = ft.0;
            println!("Filtering to: {:?}", ft.0);
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
    div_node(
        ctx,
        DivNode {
            flex_grow: Some(1.0),
            flex_direction: Some(FlexDirection::Row),
            justify_content: Some(JustifyContent::FlexStart),
            ..Default::default()
        },
        |ctx| {
            let e = text_node(
                ctx,
                TextNode {
                    text: "Some items left",
                    flex_grow: Some(0.5),
                    ..Default::default()
                },
            );

            ctx.cmds.insert_one(e, CountLabel(None));

            vec![e]
        },
    )
}

struct ClearCompletedButton;

fn spawn_clear_button(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            flex_grow: Some(1.0),
            flex_direction: Some(FlexDirection::Row),
            justify_content: Some(JustifyContent::FlexEnd),
            ..Default::default()
        },
        |ctx| {
            let btn = text_button_node(
                ctx,
                TextButtonNode {
                    label: TextNode {
                        text: "Clear Completed",
                        ..Default::default()
                    },
                    color_normal: ctx.colors.white.into(),
                    color_hover: ctx.colors.btn_light_hovered.into(),
                    color_pressed: ctx.colors.btn_light_pressed.into(),
                    flex_grow: Some(0.5),
                    ..Default::default()
                },
            );

            ctx.cmds.insert_one(btn, ClearCompletedButton);

            vec![btn]
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
            margin: Some(Rect::right(if last { sizes::ZERO } else { sizes::SPACER })),
            padding: Some(Rect::all(sizes::SPACER_XS)),
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
            justify_content: Some(JustifyContent::Center),
            flex_direction: Some(FlexDirection::Row),
            flex_grow: Some(1.0),
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
