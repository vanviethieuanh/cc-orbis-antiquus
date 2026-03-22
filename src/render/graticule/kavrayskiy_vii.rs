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
pub struct KavrayskiyViiGraticuleMaterial {
    #[uniform(0)]
    pub meridians: f32,
    #[uniform(1)]
    pub parallels: f32,
    #[uniform(2)]
    pub thickness: f32,
    #[uniform(3)]
    pub smoothness: f32,
    #[uniform(4)]
    pub color: LinearRgba,
}

impl Material2d for KavrayskiyViiGraticuleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/graticule/kavrayskiy_vii.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn spawn_kavrayskiy_vii_graticule(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<KavrayskiyViiGraticuleMaterial>>,
    position: Vec3,
    size: Rectangle,
    meridians: f32,
    parallels: f32,
    thickness: f32,
    smoothness: f32,
    color: LinearRgba,
) {
    let mesh = meshes.add(size);

    let material = materials.add(KavrayskiyViiGraticuleMaterial {
        meridians,
        parallels,
        thickness,
        smoothness,
        color,
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position),
    ));
}
