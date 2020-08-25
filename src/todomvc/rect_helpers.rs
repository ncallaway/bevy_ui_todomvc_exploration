use bevy::prelude::{Rect, Val};
pub trait RectHelpers {
    fn all(s: Val) -> Rect<Val>;
    fn x(x: Val) -> Rect<Val>;
    fn y(y: Val) -> Rect<Val>;
    fn xy(x: Val, y: Val) -> Rect<Val>;
    fn left(x: Val) -> Rect<Val>;
    fn right(x: Val) -> Rect<Val>;
    fn top(x: Val) -> Rect<Val>;
    fn bottom(x: Val) -> Rect<Val>;
}

impl RectHelpers for Rect<Val> {
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
            ..Default::default()
        }
    }

    fn y(y: Val) -> Rect<Val> {
        Rect {
            top: y,
            bottom: y,
            ..Default::default()
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

    fn left(l: Val) -> Rect<Val> {
        Rect {
            left: l,
            ..Default::default()
        }
    }
    fn right(r: Val) -> Rect<Val> {
        Rect {
            right: r,
            ..Default::default()
        }
    }

    fn top(t: Val) -> Rect<Val> {
        Rect {
            top: t,
            ..Default::default()
        }
    }

    fn bottom(b: Val) -> Rect<Val> {
        Rect {
            bottom: b,
            ..Default::default()
        }
    }
}
