use crate::material::Material;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RayIntersection {
    pub distance: f32,
    pub material: Material,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Renderable {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection>;
}
