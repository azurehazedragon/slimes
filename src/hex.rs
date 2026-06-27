use super::*;

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
