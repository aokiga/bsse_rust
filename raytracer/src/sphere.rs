use crate::material::Material;
use crate::ray::{Ray, RayIntersection, Renderable};
use crate::vec3::{dot_product, Vec3};

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Renderable for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        let l = self.center - ray.origin;
        let tca = dot_product(&l, &ray.direction);
        let d2 = dot_product(&l, &l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = f32::sqrt(self.radius * self.radius - d2);
        let t0 = tca - thc;
        let t1 = tca + thc;
        let distance = if t0 < 0.0 { t1 } else { t0 };
        if distance >= 0.0 {
            let point = ray.origin + ray.direction * distance;
            Some(RayIntersection {
                distance,
                point,
                normal: (point - self.center).normalize(),
                material: self.material,
            })
        } else {
            None
        }
    }
}
