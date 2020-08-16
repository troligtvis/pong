use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{self, thread_rng, Rng};

// due to not being able to access windows from a `startup_system`
// fixed values will be needed for screen size during startup
// https://github.com/bevyengine/bevy/issues/175
const SCR_WIDTH: f32 = 800.0; 
const SCR_HEIGHT: f32 = 600.0;

const PADDING: f32 = 40.0;
const PADDLE_HEIGHT: f32 = 120.0;
const PADDLE_WIDTH: f32 = 12.0;

fn randomize_vec(vec: &mut Vec3, x: f32, y: f32) {
    let mut rng = thread_rng();
    *vec.x_mut() = if rng.gen_bool(0.5) { x } else { -x };
    *vec.y_mut() = if rng.gen_bool(0.5) { y } else { -y };
}

struct KeyboardControls {
    up: KeyCode,
    down: KeyCode,
}

enum Collider {
    Solid, 
    Scorable,
}

struct Ball {
    velocity: Vec3,
}

struct Paddle {
    speed: f32,
    controls: KeyboardControls, 
}

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    _asset_server: Res<AssetServer>
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    // Player 1
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(-SCR_WIDTH / 2.0 + PADDING, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            },
            ..Default::default()
        })
        .with(Paddle {
            speed: 500.0,
            controls: KeyboardControls { up: KeyCode::W, down: KeyCode::S }
        })
        .with(Collider::Solid);

    // Player 2
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            translation: Translation(Vec3::new(SCR_WIDTH / 2.0 - PADDING, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            },
            ..Default::default()
        })
        .with(Paddle {
            speed: 500.0,
            controls: KeyboardControls { up: KeyCode::Up, down: KeyCode::Down }
        })
        .with(Collider::Solid);

    // Ball 

    let mut velocity = Vec3::zero();
    randomize_vec(&mut velocity, 200.0, 200.0);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(20.0, 20.0),
            },
            ..Default::default()
        })
        .with(Ball {
            velocity,
        })
        .with(Collider::Solid);

    // Walls
    let wall_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(SCR_WIDTH, SCR_HEIGHT);

    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Scorable)
        // right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Scorable)
        // bottom
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // top
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid);
}

fn paddle_movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&Paddle, &mut Translation)>
) {

    let scr_height: f32 = if let Some(window) = windows.get_primary() {
        window.height as f32
    } else {
        SCR_HEIGHT
    };

    for (paddle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(paddle.controls.up) {
            direction += 1.0;
        }

        if keyboard_input.pressed(paddle.controls.down) {
            direction -= 1.0;
        }

        *translation.0.y_mut() += time.delta_seconds * direction * paddle.speed;

        // bound the paddle to screen
        *translation.0.y_mut() = f32::max(
            -scr_height / 2.0 + PADDLE_HEIGHT / 2.0, 
            f32::min(scr_height / 2.0 - PADDLE_HEIGHT / 2.01, translation.0.y())
        );
    }
}

fn ball_movement_system(
    time: Res<Time>,
    _windows: Res<Windows>,
    mut query: Query<(&Ball, &mut Translation)>
) {
    for (ball, mut translation) in &mut query.iter() {
        translation.0 += ball.velocity * time.delta_seconds; 
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &mut Translation, &Sprite)>,
    mut collider_query: Query<(Entity, &Collider, Without<Ball, &Translation>, &Sprite)>,
) {
    for (mut ball, mut ball_translation, sprite) in &mut ball_query.iter() {
        let ball_size = sprite.size;
        let mut velocity = &mut ball.velocity;

        // check collision with walls
        for (_collider_entity, collider, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
            if let Some(collision) = collision {
                if let &Collider::Scorable = collider {
                    ball_translation.0 = Vec3::zero();
                    randomize_vec(&mut velocity, 200.0, 200.0);
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = velocity.x() > 0.0,
                    Collision::Right => reflect_x = velocity.x() < 0.0,
                    Collision::Top => reflect_y = velocity.y() < 0.0,
                    Collision::Bottom => reflect_y = velocity.y() > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    *velocity.x_mut() = -velocity.x();
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    *velocity.y_mut() = -velocity.y();
                }

                break;
            }
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy Pong".to_string(),
            width: SCR_WIDTH as u32,
            height: SCR_HEIGHT as u32,
            vsync: true,
        })
        .add_default_plugins()
        .add_startup_system(setup.system()) 
        .add_system(paddle_movement_system.system())
        .add_system(ball_movement_system.system())
        .add_system(ball_collision_system.system())
        .run();
}