use crate::light::Light;
use crate::ray::{Ray, RayIntersection, Renderable};

pub struct Scene {
    pub objects: Vec<Box<dyn Renderable>>,
    pub lights: Vec<Light>,
}

fn choose_best_intersection(
    acc: Option<RayIntersection>,
    x: Option<RayIntersection>,
) -> Option<RayIntersection> {
    if acc.is_none() || x.is_none() {
        return if acc.is_some() { acc } else { x };
    }
    let intersection1 = acc.unwrap();
    let intersection2 = x.unwrap();
    if intersection1.distance < intersection2.distance {
        Some(intersection1)
    } else {
        Some(intersection2)
    }
}

impl Renderable for Scene {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        self.objects
            .iter()
            .map(|x| x.ray_intersect(ray))
            .fold(None, choose_best_intersection)
    }
}
