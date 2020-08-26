use bevy::input::keyboard::ElementState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::rect_helpers::*;

mod common_nodes;
mod todo_body;
mod todo_footer;
mod todo_input;
mod todo_list;
mod todo_row;

pub use common_nodes::*;

pub mod ui_stage {
    pub const USER_EVENTS: &str = "user_events";
    pub const DOMAIN_EVENTS: &str = "domain_events";
    pub const VISUAL_SYNC: &str = "visual_sync";
}

pub mod colors {
    use bevy::prelude::Color;

    const GRAY_1: Color = Color::rgb(0.95, 0.95, 0.95);
    const GRAY_2: Color = Color::rgb(0.85, 0.85, 0.85);
    const GRAY_3: Color = Color::rgb(0.75, 0.75, 0.75);
    const _GRAY_4: Color = Color::rgb(0.65, 0.65, 0.65);
    const GRAY_5: Color = Color::rgb(0.55, 0.55, 0.55);
    const _GRAY_6: Color = Color::rgb(0.40, 0.40, 0.40);
    const GRAY_7: Color = Color::rgb(0.25, 0.25, 0.25);
    const GRAY_8: Color = Color::rgb(0.15, 0.15, 0.15);
    const GRAY_9: Color = Color::rgb(0.05, 0.05, 0.05);

    const PRESSED_RED: Color = Color::rgb(253.0 / 255.0, 160.0 / 255.0, 160.0 / 255.0);
    const FADED_RED: Color = Color::rgb(253.0 / 255.0, 190.0 / 255.0, 190.0 / 255.0);
    const LIGHT_RED: Color = Color::rgb(253.0 / 255.0, 235.0 / 255.0, 235.0 / 255.0);

    pub const PAGE_BACKGROUND: Color = GRAY_1;
    pub const BTN_DARK: Color = GRAY_9;
    pub const BTN_DARK_HOVER: Color = GRAY_8;
    pub const BTN_DARK_PRESSED: Color = GRAY_7;
    pub const BTN_LIGHT: Color = GRAY_1;
    pub const BTN_LIGHT_HOVER: Color = GRAY_2;
    pub const BTN_LIGHT_PRESSED: Color = GRAY_3;
    pub const BACKGROUND_ACTIVE_RED: Color = FADED_RED;
    pub const BACKGROUND_HOVER_RED: Color = LIGHT_RED;
    pub const BACKGROUND_PRESSED_RED: Color = PRESSED_RED;
    pub const HEADER_RED: Color = FADED_RED;
    pub const TEXT_MUTED: Color = GRAY_5;
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

    pub const APP_WIDTH: Val = Val::Px(900.0);

    pub const FONT_H1: f32 = 100.0;
    pub const FONT_LARGE: f32 = 24.0;
    pub const FONT_BODY: f32 = 16.0;
}

pub struct ColorMaterials {
    pub page_background: Handle<ColorMaterial>,
    pub background_active_red: Handle<ColorMaterial>,
    pub background_hover_red: Handle<ColorMaterial>,
    pub background_pressed_red: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,

    pub btn_dark: Handle<ColorMaterial>,
    pub btn_dark_hovered: Handle<ColorMaterial>,
    pub btn_dark_pressed: Handle<ColorMaterial>,

    pub btn_light: Handle<ColorMaterial>,
    pub btn_light_hovered: Handle<ColorMaterial>,
    pub btn_light_pressed: Handle<ColorMaterial>,
}

impl FromResources for ColorMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ColorMaterials {
            page_background: materials.add(colors::PAGE_BACKGROUND.into()),
            background_active_red: materials.add(colors::BACKGROUND_ACTIVE_RED.into()),
            background_hover_red: materials.add(colors::BACKGROUND_HOVER_RED.into()),
            background_pressed_red: materials.add(colors::BACKGROUND_PRESSED_RED.into()),
            white: materials.add(colors::WHITE.into()),

            btn_dark: materials.add(colors::BTN_DARK.into()),
            btn_dark_hovered: materials.add(colors::BTN_DARK_HOVER.into()),
            btn_dark_pressed: materials.add(colors::BTN_DARK_PRESSED.into()),

            btn_light: materials.add(colors::BTN_LIGHT.into()),
            btn_light_hovered: materials.add(colors::BTN_LIGHT_HOVER.into()),
            btn_light_pressed: materials.add(colors::BTN_LIGHT_PRESSED.into()),
        }
    }
}

pub fn build(app: &mut AppBuilder) {
    app.add_event::<NodeClickEvent>()
        .add_event::<FocusEvent>()
        .add_event::<BlurEvent>()
        .add_stage_before(stage::UPDATE, ui_stage::USER_EVENTS)
        .add_stage_after(stage::UPDATE, ui_stage::DOMAIN_EVENTS)
        .add_stage_after(ui_stage::DOMAIN_EVENTS, ui_stage::VISUAL_SYNC)
        .init_resource::<ColorMaterials>()
        .init_resource::<Focus>()
        .init_resource::<FocusableClickedState>()
        .add_startup_system(setup_ui.system())
        .add_system(node_click_event_source.system())
        .add_system(focusable_click_system.system())
        .add_system_to_stage(ui_stage::VISUAL_SYNC, button_interaction_system.system());
    // .add_system(clear_click_focus_system.system());

    todo_input::build(app);
    todo_body::build(app);
    todo_list::build(app);
    todo_row::build(app);
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
    mut interaction_query: Query<(
        &Button,
        &ButtonBehavior,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
    )>,

    mut active_query: Query<(
        &Button,
        Mutated<ButtonBehavior>,
        &Interaction,
        &mut Handle<ColorMaterial>,
    )>,
) {
    for (_, b, interaction, material) in &mut interaction_query.iter() {
        style_button(b, &(*interaction), material);
    }

    // todo: this isn't great, fix this
    for (_, b, interaction, material) in &mut active_query.iter() {
        style_button(&(*b), interaction, material);
    }
}

fn style_button(b: &ButtonBehavior, i: &Interaction, mut material: Mut<Handle<ColorMaterial>>) {
    *material = if b.is_active {
        b.active.unwrap_or(b.normal)
    } else {
        match *i {
            Interaction::Clicked => b.pressed,
            Interaction::Hovered => b.hover,
            Interaction::None => b.normal,
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
