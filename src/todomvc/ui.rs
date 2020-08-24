use bevy::prelude::*;

// #F5F5F5
mod colors {
    use bevy::prelude::Color;

    const OFF_WHITE: Color = Color::rgb(0.96, 0.96, 0.96);

    pub const PAGE_BACKGROUND: Color = OFF_WHITE;
    pub const HEADER_RED: Color =
        Color::rgba(175f32 / 255f32, 47f32 / 255f32, 47f32 / 255f32, 0.45);
}

mod sizes {
    use bevy::prelude::Val;

    pub const SPACER_XS: Val = Val::Px(5f32);
    pub const SPACER_SM: Val = Val::Px(10f32);
    pub const SPACER_MD: Val = Val::Px(20f32);
    pub const SPACER_LG: Val = Val::Px(40f32);
    pub const SPACER_XL: Val = Val::Px(80f32);
    pub const SPACER: Val = SPACER_MD;
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
    app.init_resource::<ButtonMaterials>()
        .add_startup_system(setup_ui.system());
}

trait Wanted {
    fn all(s: Val) -> Rect<Val>;
    fn x(x: Val) -> Rect<Val>;
    fn y(y: Val) -> Rect<Val>;
    fn xy(x: Val, y: Val) -> Rect<Val>;
}

impl Wanted for Rect<Val> {
    fn all(s: Val) -> Rect<Val> {
        Rect {
            left: s,
            right: s,
            top: s,
            bottom: s,
        }
    }

    fn x(x: Val) -> Rect<Val> {
        Rect {
            left: x,
            right: x,
            top: Val::Px(0f32),
            bottom: Val::Px(0f32),
        }
    }

    fn y(y: Val) -> Rect<Val> {
        Rect {
            left: Val::Px(0f32),
            right: Val::Px(0f32),
            top: y,
            bottom: y,
        }
    }

    fn xy(x: Val, y: Val) -> Rect<Val> {
        Rect {
            left: x,
            right: x,
            top: y,
            bottom: y,
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
    let font_handle = asset_server
        .load("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // root sidebar
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::xy(sizes::SPACER_XL, sizes::SPACER_LG),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: materials.add(colors::PAGE_BACKGROUND.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // button
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(45.0)),
                        // center button
                        margin: Rect::all(Val::Px(0.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    // button label
                    parent.spawn(TextComponents {
                        text: Text {
                            value: "Send a message".to_string(),
                            font: font_handle,
                            style: TextStyle {
                                font_size: 12.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });
                })
                // todos container
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    material: materials.add(colors::PAGE_BACKGROUND.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            // material: materials.add(UI_BACKGROUND.into()),
                            ..Default::default()
                        })
                        // todos header
                        .spawn(TextComponents {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text {
                                value: "todos ".to_string(),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 100.0,
                                    color: colors::HEADER_RED,
                                },
                            },
                            ..Default::default()
                        });
                });
        });
}

fn root_node(button_materials: Res<ButtonMaterials>) -> ButtonComponents {
    ButtonComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(45.0)),
            // center button
            margin: Rect::all(Val::Px(0.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal,
        ..Default::default()
    }
}
