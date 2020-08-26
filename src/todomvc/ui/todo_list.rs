use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::{Filter, Todo};

pub fn build(app: &mut AppBuilder) {
    app.add_system_to_stage(ui_stage::VISUAL_SYNC, sync_todo_system.system());
    // app.add_system(count_label_system.system())
    //     .add_system_to_stage(ui_stage::USER_EVENTS, on_filter_tab_button_click.system())
    //     .add_system(set_filter_tab_active_system.system());
}

fn sync_todo_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<ColorMaterials>,
    fonts: ResMut<Assets<Font>>,
    asset_materials: ResMut<Assets<ColorMaterial>>,
    mut todos: Query<(Entity, &Todo)>,
    mut lists: Query<(Entity, &TodoList)>,
    mut rows: Query<(Entity, &mut todo_row::TodoRow)>,
) {
    let font = asset_server
        .get_handle("assets/fonts/FiraSans-ExtraLight.ttf")
        .unwrap();

    let mut ctx = NodeContext {
        cmds: &mut commands,
        asset_server: asset_server,
        fonts: fonts,
        colors: materials,
        asset_materials: asset_materials,
        font: font,
    };

    let mut row_borrow = rows.iter();
    let mut row_iter = row_borrow.into_iter();

    // todo: apply the filter

    for (todo, _) in &mut todos.iter() {
        let has_row = row_iter.next();

        match has_row {
            Some((_, mut row)) => {
                row.0 = todo;
            }
            None => {
                for (list, _) in &mut lists.iter() {
                    let e = todo_row::spawn_todo_row(&mut ctx, todo);
                    ctx.cmds.push_children(list, &[e]);
                    break;
                }
            }
        }
    }

    // remove the remaining entities
    for (e, _) in row_iter {
        commands.despawn_recursive(e);
    }
}

// fn message_display_sync_system(
//   mut commands: Commands,
//   font_handle: Res<Handle<Font>>,
//   mut messages: Query<&Message>,
//   mut display_containers: Query<(Entity, &MessageContainer)>,
//   mut message_displays: Query<(Entity, &mut MessageDisplay, &mut Text)>,
// ) {
//   let mut display_borrow = message_displays.iter();
//   let mut display_iter = display_borrow.into_iter();

//   for msg in &mut messages.iter() {
//       let has_display = display_iter.next();

//       match has_display {
//           Some((_, mut display, mut text)) => {
//               // oh no! A string allocation in a system loop! That's not great. I don't care
//               // about the performance of this, but please don't copy this somewhere real.
//               let desired_text = format!("[{}] {}", msg.from, msg.message);
//               if text.value != desired_text || display.ordinal != msg.ordinal {
//                   text.value = format!("[{}] {}", msg.from, msg.message);
//                   display.ordinal = msg.ordinal;
//               }
//           }
//           None => spawn_message_display(
//               &mut commands,
//               *font_handle,
//               &mut display_containers,
//               format!("[{}] {}", msg.from, msg.message),
//               msg.ordinal,
//           ),
//       }
//   }

//   // remove the remaining entities
//   for (e, _, _) in display_iter {
//       commands.despawn(e);
//   }
// }

struct TodoList;

pub fn spawn_todo_list(ctx: &mut NodeContext) -> Entity {
    let e = div_node(
        ctx,
        DivNode {
            background: ctx.colors.white.into(),
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        |_| vec![],
    );

    ctx.cmds.insert_one(e, TodoList);

    e
}
