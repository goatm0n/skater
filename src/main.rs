use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Goatman".to_string())));
    commands.spawn((Person, Name("Stabby Joe".to_string())));
    commands.spawn((Person, Name("Niko".to_string())));

}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GreetTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating)
        ))
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
    .run();
}