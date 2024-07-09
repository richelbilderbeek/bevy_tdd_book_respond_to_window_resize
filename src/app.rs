use bevy::prelude::*;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::window::WindowPlugin::default());
    }

    app.add_systems(Startup, (add_camera, add_text));
    app.add_systems(Update, respond_to_window_resize);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_text(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(String::new(), TextStyle { ..default() }),
        ..default()
    });
}

#[cfg(test)]
fn count_n_texts(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Text>();
    query.iter(app.world_mut()).len()
}

#[cfg(test)]
fn get_text_text(app: &mut App) -> String {
    assert_eq!(count_n_texts(app), 1);
    let mut query = app.world_mut().query::<&Text>();
    query.single_mut(app.world_mut()).sections[0].value.clone()
}

fn respond_to_window_resize(
    mut q: Query<&mut Text>,
    mut resize_reader: EventReader<bevy::window::WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_texts() {
        let mut app = App::new();
        assert_eq!(count_n_texts(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_text() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_texts(&mut app), 1);
    }

    #[test]
    fn test_text_at_start_is_empty() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_text_text(&mut app), "");
    }

    #[test]
    fn test_respond_to_window_resize() {
        let mut app = create_app();
        app.update();

        //Resize the window
        app.world_mut().send_event(bevy::window::WindowResized {
            window: Entity::PLACEHOLDER,
            width: 100.0,
            height: 100.0,
        });
        app.update();

        assert_ne!(get_text_text(&mut app), "");
    }
}
