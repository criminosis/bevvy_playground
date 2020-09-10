//from https://github.com/bevyengine/bevy/blob/master/examples/game/breakout.rs

/*
Things to try out
  - +/- to make the ball go faster/slower (interesting to see if having to how holding down shift is represented if at all
  - A restart button
  - "You win" after all bars are broken
     - A continue screen which then starts with more bars / faster ball scaling
  - "You lose" screen when the ball hits the bottom
    ^ or alternatively hitting the backboard decrements the score and respawns a bar (hitting the backboard while at 0 causes the lose screen)
  - Make the ball go faster when it breaks a bar
  - Make a sound play when a bar is broken
  - A start screen for when the game starts instead of immediately starting as soon as launched
  - Pressing a button spawns another ball
  - Replace the collide method to using bevy_rapier (https://github.com/dimforge/bevy_rapier / https://rapier.rs/docs/) a physics plugin
      - https://rapier.rs/docs/user_guides/rust_bevy_plugin/getting_started
*/

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

/// An implementation of the classic game "Breakout"
pub fn run() {
    App::build()
        //This does a lot of stuff, setting up sprite & UI rendering among many other things
        //https://docs.rs/bevy/0.1.3/src/bevy/add_default_plugins.rs.html#7-37
        //UI Plugin setup: https://docs.rs/bevy_ui/0.1.3/src/bevy_ui/lib.rs.html#39-55
        //Sprite Setup: https://docs.rs/bevy_sprite/0.1.3/src/bevy_sprite/lib.rs.html#43-67
        //Sprite System: https://docs.rs/bevy_sprite/0.1.3/src/bevy_sprite/sprite.rs.html#21-34
        //Sprites added to the Render graph here: https://docs.rs/bevy_sprite/0.1.3/src/bevy_sprite/lib.rs.html#55
        //The specifics of how the rendering plugin sets up rendering systems: https://docs.rs/bevy_render/0.1.3/src/bevy_render/lib.rs.html#80-179
        .add_default_plugins()
        //Scoreboard state
        .add_resource(Scoreboard { score: 0 })
        //Kind of a silvery color -- ClearColor resources are the background color of the window
        //https://github.com/jamadazi/bevy-cheatsheet/blob/master/bevy-cheatsheet.md#configuration-resources
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(ball_movement_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

struct Paddle {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

enum Collider {
    Solid,
    Scorable,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    //Looks like SpriteComponents describes the attributes that are used by the
    //rendering layer describing its material and sprite dimension and its translation
    commands
        // cameras
        //Looks like this creates a camera with a draw distance of near 1000 units on the z axis centralized around 0,0
        //by default Camera2dComponents uses the center of the Window as its 0,0 and its viewport is the size of the window
        //
        //https://docs.rs/bevy_render/0.1.3/src/bevy_render/entity.rs.html#76
        .spawn(Camera2dComponents::default())
        //Looks like this creates the default UI Camera settings per:
        //https://docs.rs/bevy_ui/0.1.3/src/bevy_ui/entity.rs.html#204
        //The UiCameraComponent's default seems very similar to the Camera2dComponents but it changes the projection
        //to be based on the bottom left: https://docs.rs/bevy_ui/0.1.3/src/bevy_ui/entity.rs.html#216
        //Guessing this was done in order to make it easier to understand UI placement as the coordinates would be all positive
        //Instead of stuff like bottom left being -x_width/2, -y_width/2 when being assigned an hp bar, etc
        //TextComponent: https://docs.rs/bevy_ui/0.1.3/src/bevy_ui/entity.rs.html#114
        //ImageComponent: https://docs.rs/bevy_ui/0.1.3/src/bevy_ui/entity.rs.html#114
        .spawn(UiCameraComponents::default())
        // paddle
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(120.0, 30.0),
            },
            //Looks like this is inserting the default values for SpriteComponents that wasn't set
            ..Default::default()
        })
        .with(Paddle { speed: 500.0 })
        .with(Collider::Solid)
        // ball
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            translation: Translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite {
                size: Vec2::new(30.0, 30.0),
            },
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        })
        // scoreboard
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

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
        .with(Collider::Solid)
        // right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
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

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x() + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x()) / 2.0, 100.0, 0.0);

    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y() + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x() + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                // brick
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                    sprite: Sprite { size: brick_size },
                    translation: Translation(brick_position),
                    ..Default::default()
                })
                .with(Collider::Scorable);
        }
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Translation)>,
) {
    for (paddle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        *translation.0.x_mut() += time.delta_seconds * direction * paddle.speed;

        // bound the paddle within the walls
        *translation.0.x_mut() = f32::max(-380.0, f32::min(380.0, translation.0.x()));
    }
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Translation)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (ball, mut translation) in &mut ball_query.iter() {
        translation.0 += ball.velocity * delta_seconds;
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

//So take in the scoreboard resource (so we can increment the score if needed)
//query for the ball (though if we don't care to do anything on a batch of balls, why query instead of pass in?)
//then do a query for all the colliders in the game and check for collisions
fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Translation, &Sprite)>,
    mut collider_query: Query<(Entity, &Collider, &Translation, &Sprite)>,
) {
    for (mut ball, ball_translation, sprite) in &mut ball_query.iter() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (collider_entity, collider, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    scoreboard.score += 1;
                    commands.despawn(collider_entity);
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
