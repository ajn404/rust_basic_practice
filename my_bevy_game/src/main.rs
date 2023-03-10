use bevy::prelude::*;

fn hello() {
    println!("hello world");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(add_people)
            .add_system(greet_people);
    }
}

//ECS
//Entities
//Components
//Systems

//实体，数据和行为

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }
#[derive(Resource)]
struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello{}", name.0);
        }
    }
}
