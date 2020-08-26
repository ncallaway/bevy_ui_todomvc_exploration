use bevy::prelude::*;

use super::*;
use crate::todomvc::domain::Todo;
// structure
// todo_body [
//   todo_list
//   todo_footer
// ]

// todo_list [
//    todo_row
// ]

pub fn build(app: &mut AppBuilder) {
    app.add_resource(PriorTodos(false))
        .add_system_to_stage(ui_stage::DOMAIN_EVENTS, on_todo_added.system());
}

struct PriorTodos(bool);

fn on_todo_added(
    mut commands: Commands,
    mut prior: ResMut<PriorTodos>,
    asset_server: Res<AssetServer>,
    fonts: ResMut<Assets<Font>>,
    materials: Res<ColorMaterials>,
    asset_materials: ResMut<Assets<ColorMaterial>>,
    mut added_query: Query<Added<Todo>>,
    mut any_query: Query<&Todo>,
    mut container_query: Query<(Entity, &TodoContainer)>,
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

    for _ in &mut added_query.iter() {
        if prior.0 == false {
            prior.0 = true;
            for (parent, _) in &mut container_query.iter() {
                let e = spawn_todo_body(&mut ctx);
                ctx.cmds.push_children(parent, &[e]);
                break;
            }
        }
    }

    if prior.0 {
        let mut any = false;
        for _ in &mut any_query.iter() {
            any = true;
            break;
        }

        if !any {
            println!("there are no remaining todos!!!! GOOD JOB!");
            prior.0 = false
        }
    }
}

fn spawn_todo_body(ctx: &mut NodeContext) -> Entity {
    div_node(
        ctx,
        DivNode {
            ..Default::default()
        },
        |ctx| {
            vec![
                todo_list::spawn_todo_list(ctx),
                todo_footer::spawn_todo_footer(ctx),
            ]
        },
    )
}
