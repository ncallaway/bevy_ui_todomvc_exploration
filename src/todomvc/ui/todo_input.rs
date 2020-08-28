use bevy::prelude::*;

use super::colors;
use super::sizes;
use super::*;
use crate::rect_helpers::RectHelpers;
use crate::todomvc::domain::Todo;

pub fn build(app: &mut AppBuilder) {
    app.init_resource::<TodoInputReaderState>()
        .add_system_to_stage(ui_stage::USER_EVENTS, on_add_button_clicked.system())
        .add_system_to_stage(ui_stage::USER_EVENTS, on_todo_input_focus.system());
}

pub struct TodoInputNode {}
pub struct AddTodoButton;

#[derive(Default)]
struct TodoInputReaderState {
    focus_reader: EventReader<FocusEvent>,
    blur_reader: EventReader<BlurEvent>,
}

fn on_add_button_clicked(
    mut commands: Commands,
    mut click_query: Query<(&AddTodoButton, Mutated<Interaction>)>,
) {
    for (_, interaction) in &mut click_query.iter() {
        if *interaction == Interaction::Clicked {
            let label = Todo::random_message();
            commands.spawn((Todo::new(label),));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn on_todo_input_focus(
    mut commands: Commands,
    mut readers: ResMut<TodoInputReaderState>,
    focus_events: Res<Events<FocusEvent>>,
    blur_events: Res<Events<BlurEvent>>,
    asset_server: Res<AssetServer>,
    fonts: ResMut<Assets<Font>>,
    materials: Res<ColorMaterials>,
    asset_materials: ResMut<Assets<ColorMaterial>>,
    inputs: Query<(&TodoInputNode, &mut Children)>,
    texts: Query<(Entity, &Text)>,
    mut add_buttons: Query<(Entity, &AddTodoButton)>,
) {
    let font = asset_server
        .get_handle("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    let mut ctx = NodeContext {
        cmds: &mut commands,
        asset_server,
        fonts,
        colors: materials,
        asset_materials,
        font,
    };

    // on focus, despawn the placeholder and spawn the placeholder
    for event in readers.focus_reader.iter(&focus_events) {
        if let Ok(focused_children) = inputs.get_mut::<Children>(event.focused) {
            for child in &focused_children.0 {
                if texts.get::<Text>(*child).is_ok() {
                    ctx.cmds.despawn_recursive(*child);
                }
            }

            let child = spawn_add_button_node(&mut ctx);
            ctx.cmds.push_children(event.focused, &[child]);
        }
    }

    // on focus, despawn the placeholder and spawn the placeholder
    for event in readers.blur_reader.iter(&blur_events) {
        if let Ok(blurred_children) = inputs.get::<Children>(event.blurred) {
            for child in &blurred_children.0 {
                // normally we'd use add_buttons.get::() here, but see below
                for (e, _) in &mut add_buttons.iter() {
                    if e == *child {
                        ctx.cmds.despawn_recursive(*child);
                    }
                }
                // todo: the following is producing a `Query error: CannotReadArchetype`. Is it my fault or
                // bevy's fault? Who knows! I'll figure it out later.
                // let r = add_buttons.get::<InsertTodoButton>(*child);
                // if let Ok(_) = r {
                //     println!("\t\t\tDespawning button recurisve: {:?}", child);
                //     commands.despawn_recursive(*child);
                // } else if let Err(x) = r {
                //     println!("\t\t\tQuery error: {:?}", x);
                // }
            }

            let label = spawn_placeholder_label(&mut ctx);
            ctx.cmds.push_children(event.blurred, &[label]);
        }
    }
}

pub fn spawn_todo_input_node(ctx: &mut NodeContext) -> Entity {
    let e = Entity::new();

    let bundle = NodeComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: ctx.colors.white,
        ..Default::default()
    };

    let children = [spawn_placeholder_label(ctx)];

    ctx.cmds
        .spawn_as_entity(e, bundle)
        .with(TodoInputNode {})
        .with(Focusable::default())
        .with(Interaction::default())
        .push_children(e, &children);

    e
}

fn spawn_placeholder_label(ctx: &mut NodeContext) -> Entity {
    super::text_node(
        ctx,
        TextNode {
            text: "What needs to be done?",
            font_size: Some(sizes::FONT_LARGE),
            color: Some(colors::TEXT_MUTED),
            margin: Some(Rect::xy(sizes::SPACER_LG, sizes::SPACER_SM)),
            ..Default::default()
        },
    )
}

fn spawn_add_button_node(ctx: &mut NodeContext) -> Entity {
    let e = text_button_node(
        ctx,
        TextButtonNode {
            label: TextNode {
                text: "Add a random todo",
                color: Some(colors::TEXT_LIGHT),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    ctx.cmds.insert_one(e, AddTodoButton);

    e
}
