use bevy::input::keyboard::ElementState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::rect_helpers::*;

mod todo_body;
mod todo_footer;
mod todo_input;

pub mod ui_stage {
    pub const USER_EVENTS: &str = "user_events";
    pub const DOMAIN_EVENTS: &str = "domain_events";
}

pub mod colors {
    use bevy::prelude::Color;

    const GRAY_1: Color = Color::rgb(0.95, 0.95, 0.95);
    const GRAY_3: Color = Color::rgb(0.6, 0.6, 0.6);
    const _GRAY_8: Color = Color::rgb(0.1, 0.1, 0.1);
    const GRAY_9: Color = Color::rgb(0.05, 0.05, 0.05);

    pub const PAGE_BACKGROUND: Color = GRAY_1;
    pub const HEADER_RED: Color =
        Color::rgba(175f32 / 255f32, 47f32 / 255f32, 47f32 / 255f32, 0.45);
    pub const TEXT_MUTED: Color = GRAY_3;
    pub const TEXT: Color = GRAY_9;
    pub const WHITE: Color = Color::WHITE;
}

pub mod sizes {
    use bevy::prelude::Val;

    pub const ZERO: Val = Val::Px(0.0);

    pub const SPACER_XS: Val = Val::Px(5.0);
    pub const SPACER_SM: Val = Val::Px(10.0);
    pub const SPACER_MD: Val = Val::Px(20.0);
    pub const SPACER_LG: Val = Val::Px(40.0);
    pub const SPACER_XL: Val = Val::Px(80.0);
    pub const SPACER: Val = SPACER_MD;

    pub const APP_WIDTH: Val = Val::Px(600.0);

    pub const FONT_H1: f32 = 100.0;
    pub const FONT_LARGE: f32 = 24.0;
    pub const FONT_BODY: f32 = 16.0;
}

pub struct ColorMaterials {
    pub page_background: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,

    pub btn_normal: Handle<ColorMaterial>,
    pub btn_hovered: Handle<ColorMaterial>,
    pub btn_pressed: Handle<ColorMaterial>,
}

impl FromResources for ColorMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ColorMaterials {
            page_background: materials.add(colors::PAGE_BACKGROUND.into()),
            white: materials.add(colors::WHITE.into()),

            btn_normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            btn_hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            btn_pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

pub fn build(app: &mut AppBuilder) {
    app.add_event::<NodeClickEvent>()
        .add_event::<FocusEvent>()
        .add_event::<BlurEvent>()
        .add_stage_before(stage::UPDATE, ui_stage::USER_EVENTS)
        .add_stage_after(stage::UPDATE, ui_stage::DOMAIN_EVENTS)
        .init_resource::<ColorMaterials>()
        .init_resource::<Focus>()
        .init_resource::<FocusableClickedState>()
        .add_startup_system(setup_ui.system())
        .add_system(node_click_event_source.system())
        .add_system(focusable_click_system.system())
        .add_system(button_interaction_system.system());
    // .add_system(clear_click_focus_system.system());

    todo_input::build(app);
    todo_body::build(app);
    todo_footer::build(app);
}

#[derive(Debug, Clone)]
struct NodeClickEvent {
    clicked: Entity,
}

#[derive(Debug, Clone)]
pub struct FocusEvent {
    pub focused: Entity,
}

#[derive(Debug, Clone)]
pub struct BlurEvent {
    pub blurred: Entity,
}

struct Root;

pub struct TodoContainer;

pub struct Focusable {
    has_focus: bool,
}

impl Focusable {
    pub fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl Default for Focusable {
    fn default() -> Self {
        Focusable { has_focus: false }
    }
}

#[derive(Default)]
struct Focus {
    entity: Option<Entity>,
}

pub struct NodeContext<'a> {
    pub cmds: &'a mut Commands,
    pub asset_server: Res<'a, AssetServer>,
    pub fonts: ResMut<'a, Assets<Font>>,
    pub asset_materials: ResMut<'a, Assets<ColorMaterial>>,
    pub colors: Res<'a, ColorMaterials>,
    pub font: Handle<Font>,
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
) {
    let mut mouse_clicked = false;
    for e in click_state.mouse_click_reader.iter(&mouse_click_events) {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
            mouse_clicked = true
        }
    }

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
    let had_focus = focusable.has_focus();

    if is_focused != had_focus && is_focused {
        focus_events.send(FocusEvent { focused: entity });
    }

    if is_focused != had_focus && !is_focused {
        blur_events.send(BlurEvent { blurred: entity });
    }

    focusable.has_focus = is_focused;
}

fn button_interaction_system(
    button_materials: Res<ColorMaterials>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &mut Handle<ColorMaterial>)>,
) {
    for (_, interaction, mut material) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.btn_pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.btn_hovered;
            }
            Interaction::None => {
                *material = button_materials.btn_normal;
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<ColorMaterials>,
    fonts: ResMut<Assets<Font>>,
    asset_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server
        .load("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    let mut ctx = NodeContext {
        cmds: &mut commands,
        asset_server: asset_server,
        fonts: fonts,
        colors: materials,
        asset_materials: asset_materials,
        font: font,
    };

    // 2d camera
    ctx.cmds.spawn(UiCameraComponents::default());

    // root
    root_node(&mut ctx, |ctx| {
        vec![
            div_node(
                ctx,
                DivNode {
                    background: ctx.colors.page_background.into(),
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                |ctx| {
                    let heading = heading_node(
                        ctx,
                        TextNode {
                            text: "todos",
                            ..Default::default()
                        },
                    );

                    let container = div_node(
                        ctx,
                        DivNode {
                            margin: Some(Rect::top(sizes::SPACER)),
                            size: Some(Size::new(Val::Percent(100.0), Val::Auto)),
                            max_size: Some(Size::new(sizes::APP_WIDTH, Val::Auto)),
                            ..Default::default()
                        },
                        |ctx| vec![todo_input::spawn_todo_input_node(ctx)],
                    );
                    ctx.cmds.insert_one(container, TodoContainer);

                    vec![heading, container]
                },
            ),
            div_node(
                ctx,
                DivNode {
                    background: ctx.colors.page_background.into(),
                    ..Default::default()
                },
                |ctx| {
                    vec![
                        text_node(
                            ctx,
                            TextNode {
                                text: "Double-click to edit a todo",
                                color: Some(colors::TEXT_MUTED),
                                ..Default::default()
                            },
                        ),
                        text_node(
                            ctx,
                            TextNode {
                                text: "Made with bevy_ui",
                                color: Some(colors::TEXT_MUTED),
                                ..Default::default()
                            },
                        ),
                    ]
                },
            ),
        ]
    });
}

fn heading_node(ctx: &mut NodeContext, mut node: TextNode) -> Entity {
    node.font_size = node.font_size.or(Some(sizes::FONT_H1));
    node.color = node.color.or(Some(colors::HEADER_RED));
    text_node(ctx, node)
}

#[derive(Default)]
pub struct DivNode {
    pub background: Background,
    pub size: Option<Size<Val>>,
    pub min_size: Option<Size<Val>>,
    pub max_size: Option<Size<Val>>,
    pub align_items: Option<AlignItems>,
    pub padding: Option<Rect<Val>>,
    pub margin: Option<Rect<Val>>,
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
}

pub enum Background {
    Color(Color),
    Material(Handle<ColorMaterial>),
}

impl From<Handle<ColorMaterial>> for Background {
    fn from(m: Handle<ColorMaterial>) -> Background {
        Background::Material(m)
    }
}

impl Default for Background {
    fn default() -> Self {
        return Background::Color(Color::default());
    }
}

impl Background {
    fn get_material(&self, ctx: &mut NodeContext) -> Handle<ColorMaterial> {
        match self {
            Background::Color(c) => ctx.asset_materials.add((*c).into()),
            Background::Material(m) => *m,
        }
    }
}

fn div_node(
    ctx: &mut NodeContext,
    node: DivNode,
    mut children: impl FnMut(&mut NodeContext) -> Vec<Entity>,
) -> Entity {
    ctx.spawn_node(|e, ctx| {
        let bundle = NodeComponents {
            style: Style {
                flex_direction: node.flex_direction.unwrap_or(FlexDirection::ColumnReverse),
                align_items: node.align_items.unwrap_or_default(),
                padding: node.padding.unwrap_or_default(),
                margin: node.margin.unwrap_or_default(),
                size: node.size.unwrap_or(Size::new(Val::Auto, Val::Auto)),
                min_size: node.min_size.unwrap_or(Size::new(Val::Auto, Val::Auto)),
                max_size: node.max_size.unwrap_or(Size::new(Val::Auto, Val::Auto)),
                justify_content: node.justify_content.unwrap_or_default(),
                ..Default::default()
            },
            material: node.background.get_material(ctx),
            ..Default::default()
        };

        let children = children(ctx);
        ctx.cmds
            .spawn_as_entity(e, bundle)
            .push_children(e, &children);
    })
}

#[derive(Default, Clone)]
pub struct TextNode<'a> {
    pub text: &'a str,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
    pub padding: Option<Rect<Val>>,
    pub margin: Option<Rect<Val>>,
}

pub fn text_node(ctx: &mut NodeContext, node: TextNode) -> Entity {
    ctx.spawn_node(|e, ctx| {
        let bundle = TextComponents {
            style: Style {
                align_self: AlignSelf::Center,
                padding: node.padding.unwrap_or_default(),
                margin: node.margin.unwrap_or_default(),
                ..Default::default()
            },
            text: Text {
                value: node.text.to_string(),
                font: ctx.font,
                style: TextStyle {
                    font_size: node.font_size.unwrap_or(sizes::FONT_BODY),
                    color: node.color.unwrap_or(colors::TEXT),
                },
            },
            ..Default::default()
        };

        ctx.cmds.spawn_as_entity(e, bundle);
    })
}

fn root_node(ctx: &mut NodeContext, children: impl Fn(&mut NodeContext) -> Vec<Entity>) -> Entity {
    ctx.spawn_node(|e, ctx| {
        let bundle = NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::xy(sizes::SPACER_XL, sizes::SPACER_LG),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: ctx.colors.page_background,
            ..Default::default()
        };

        let children = children(ctx);

        ctx.cmds
            .spawn_as_entity(e, bundle)
            .with(Root)
            .push_children(e, &children);
    })
}

impl NodeContext<'_> {
    fn spawn_node(&mut self, mut s: impl FnMut(Entity, &mut NodeContext)) -> Entity {
        let e = Entity::new();
        s(e, self);
        return e;
    }
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
