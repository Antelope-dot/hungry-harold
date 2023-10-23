use bevy::prelude::*;
use rand::prelude::*;

mod animation;
use animation::*;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Hunger {
    value: i32,
}
#[derive(Resource)]
struct HungerTimer(Timer);

#[derive(Resource)]
struct AppleTimer(Timer);

#[derive(Component)]
struct Apple;

const RADIUS: f32 = 400. / (2. * std::f32::consts::PI);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AnimationPlugin)
        .insert_resource(HungerTimer(Timer::from_seconds(5., TimerMode::Repeating)))
        .insert_resource(AppleTimer(Timer::from_seconds(10., TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(hunger_system)
        .add_system(player_movement)
        .add_system(apple_spawner)
        .add_system(collission_system)
        .insert_resource(ClearColor(Color::rgb(0.44, 0.33, 0.23)))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Snake.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        Player{}, 
        Hunger{value: 0}
    ));
}

fn hunger_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<HungerTimer>,
    mut query: Query<(Entity, &mut Hunger), With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (entity, mut hunger )in query.iter_mut() {
            hunger.value += 1;
            println!("Current hunger is: {}", hunger.value);
            if hunger.value >= 10 {
                println!("Oh no! You Died :(");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn apple_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AppleTimer>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    if timer.0.tick(time.delta()).just_finished() {
        let x: f32 = rng.gen_range(-330.0..330.0);
        let y: f32 = rng.gen_range(-625.0..625.0);
        println!("x: {}", x);
        println!("y: {}", y);

        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("apple.png"),
                transform: Transform::from_xyz(x,y,0.).with_scale(Vec3::new(3.,3.,3.)),
                ..default()
            },
            Apple{}
        ));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
        if let Ok(mut transform) = player_query.get_single_mut() {
            if keyboard_input.pressed(KeyCode::Left) {
                transform.translation.x -= 150. * time.delta_seconds();
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI); //ANGLE IS IN RADIANS NOT DEGREES!!!
            }
            if keyboard_input.pressed(KeyCode::Right) {
                transform.translation.x += 150. * time.delta_seconds();
                transform.rotation = Quat::from_rotation_y(0.);
            }
            if keyboard_input.pressed(KeyCode::Up) {
                transform.translation.y += 150. * time.delta_seconds();

            }
            if keyboard_input.pressed(KeyCode::Down) {
                transform.translation.y -= 150. * time.delta_seconds();

            }
            if keyboard_input.pressed(KeyCode::Space) {
                transform.scale *= 1.1;
            }

        }
}

fn collission_system(
    mut commands: Commands,
    mut player_query: Query<( &mut Hunger,&Transform), With<Player>>,
    mut apple_query: Query<(Entity, &Transform), With<Apple>>,

) {
    
    for (mut hunger, player_pos )in player_query.iter_mut() {
        for (apple, apple_pos )in apple_query.iter_mut() {
            // LETS GET THAT HYPOTENUSE
            let distance = (f32::powf(apple_pos.translation.x - player_pos.translation.x, 2.) + 
            f32::powf(apple_pos.translation.y - player_pos.translation.y, 2.)).sqrt();
            if distance < RADIUS {
                hunger.value -= 1;
                println!("OM NOM NOM: {}", hunger.value);
                commands.entity(apple).despawn();
                
            }
        }
    }
}