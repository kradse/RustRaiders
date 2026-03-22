use bevy::prelude::*;

#[derive(Component)]
pub struct Toolbox {
    pub active: ToolKind,
}

pub struct ToolboxPlugin;
impl Plugin for ToolboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_toolbox);
        app.add_systems(Update, change_tool);
    }
}

fn init_toolbox(
    mut commands: Commands,
) {
    commands.spawn(
        Toolbox {
            active: ToolKind::None
        }
    );
}

fn change_tool(
    mut query: Query<&mut Toolbox>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut toolbox) = query.single_mut() else
        { return; };

    if keyboard_input.just_pressed(KeyCode::Escape) {
        toolbox.active = ToolKind::None;
    }
    if keyboard_input.just_pressed(KeyCode::F1) {
        toolbox.active = ToolKind::Selection;
    }
}

#[derive(PartialEq)]
pub enum ToolKind {
    None,
    Selection,
}