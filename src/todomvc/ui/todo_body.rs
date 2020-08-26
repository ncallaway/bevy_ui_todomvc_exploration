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
    //   todo_input::build(app);

    //   app.add_event::<NodeClickEvent>()
    //     .add_event::<FocusEvent>()
    //     .add_event::<BlurEvent>()
    //     .init_resource::<ButtonMaterials>()
    //     .init_resource::<Focus>()
    //     .init_resource::<FocusableClickedState>()
    //     .add_startup_system(setup_ui.system())
    //     .add_system(node_click_event_source.system())
    //     .add_system(focusable_click_system.system())
    //     .add_system(button_interaction_system.system())
    // // .add_system(clear_click_focus_system.system());
    // ;
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
        println!("a todo was added");
        if prior.0 == false {
            println!("THIS IS THE FIRST TODO, WE SHOULD SPAWN THE BODY!");
            prior.0 = true;
            for (parent, _) in &mut container_query.iter() {
                println!("Found a home, spawning the thing");
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
        |ctx| vec![todo_footer::spawn_todo_footer(ctx)],
    )
}
