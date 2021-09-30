mod chessboard;
mod light;
mod material;
mod ray;
mod render;
mod scene;
mod sphere;
mod vec3;

use crate::chessboard::Chessboard;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Renderable;
use crate::render::{render_scene, CameraParams};
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use std::f32::consts::PI;

fn main() {
    let camera_params = CameraParams {
        width: 1024,
        height: 768,
        fov: PI / 2.0,
    };

    let ivory = Material {
        diffuse_color: Vec3::new(0.4, 0.4, 0.3),
        albedo: [0.6, 0.3, 0.1, 0.0],
        specular_exponent: 50.0,
        refractive_index: 1.0,
    };

    let glass = Material {
        diffuse_color: Vec3::new(0.6, 0.7, 0.8),
        albedo: [0.0, 0.5, 0.1, 0.8],
        specular_exponent: 125.0,
        refractive_index: 1.5,
    };

    let red_rubber = Material {
        diffuse_color: Vec3::new(0.3, 0.1, 0.1),
        albedo: [0.9, 0.1, 0.0, 0.0],
        specular_exponent: 10.0,
        refractive_index: 1.0,
    };

    let mirror = Material {
        diffuse_color: Vec3::new(1.0, 1.0, 1.0),
        albedo: [0.0, 10.0, 0.8, 0.0],
        specular_exponent: 1425.0,
        refractive_index: 1.0,
    };

    let spheres: Vec<Box<dyn Renderable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        }),
        Box::new(Sphere {
            center: Vec3::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        }),
        Box::new(Sphere {
            center: Vec3::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
        }),
        Box::new(Chessboard {
            material1: Material {
                diffuse_color: Vec3::new(0.3, 0.3, 0.3),
                albedo: [1.0, 0.0, 0.0, 0.0],
                specular_exponent: 0.0,
                refractive_index: 1.0,
            },
            material2: Material {
                diffuse_color: Vec3::new(0.3, 0.2, 0.1),
                albedo: [1.0, 0.0, 0.0, 0.0],
                specular_exponent: 0.0,
                refractive_index: 1.0,
            },
        }),
    ];
    let lights = vec![
        Light {
            position: Vec3::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3::new(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        Light {
            position: Vec3::new(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];
    let scene = Scene {
        objects: spheres,
        lights,
    };

    render_scene(&scene, &camera_params)
        .save("output.png")
        .unwrap()
}
