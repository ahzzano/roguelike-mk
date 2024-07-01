use bevy::{prelude::*, window::PrimaryWindow};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub struct Board;

#[derive(Component)]
pub struct Wall {}

const TILE_WIDTH: f32 = 64.0;
const TILE_HEIGHT: f32 = 64.0;

impl Plugin for Board {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_board);
    }
}

fn create_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = File::open("assets/level.txt").expect("No map found");
    let window = window_query.get_single().unwrap();

    let initial_x = 0.0;
    let initial_y = window.height();

    let mut fx = initial_x;
    let mut fy = initial_y;

    let mut lines: Vec<Result<String, Error>> = BufReader::new(layout).lines().collect();

    let mut walls: Vec<Entity> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            for ch in line.chars() {
                match ch {
                    '#' => {
                        let wall = commands
                            .spawn((
                                SpriteBundle {
                                    transform: Transform::from_xyz(fx, fy, 0.0),
                                    texture: asset_server.load("wall.png"),
                                    ..default()
                                },
                                Wall {},
                            ))
                            .id();

                        walls.push(wall);
                    }
                    _ => (),
                }
                fx += TILE_WIDTH;
            }
        }

        fy -= TILE_HEIGHT;
        fx = initial_x;
    }

    commands
        .spawn(())
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Walls"))
        .push_children(&walls);

    // print!("{}", level_layout.clone());
}
