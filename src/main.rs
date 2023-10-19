use bevy::prelude::*;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Name(String);

fn add_characters(mut commands: Commands) {
    commands.spawn((Character, Name("Goatman".to_string())));
    commands.spawn((Character, Name("Stabby Joe".to_string())));
    commands.spawn((Character, Name("Niko".to_string())));

}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_characters(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Character>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


fn main() {
    App::new()
        // resouces:
        .insert_resource(GreetTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating)
        ))
        
        // plugins:
        // most of bevys functionality is contained within DefaultPlugins
        .add_plugins(DefaultPlugins)

        // events:


        // systems to run at startup
        .add_systems(Startup, add_characters)

        // systems to run each frame
        .add_systems(Update, greet_characters)

    .run();
}
