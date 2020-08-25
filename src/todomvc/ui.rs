use bevy::input::keyboard::ElementState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use smallvec::smallvec;

use super::rect_helpers::RectHelpers;

mod colors {
    use bevy::prelude::Color;

    const GRAY_1: Color = Color::rgb(0.95, 0.95, 0.95);
    const GRAY_3: Color = Color::rgb(0.6, 0.6, 0.6);
    const GRAY_8: Color = Color::rgb(0.1, 0.1, 0.1);
    const GRAY_9: Color = Color::rgb(0.05, 0.05, 0.05);

    pub const PAGE_BACKGROUND: Color = GRAY_1;
    pub const HEADER_RED: Color =
        Color::rgba(175f32 / 255f32, 47f32 / 255f32, 47f32 / 255f32, 0.45);
    pub const TEXT_MUTED: Color = GRAY_3;
    pub const TEXT: Color = GRAY_8;
    pub const WHITE: Color = Color::WHITE;
}

mod sizes {
    use bevy::prelude::Val;

    pub const SPACER_XS: Val = Val::Px(5.0);
    pub const SPACER_SM: Val = Val::Px(10.0);
    pub const SPACER_MD: Val = Val::Px(20.0);
    pub const SPACER_LG: Val = Val::Px(40.0);
    pub const SPACER_XL: Val = Val::Px(80.0);
    pub const SPACER: Val = SPACER_MD;

    pub const APP_WIDTH: Val = Val::Px(550.0);
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

pub fn build(app: &mut AppBuilder) {
    app.add_event::<NodeClickEvent>()
        .add_event::<FocusEvent>()
        .add_event::<BlurEvent>()
        .init_resource::<ButtonMaterials>()
        .init_resource::<Focus>()
        .init_resource::<FocusableClickedState>()
        .init_resource::<TodoInputReaderState>()
        .add_startup_system(setup_ui.system())
        .add_system(node_click_event_source.system())
        .add_system(focusable_click_system.system())
        .add_system(on_todo_input_focus.system())
        .add_system(button_interaction_system.system())
    // .add_system(clear_click_focus_system.system());
    ;
}

#[derive(Debug, Clone)]
struct NodeClickEvent {
    clicked: Entity,
}

#[derive(Debug, Clone)]
struct FocusEvent {
    focused: Entity,
}

#[derive(Debug, Clone)]
struct BlurEvent {
    blurred: Entity,
}

struct TodoInputNode {}

struct Root;

struct Focusable {
    has_focus: bool,
}

#[derive(Default)]
struct Focus {
    entity: Option<Entity>,
}

struct NodeContext<'a> {
    asset_server: Res<'a, AssetServer>,
    fonts: ResMut<'a, Assets<Font>>,
    materials: ResMut<'a, Assets<ColorMaterial>>,
    button_materials: Res<'a, ButtonMaterials>,
    font: Handle<Font>,
}

fn node_click_event_source(
    mut ui_node_click_events: ResMut<Events<NodeClickEvent>>,
    mut interation_query: Query<(Entity, Mutated<Interaction>)>,
) {
    for (entity, interaction) in &mut interation_query.iter() {
        if let Interaction::Clicked = *interaction {
            ui_node_click_events.send(NodeClickEvent { clicked: entity });
        }
    }
}

#[derive(Default)]
struct FocusableClickedState {
    node_click_reader: EventReader<NodeClickEvent>,
    mouse_click_reader: EventReader<MouseButtonInput>,
}

fn focusable_click_system(
    node_click_events: Res<Events<NodeClickEvent>>,
    mouse_click_events: Res<Events<MouseButtonInput>>,
    mut click_state: ResMut<FocusableClickedState>,
    mut focus_events: ResMut<Events<FocusEvent>>,
    mut blur_events: ResMut<Events<BlurEvent>>,
    mut focusable_query: Query<(Entity, &mut Focusable)>,
    // mut interation_query: Query<(&TodoInputNode, &mut Focusable, Mutated<Interaction>)>,
) {
    // for (node, mut focus, int) in &mut interation_query.iter() {
    //     if let Interaction::Clicked = *int {
    //         println!("todo input has focus (before): {}", focus.has_focus);
    //         focus.has_focus = true;
    //         println!("todo input has focus (after): {}", focus.has_focus);
    //     }
    // }

    let mut mouse_clicked = false;
    for e in click_state.mouse_click_reader.iter(&mouse_click_events) {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
            mouse_clicked = true
        }
    }
    // map, any because we always want our reader to consume all the events
    // .map(|e| e.button == MouseButton::Left && e.state == ElementState::Pressed)
    // .any(|e| e);

    // let focusable_entity_clicked = click_state
    //     .node_click_reader
    //     .iter(&node_click_events)
    // .map(|e| focusable_query.get::<Focusable>(e.clicked).ok());
    // .find(|e| e.is_some());

    let mut focusable_entity_clicked = None;
    let mut focusable_clicked = false;
    for e in click_state.node_click_reader.iter(&node_click_events) {
        let f = focusable_query.get::<Focusable>(e.clicked).ok();
        if f.is_some() {
            focusable_entity_clicked = Some(e.clicked);
            focusable_clicked = true;
        }
    }

    if let Some(clicked) = focusable_entity_clicked {
        for (entity, focusable) in &mut focusable_query.iter() {
            set_focus(
                entity,
                focusable,
                entity == clicked,
                &mut focus_events,
                &mut blur_events,
            );
        }
    }

    if mouse_clicked {
        println!(
            "Mouse clicked! Was a focusable clicked: {} ",
            focusable_clicked
        );
    }

    if focusable_clicked && !mouse_clicked {
        println!("FOCUS CLICK WITHOUT A MOUSE CLICK");
    }

    if !focusable_clicked && mouse_clicked {
        for (entity, focusable) in &mut focusable_query.iter() {
            set_focus(
                entity,
                focusable,
                false,
                &mut focus_events,
                &mut blur_events,
            );
        }
    }
}

fn set_focus(
    entity: Entity,
    mut focusable: Mut<Focusable>,
    is_focused: bool,
    focus_events: &mut ResMut<Events<FocusEvent>>,
    blur_events: &mut ResMut<Events<BlurEvent>>,
) {
    let had_focus = focusable.has_focus;

    if is_focused != had_focus && is_focused {
        println!("focusing: {:?}", entity);
        focus_events.send(FocusEvent { focused: entity });
    }

    if is_focused != had_focus && !is_focused {
        println!("blurruing: {:?}", entity);
        blur_events.send(BlurEvent { blurred: entity });
    }

    focusable.has_focus = is_focused;
}

#[derive(Default)]
struct TodoInputReaderState {
    focus_reader: EventReader<FocusEvent>,
    blur_reader: EventReader<BlurEvent>,
}

// fn spam_todo_input_events(
//     mut readers: ResMut<TodoInputReaderState>,
//     focus_events: Res<Events<FocusEvent>>,
//     blur_events: Res<Events<BlurEvent>>,
//     inputs: Query<&TodoInputNode>,
// ) {
//     for event in readers.focus_reader.iter(&focus_events) {
//         if let Ok(_) = inputs.get::<TodoInputNode>(event.focused) {
//             println!("Got focus!");
//         }
//     }

//     for event in readers.blur_reader.iter(&blur_events) {
//         if let Ok(_) = inputs.get::<TodoInputNode>(event.blurred) {
//             println!("Lost focus :/");
//         }
//     }
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
            println!("And we have a child node");
            // find the Text element
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
        println!("TodoInput was blurred: {:?}", event.blurred);
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

fn button_interaction_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &mut Handle<ColorMaterial>)>,
) {
    for (_, interaction, mut material) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<Assets<Font>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    let font = asset_server
        .load("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    let mut ctx = NodeContext {
        asset_server: asset_server,
        fonts: fonts,
        materials: materials,
        button_materials: button_materials,
        font: font,
    };

    // 2d camera
    commands.spawn(UiCameraComponents::default());

    // root
    root_node(&mut commands, &mut ctx, |cmds, ctx| {
        vec![
            div_node(
                cmds,
                ctx,
                Div {
                    background: colors::PAGE_BACKGROUND,
                    ..Default::default()
                },
                |cmds, ctx| {
                    vec![
                        heading_node(cmds, ctx, "todos"),
                        div_node(
                            cmds,
                            ctx,
                            Div {
                                background: colors::PAGE_BACKGROUND,
                                align_items: Some(AlignItems::Center),
                                margin: Some(Rect::top(sizes::SPACER)),
                                ..Default::default()
                            },
                            |cmds, ctx| {
                                vec![input_node(cmds, ctx), insert_todo_button_node(cmds, ctx)]
                            },
                        ),
                    ]
                    // heading_node(cmds, parent, ctx, "todos");
                    // div_node(
                    //     cmds,
                    //     parent,
                    //     ctx,
                    //     Div {
                    //         background: colors::PAGE_BACKGROUND,
                    //         align_items: Some(AlignItems::Center),
                    //         margin: Some(Rect::top(sizes::SPACER)),
                    //         ..Default::default()
                    //     },
                    //     |cmds, p, ctx| {
                    //         input_node(cmds, p, ctx);
                    //         insert_text_button_node(cmds, p, ctx);
                    //     },
                    // );
                },
            ),
            div_node(
                cmds,
                ctx,
                Div {
                    background: colors::PAGE_BACKGROUND,
                    ..Default::default()
                },
                |cmds, ctx| {
                    vec![
                        text_node(
                            cmds,
                            ctx,
                            "Double-click to edit a todo",
                            Some(Txt {
                                color: Some(colors::TEXT_MUTED),
                                ..Default::default()
                            }),
                        ),
                        text_node(
                            cmds,
                            ctx,
                            "Made with bevy_ui",
                            Some(Txt {
                                color: Some(colors::TEXT_MUTED),
                                ..Default::default()
                            }),
                        ),
                    ]
                },
            ),
        ]
    });
}

fn input_bundle(ctx: &mut NodeContext) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            max_size: Size::new(Val::Px(550.0), Val::Auto),
            ..Default::default()
        },
        material: ctx.materials.add(colors::WHITE.into()),
        ..Default::default()
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

fn input_node(commands: &mut Commands, ctx: &mut NodeContext) -> Entity {
    let e = Entity::new();

    commands
        .spawn_as_entity(e, input_bundle(ctx))
        .with_children(|p| {
            p.spawn(input_node_label_bundle(ctx));
        })
        .with(TodoInputNode {})
        .with(Focusable { has_focus: false })
        .with(Interaction::default());

    return e;
}

fn heading_node(commands: &mut Commands, ctx: &NodeContext, heading: &str) -> Entity {
    let e = Entity::new();
    commands.spawn_as_entity(e, heading_bundle(heading, ctx));

    return e;
}

fn heading_bundle(heading: &str, ctx: &NodeContext) -> TextComponents {
    return text_bundle(
        ctx,
        heading,
        Txt {
            font_size: Some(100f32),
            color: Some(colors::HEADER_RED),
            ..Default::default()
        },
    );
}

fn div_bundle(ctx: &mut NodeContext, opts: Div) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: opts.align_items.unwrap_or_default(),
            padding: opts.padding.unwrap_or_default(),
            margin: opts.margin.unwrap_or_default(),
            ..Default::default()
        },
        material: ctx.materials.add(opts.background.into()),
        ..Default::default()
    }
}

#[derive(Default)]
struct Div {
    background: Color,
    align_items: Option<AlignItems>,
    padding: Option<Rect<Val>>,
    margin: Option<Rect<Val>>,
}

fn div_node(
    commands: &mut Commands,
    ctx: &mut NodeContext,
    opts: Div,
    mut children: impl FnMut(&mut Commands, &mut NodeContext) -> Vec<Entity>,
) -> Entity {
    let children = children(commands, ctx);

    let e = Entity::new();
    commands
        .spawn_as_entity(e, div_bundle(ctx, opts))
        .push_children(e, &children);

    return e;
}

#[derive(Default)]
struct Txt {
    font_size: Option<f32>,
    color: Option<Color>,
    padding: Option<Rect<Val>>,
    margin: Option<Rect<Val>>,
}

fn text_bundle(ctx: &NodeContext, heading: &str, opts: Txt) -> TextComponents {
    return TextComponents {
        style: Style {
            align_self: AlignSelf::Center,
            padding: opts.padding.unwrap_or_default(),
            margin: opts.margin.unwrap_or_default(),
            ..Default::default()
        },
        text: Text {
            value: heading.to_string(),
            font: ctx.font,
            style: TextStyle {
                font_size: opts.font_size.unwrap_or(16.0),
                color: opts.color.unwrap_or(colors::HEADER_RED),
            },
        },
        ..Default::default()
    };
}

fn text_node(commands: &mut Commands, ctx: &NodeContext, text: &str, opts: Option<Txt>) -> Entity {
    let e = Entity::new();
    commands.spawn_as_entity(e, text_bundle(ctx, text, opts.unwrap_or_default()));

    return e;
}

fn root_bundle(context: &mut NodeContext) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            padding: Rect::xy(sizes::SPACER_XL, sizes::SPACER_LG),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        material: context.materials.add(colors::PAGE_BACKGROUND.into()),
        ..Default::default()
    }
}

fn root_node(
    commands: &mut Commands,
    ctx: &mut NodeContext,
    children: impl Fn(&mut Commands, &mut NodeContext) -> Vec<Entity>,
) -> Entity {
    let root = Entity::new();

    let children = children(commands, ctx);

    commands
        .spawn_as_entity(root, root_bundle(ctx))
        .with(Root)
        .push_children(root, &children);

    return root;
}

struct InsertTodoButton;

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

// fn button(
//     commands: &mut ChildBuilder,
//     button_materials: &Res<ButtonMaterials>,
//     font: Handle<Font>,
// ) {
//     commands
//         .spawn(ButtonComponents {
//             style: Style {
//                 size: Size::new(Val::Percent(100.0), Val::Px(45.0)),
//                 // center button
//                 margin: Rect::all(Val::Px(0.0)),
//                 // horizontally center child text
//                 justify_content: JustifyContent::Center,
//                 // // vertically center child text
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//             },
//             material: button_materials.normal,
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             // button label
//             parent.spawn(TextComponents {
//                 text: Text {
//                     value: "Send a message".to_string(),
//                     font: font,
//                     style: TextStyle {
//                         font_size: 12.0,
//                         color: Color::rgb(0.8, 0.8, 0.8),
//                     },
//                 },
//                 ..Default::default()
//             });
//         });
// }
