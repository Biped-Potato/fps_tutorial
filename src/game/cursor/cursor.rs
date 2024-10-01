use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Startup, init_cursor_properties);
    }
}
#[derive(Resource)]
pub struct Cursor {
    locked: bool,
}

fn init_cursor_properties(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
) {
    let mut window = window_query.get_single_mut().unwrap();
    cursor.invert_lock(&mut window);
}
impl Cursor {
    pub fn invert_lock(&mut self, window: &mut Mut<'_, Window>) {
        self.locked = !self.locked;
        window.cursor.visible = !self.locked;
        if self.locked {
            let window_width = window.width();
            let window_height = window.height();
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(window_width / 2., window_height / 2.)));
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}
impl Default for Cursor {
    fn default() -> Self {
        Self { locked: false }
    }
}
