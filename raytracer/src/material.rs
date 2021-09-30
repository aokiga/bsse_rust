use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub albedo: [f32; 4],
    pub specular_exponent: f32,
    pub refractive_index: f32,
}
