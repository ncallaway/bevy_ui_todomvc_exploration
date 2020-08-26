use super::*;

pub fn heading_node(ctx: &mut NodeContext, mut node: TextNode) -> Entity {
    node.font_size = node.font_size.or(Some(sizes::FONT_H1));
    node.color = node.color.or(Some(colors::HEADER_RED));
    text_node(ctx, node)
}

pub struct ButtonBehavior {
    pub normal: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
    pub hover: Handle<ColorMaterial>,
    pub active: Option<Handle<ColorMaterial>>,

    pub is_active: bool,
}

pub fn text_button_node(ctx: &mut NodeContext, node: TextButtonNode) -> Entity {
    ctx.spawn_node(|e, ctx| {
        let bundle = ButtonComponents {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                padding: node
                    .padding
                    .unwrap_or(Rect::xy(sizes::SPACER, sizes::SPACER_XS)),
                // center the label
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: node.margin.unwrap_or_default(),
                ..Default::default()
            },
            material: node.color_normal.unwrap_or(ctx.colors.btn_dark),
            ..Default::default()
        };

        let child = text_node(ctx, node.label.clone());

        ctx.cmds
            .spawn_as_entity(e, bundle)
            .push_children(e, &[child])
            .insert_one(
                e,
                ButtonBehavior {
                    normal: node.color_normal.unwrap_or(ctx.colors.btn_dark),
                    hover: node.color_hover.unwrap_or(ctx.colors.btn_dark_hovered),
                    pressed: node.color_pressed.unwrap_or(ctx.colors.btn_dark_pressed),
                    active: node.color_active,

                    is_active: false,
                },
            );
    })
}

pub fn div_node(
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
                flex_grow: node.flex_grow.unwrap_or(0.0),
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

pub fn text_node(ctx: &mut NodeContext, node: TextNode) -> Entity {
    ctx.spawn_node(|e, ctx| {
        let bundle = TextComponents {
            style: Style {
                align_self: AlignSelf::Center,
                padding: node.padding.unwrap_or_default(),
                margin: node.margin.unwrap_or_default(),
                // flex_grow: node.flex_grow.unwrap_or(0.0),
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

#[derive(Default, Clone)]
pub struct TextNode<'a> {
    pub text: &'a str,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
    pub padding: Option<Rect<Val>>,
    pub margin: Option<Rect<Val>>,
    pub flex_grow: Option<f32>,
}

#[derive(Default, Clone)]
pub struct TextButtonNode<'a> {
    pub label: TextNode<'a>,
    pub margin: Option<Rect<Val>>,
    pub padding: Option<Rect<Val>>,
    pub flex_grow: Option<f32>,

    pub color_normal: Option<Handle<ColorMaterial>>,
    pub color_pressed: Option<Handle<ColorMaterial>>,
    pub color_hover: Option<Handle<ColorMaterial>>,
    pub color_active: Option<Handle<ColorMaterial>>,
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
    pub flex_grow: Option<f32>,
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
