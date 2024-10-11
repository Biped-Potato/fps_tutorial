use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_crosshair(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.get_single().unwrap();
    let crosshair_size = 2.0;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::solid_color(Color::srgb(0., 1., 0.)),
                style: Style {
                    width: Val::Px(crosshair_size),
                    height: Val::Px(crosshair_size),
                    left: Val::Px(window.width() / 2. - (crosshair_size / 2.)),
                    top: Val::Px(window.height() / 2. - (crosshair_size / 2.)),
                    ..default()
                },
                ..default()
            });
        });
}
