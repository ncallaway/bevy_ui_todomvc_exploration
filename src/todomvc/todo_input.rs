use bevy::prelude::*;

use super::rect_helpers::RectHelpers;
use super::ui::colors;
use super::ui::sizes;
use super::ui::*;

use super::domain::Todo;

pub fn build(app: &mut AppBuilder) {
    app.init_resource::<TodoInputReaderState>()
        .add_system(on_add_button_clicked.system())
        .add_system(on_todo_input_focus.system());
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
    mut click_query: Query<(Entity, &AddTodoButton, Mutated<Interaction>)>,
) {
    for (e, _button, interaction) in &mut click_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("an add todo button was clicked: {:?}", e);

            commands.spawn((Todo::new(Todo::random_message()),));
        }
    }
}

// fn add_message_system(
//   mut commands: Commands,
//   ci: Res<ConnectionInfo>,
//   mut network_create_messages: ResMut<CreateMessages>,
//   mut interaction_query: Query<(&Button, Mutated<Interaction>)>,
// ) {
//   for (_button, interaction) in &mut interaction_query.iter() {
//       if let Interaction::Clicked = *interaction {
//           if ci.is_server() {
//               // immediately create the message
//               commands.spawn((Message::new(&random_message(), "server", 255),));
//           } else {
//               // schedule the message to be sent to the server
//               network_create_messages.messages.push(random_message());
//           }
//       }
//   }
// }

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
    mut add_buttons: Query<(Entity, &AddTodoButton)>,
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
        if let Ok(focused_children) = inputs.get_mut::<Children>(event.focused) {
            for child in &focused_children.0 {
                if let Ok(_) = texts.get::<Text>(*child) {
                    println!("\tDespawning placeholder label recurisve: {:?}", child);
                    commands.despawn_recursive(*child);
                }
            }

            let child = spawn_add_button_node(&mut commands, &mut ctx);
            commands.push_children(event.focused, &[child]);
        }
    }

    for event in readers.blur_reader.iter(&blur_events) {
        if let Ok(blurred_children) = inputs.get::<Children>(event.blurred) {
            for child in &blurred_children.0 {
                // normally we'd use add_buttons.get::() here, but see below
                for (e, _) in &mut add_buttons.iter() {
                    if e == *child {
                        println!("\tDespawning button recurisve: {:?}", child);
                        commands.despawn_recursive(*child);
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

            let label = spawn_placeholder_label(&mut commands, &mut ctx);
            commands.push_children(event.blurred, &[label]);
        }
    }
}

fn spawn_placeholder_label(commands: &mut Commands, ctx: &mut NodeContext) -> Entity {
    let bundle = text_bundle(
        ctx,
        "What needs to be done?",
        Txt {
            font_size: Some(24.0),
            color: Some(colors::TEXT_MUTED),
            margin: Some(Rect::xy(sizes::SPACER_LG, sizes::SPACER_SM)),
            ..Default::default()
        },
    );

    let e = Entity::new();
    println!("Spawning placeholder label: {:?}", e);
    commands.spawn_as_entity(e, bundle);
    return e;
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

    let children = [spawn_placeholder_label(commands, ctx)];

    commands
        .spawn_as_entity(e, bundle)
        .with(TodoInputNode {})
        .with(Focusable::default())
        .with(Interaction::default())
        .push_children(e, &children);

    return e;
}

fn spawn_add_button_node(commands: &mut Commands, ctx: &NodeContext) -> Entity {
    let e = Entity::new();

    println!("Spawning add button: {:?}", e);

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
        .with(AddTodoButton)
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