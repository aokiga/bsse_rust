use crate::material::Material;
use crate::ray::{Ray, RayIntersection, Renderable};
use crate::vec3::Vec3;

#[derive(PartialEq, Debug, Clone)]
pub struct Chessboard {
    pub(crate) material1: Material,
    pub(crate) material2: Material,
}

const EPS: f32 = 1e-3;

impl Renderable for Chessboard {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        if f32::abs(ray.direction.y) < EPS {
            return None;
        }
        let distance = -(ray.origin.y + 4.0) / ray.direction.y;
        let point = ray.origin + ray.direction * distance;
        if distance > 0.0 {
            Some(RayIntersection {
                distance,
                point,
                normal: Vec3::new(0.0, 1.0, 0.0),
                material: if ((point.x * 0.5 + 1000.0) as i32 + (0.5 * point.z) as i32) % 2 == 0 {
                    self.material1
                } else {
                    self.material2
                },
            })
        } else {
            None
        }
    }
}
