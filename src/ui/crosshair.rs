use bevy::{
    color::Color,
    prelude::{BuildChildren, Commands, ImageBundle, NodeBundle, Query, Style, UiImage, Val, With},
    window::{PrimaryWindow, Window},
};

pub fn spawn(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.get_single().unwrap();
    let size = 2.0;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::solid_color(Color::srgb(0., 1., 0.)),
                style: Style {
                    width: Val::Px(size),
                    height: Val::Px(size),
                    left: Val::Px(window.width() / 2. - size / 2.),
                    top: Val::Px(window.height() / 2. - size / 2.),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
