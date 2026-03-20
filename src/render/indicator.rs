use bevy::{
    asset::{Asset, Assets},
    color::LinearRgba,
    ecs::system::{Commands, ResMut},
    math::{primitives::Rectangle, Vec3},
    mesh::{Mesh, Mesh2d},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d},
    transform::components::Transform,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GraticuleRingMaterial {
    #[uniform(0)]
    pub divisions: f32,
    #[uniform(1)]
    pub color: LinearRgba,
    #[uniform(2)]
    pub thickness: f32,
    #[uniform(3)]
    pub smoothness: f32,
    #[uniform(4)]
    pub divisions_smoothness: f32,
}

impl Material2d for GraticuleRingMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/graticule_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn spawn_graticule_ring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<GraticuleRingMaterial>>,
    position: Vec3,
    diameter: f32,
    divisions: f32,
    thickness: f32,
    smoothness: f32,
    divisions_smoothness: f32,
    color: LinearRgba,
) {
    let mesh = meshes.add(Rectangle::new(diameter, diameter));

    let material = materials.add(GraticuleRingMaterial {
        divisions,
        thickness: thickness / diameter,
        smoothness,
        divisions_smoothness,
        color,
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position),
    ));
}
