mod chess;

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::PrimaryWindow;
use hexx::shapes;
use hexx::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(20.0);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    pub selected: Hex,
    pub halfway: Hex,
    pub ring: Vec<Hex>,
    pub wedge: Vec<Hex>,
    pub dir_wedge: Vec<Hex>,
    pub line: Vec<Hex>,
    pub half_ring: Vec<Hex>,
    pub rotated: Vec<Hex>,
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<ColorMaterial>,
    black_material: Handle<ColorMaterial>,
    grey_material: Handle<ColorMaterial>,
    white_material: Handle<ColorMaterial>,
    origin_material: Handle<ColorMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // materials
    let selected_material = materials.add(Color::RED.into());
    let black_material = materials.add(Color::BLACK.into());
    let grey_material = materials.add(Color::GRAY.into());
    let white_material = materials.add(Color::WHITE.into());
    let origin_material = materials.add(Color::ORANGE.into());
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(hex(0, 0), 6)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(ColorMesh2dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(0.9)),
                    mesh: mesh_handle.clone().into(),
                    material: white_material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        layout,
        entities,
        selected_material,
        black_material,
        grey_material,
        white_material,
        origin_material,
    });
}

fn color_board(mut commands: Commands, map: Res<Map>,) {
    //Make Grey
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<Input<MouseButton>>,
    map: Res<Map>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    let window = windows.single();
    if let Some(pos) = window.cursor_position() {
        let pos = Vec2::new(pos.x - window.width() / 2.0, window.height() / 2.0 - pos.y);
        let coord = map.layout.world_pos_to_hex(pos);
        if mouse_buttons.just_pressed(MouseButton::Left) {
            println!("{}, {}", coord.x, coord.y);
        }
        if let Some(entity) = map.entities.get(&coord).copied() {
            if coord == highlighted_hexes.selected {
                return;
            }
            // Clear highlighted hexes materials
            for vec in [
                &highlighted_hexes.ring,
                &highlighted_hexes.line,
                &highlighted_hexes.wedge,
                &highlighted_hexes.dir_wedge,
                &highlighted_hexes.half_ring,
                &highlighted_hexes.rotated,
            ] {
                for entity in vec.iter().filter_map(|h| map.entities.get(h)) {
                    commands
                        .entity(*entity)
                        .insert(map.white_material.clone());
                }
            }
            commands
                .entity(map.entities[&highlighted_hexes.selected])
                .insert(map.white_material.clone());
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(map.white_material.clone());
            
            // Make the origin orange
            commands
                .entity(map.entities[&Hex::new(0, 0)])
                .insert(map.origin_material.clone());
            // Make the selected tile red
            commands
                .entity(entity)
                .insert(map.selected_material.clone());
            highlighted_hexes.selected = coord;
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}