use hexx::*;
use bevy::{prelude:: *, mesh::{PrimitiveTopology, Mesh, Indices},  asset::RenderAssetUsages}; 
use rand::*;

const HEX_SIZE: u32 = 2;
const WORLD_SIZE: u32 = 16; 

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_world_layout.in_set(WorldSet::Layout),
            setup_world.in_set(WorldSet::World),
        ))
        .configure_sets(Startup, WorldSet::Layout.before(WorldSet::World));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorldSet {
    Layout,
    World,
}

#[derive(Resource)]
pub struct WorldHexLayout {
    pub layout: HexLayout
}

#[derive(Component)]
pub struct HexPosition(pub Hex);

impl HexPosition {
pub fn get_world_pos(&self, hex_layout: &HexLayout) -> Vec2 {
        hex_layout.hex_to_world_pos(self.0)
    }
}

pub fn get_new_hex_direction() -> EdgeDirection {
    let mut rng = rand::rng();

    let directions = [ 
        EdgeDirection::FLAT_TOP,
        EdgeDirection::FLAT_TOP_RIGHT,
        EdgeDirection::FLAT_BOTTOM_RIGHT,
        EdgeDirection::FLAT_BOTTOM,
        EdgeDirection::FLAT_BOTTOM_LEFT,
        EdgeDirection::FLAT_TOP_LEFT,
    ];

    directions[rng.random_range(0..directions.len())]
}

pub fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
    let vertices: Vec<[f32; 3]> = mesh_info
        .vertices
        .into_iter()
        .map(|v| [v.x, v.z, v.y])
        .collect();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn get_hex_tex(atlas_layout: &TextureAtlasLayout, hex_texure_index: usize) -> UVOptions {
    let rect = atlas_layout.textures[hex_texure_index];
    let (uv_max, uv_min) = (rect.max.as_vec2(), rect.min.as_vec2());
    UVOptions::new().with_rect(
        uv_min / atlas_layout.size.as_vec2(),
        uv_max / atlas_layout.size.as_vec2(),
    ).flip_v()
}

pub fn setup_world_layout (
    mut commands: Commands,
){
    info!("Setting up world layout");

    commands.insert_resource(WorldHexLayout {layout: HexLayout {
        scale: Vec2::splat(128.),
        orientation: hexx::HexOrientation::Flat,
        ..default()
    }});
}

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    layout: Res<WorldHexLayout>,
    ) {

    info!("Setting up world");

    let hex_tilesheet_grass = asset_server.load("hex/terrain_grass.png");
    let hex_layout = TextureAtlasLayout::from_grid(UVec2::splat(125), 5, 5, Some(UVec2{x: 5, y: 5}), None);

    let world_grid = Hex::ZERO.range(WORLD_SIZE);
    
    let hex_material = materials.add(hex_tilesheet_grass);

    let mut rng = rand::rng();

    for chunk in world_grid {
        let center = chunk.to_higher_res(HEX_SIZE);
        let children = center.range(HEX_SIZE);

        let hex_chunk_mesh = children
            .map(|hex| {
                PlaneMeshBuilder::new(&layout.layout)
                    .at(hex)
                    .with_uv_options(get_hex_tex(&hex_layout, rng.random_range(0..3)))
                    .build()
            })
            .reduce(|mut acc, mesh| {
                acc.merge_with(mesh);
                acc
            })
            .unwrap();

        commands.spawn((
            Mesh2d(meshes.add(hexagonal_mesh(hex_chunk_mesh))),
            MeshMaterial2d(hex_material.clone()),
        ));
    }

}
