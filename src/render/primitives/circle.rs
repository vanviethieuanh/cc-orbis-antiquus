use bevy::{
    asset::{Asset, Assets},
    color::{Color, LinearRgba},
    ecs::system::{Commands, ResMut},
    math::{primitives::Rectangle, Vec3},
    mesh::{Mesh, Mesh2d},
    reflect::TypePath,
    render::{alpha::AlphaMode, render_resource::AsBindGroup},
    sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d},
    transform::components::Transform,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CircleMaterial {
    #[uniform(0)]
    pub border_color: LinearRgba,
    #[uniform(1)]
    pub fill_color: LinearRgba,

    #[uniform(2)]
    pub border_thickness: f32,

    #[uniform(3)]
    pub smoothness: f32,
}

impl Material2d for CircleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/primitives/circle.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn spawn_circle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CircleMaterial>>,
    position: Vec3,
    diameter: f32,
    border_thickness: f32,
    smoothness: f32,
    border_color: LinearRgba,
    fill_color: LinearRgba,
) {
    let mesh = meshes.add(Rectangle::new(diameter, diameter));

    let material = materials.add(CircleMaterial {
        fill_color,
        border_color,
        border_thickness: border_thickness / diameter,
        smoothness,
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position),
    ));
}
