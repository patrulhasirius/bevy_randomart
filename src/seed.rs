use bevy_simple_text_input::{
    TextInput, TextInputPlugin, TextInputSubmitEvent, TextInputSystem, TextInputTextColor,
    TextInputTextFont,
};

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[derive(Resource)]
pub struct Seed(pub u64);

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Seed(rand::random()))
            .add_plugins(TextInputPlugin)
            .add_systems(Update, reset_seed.run_if(input_just_pressed(KeyCode::KeyR)))
            .add_systems(
                Update,
                spawn_text_box.run_if(input_just_pressed(KeyCode::KeyS)),
            )
            .add_systems(Update, listener.after(TextInputSystem));
    }
}

fn reset_seed(mut seed: ResMut<Seed>) {
    seed.0 = rand::random();
}

fn spawn_text_box(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 34.,
                    ..default()
                }),
                TextInputTextColor(TextColor(TEXT_COLOR)),
            ));
        });
}

fn listener(
    mut commands: Commands,
    mut events: EventReader<TextInputSubmitEvent>,
    query: Query<Entity, With<Node>>,
    mut seed: ResMut<Seed>,
) {
    for event in events.read() {
        if let Ok(number) = event.value.parse::<u64>() {
            seed.0 = number;
            query.iter().for_each(|entity| {
                commands.entity(entity).despawn_recursive();
            });
        }
    }
}
