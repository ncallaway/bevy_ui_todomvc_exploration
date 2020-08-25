use bevy::prelude::*;

use super::rect_helpers::RectHelpers;
use super::ui::colors;
use super::ui::sizes;
use super::ui::*;

pub fn build(app: &mut AppBuilder) {
    app.init_resource::<TodoInputReaderState>()
        .add_system(on_todo_input_focus.system());
}

pub struct TodoInputNode {}
pub struct InsertTodoButton;

#[derive(Default)]
struct TodoInputReaderState {
    focus_reader: EventReader<FocusEvent>,
    blur_reader: EventReader<BlurEvent>,
}

fn on_todo_input_focus(
    mut commands: Commands,
    mut readers: ResMut<TodoInputReaderState>,
    focus_events: Res<Events<FocusEvent>>,
    blur_events: Res<Events<BlurEvent>>,
    asset_server: Res<AssetServer>,
    fonts: ResMut<Assets<Font>>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    inputs: Query<(&TodoInputNode, &mut Children)>,
    texts: Query<(Entity, &Text)>,
    add_buttons: Query<(Entity, &InsertTodoButton)>,
) {
    let font = asset_server
        .get_handle("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    let mut ctx = NodeContext {
        asset_server: asset_server,
        fonts: fonts,
        materials: materials,
        button_materials: button_materials,
        font: font,
    };

    for event in readers.focus_reader.iter(&focus_events) {
        println!("TodoInput was focused: {:?}", event.focused);
        if let Ok(focused_children) = inputs.get_mut::<Children>(event.focused) {
            for child in &focused_children.0 {
                if let Ok(_) = texts.get::<Text>(*child) {
                    commands.despawn_recursive(*child);
                }
            }

            let child = insert_todo_button_node(&mut commands, &mut ctx);
            commands.push_children(event.focused, &[child]);
        }
    }

    for event in readers.blur_reader.iter(&blur_events) {
        if let Ok(blurred_children) = inputs.get::<Children>(event.blurred) {
            for child in &blurred_children.0 {
                if let Ok(_) = add_buttons.get::<InsertTodoButton>(*child) {
                    commands.despawn_recursive(*child);
                }
            }

            let created = Entity::new();
            commands
                .spawn_as_entity(created, input_node_label_bundle(&ctx))
                .push_children(event.blurred, &[created]);
        }
    }
}

fn input_node_label_bundle(ctx: &NodeContext) -> TextComponents {
    text_bundle(
        ctx,
        "What needs to be done?",
        Txt {
            font_size: Some(24.0),
            color: Some(colors::TEXT_MUTED),
            margin: Some(Rect::xy(sizes::SPACER_LG, sizes::SPACER_SM)),
            ..Default::default()
        },
    )
}

pub fn spawn_todo_input_node(commands: &mut Commands, ctx: &mut NodeContext) -> Entity {
    let e = Entity::new();

    let bundle = NodeComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            max_size: Size::new(Val::Px(550.0), Val::Auto),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: ctx.materials.add(colors::WHITE.into()),
        ..Default::default()
    };

    commands
        .spawn_as_entity(e, bundle)
        .with_children(|p| {
            p.spawn(input_node_label_bundle(ctx));
        })
        .with(TodoInputNode {})
        .with(Focusable::default())
        .with(Interaction::default());

    return e;
}

fn insert_todo_button_node(commands: &mut Commands, ctx: &NodeContext) -> Entity {
    let e = Entity::new();

    commands
        .spawn_as_entity(
            e,
            ButtonComponents {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    // center button
                    padding: Rect::xy(sizes::SPACER, sizes::SPACER_XS),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: ctx.button_materials.normal,
                ..Default::default()
            },
        )
        .with(InsertTodoButton)
        .with_children(|parent| {
            // button label
            parent.spawn(TextComponents {
                text: Text {
                    value: "Add a random todo".to_string(),
                    font: ctx.font,
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        });

    return e;
}
