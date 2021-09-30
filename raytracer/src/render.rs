use crate::ray::{Ray, RayIntersection, Renderable};
use crate::scene::Scene;
use crate::vec3::{dot_product, Vec3};
use image::{DynamicImage, Rgb};

pub struct CameraParams {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

const BACKGROUND_COLOR: Vec3 = Vec3 {
    x: 0.2,
    y: 0.7,
    z: 0.8,
};

fn reflect(i: &Vec3, n: &Vec3) -> Vec3 {
    *i - *n * 2.0 * dot_product(i, n)
}

const EPS: f32 = 1e-3;

fn refract(i: &Vec3, n: Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
    let cosi = -dot_product(i, &n).clamp(-1.0, 1.0);
    if cosi < 0.0 {
        return refract(i, n * -1.0, eta_i, eta_t);
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        *i * eta + n * (eta * cosi - f32::sqrt(k))
    }
}

const DEPTH_LIM: u32 = 4;

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Vec3 {
    if depth > DEPTH_LIM {
        return BACKGROUND_COLOR;
    }
    let RayIntersection {
        distance: _distance,
        material,
        point,
        normal,
    } = match scene.ray_intersect(ray) {
        None => return BACKGROUND_COLOR,
        Some(x) => x,
    };

    let reflect_dir = reflect(&ray.direction, &normal).normalize();
    let reflect_sign = if dot_product(&reflect_dir, &normal) < 0.0 {
        -1.0
    } else {
        1.0
    };
    let reflect_orig = point + normal * EPS * reflect_sign;
    let reflect_ray = Ray {
        origin: reflect_orig,
        direction: reflect_dir,
    };
    let reflect_color = cast_ray(scene, &reflect_ray, depth + 1);

    let refract_dir = refract(&ray.direction, normal, material.refractive_index, 1.0).normalize();
    let refract_sign = if dot_product(&refract_dir, &normal) < 0.0 {
        -1.0
    } else {
        1.0
    };
    let refract_orig = point + normal * EPS * refract_sign;
    let refract_ray = Ray {
        origin: refract_orig,
        direction: refract_dir,
    };
    let refract_color = cast_ray(scene, &refract_ray, depth + 1);

    let mut specular_light_intensity = 0.0;
    let mut diffuse_light_intensity = 0.0;
    for light in scene.lights.iter() {
        let light_dir = (light.position - point).normalize();
        let light_distance = (light.position - point).length();
        let light_sign = if dot_product(&light_dir, &normal) < 0.0 {
            -1.0
        } else {
            1.0
        };
        let shadow_orig = point + normal * EPS * light_sign;
        let tmp_ray = Ray {
            origin: shadow_orig,
            direction: light_dir,
        };
        if let Some(tmp_intersection) = scene.ray_intersect(&tmp_ray) {
            if (tmp_intersection.point - shadow_orig).length() < light_distance {
                continue;
            }
        }
        diffuse_light_intensity += light.intensity * dot_product(&light_dir, &normal).max(0.0);
        specular_light_intensity += light.intensity
            * f32::powf(
                dot_product(&reflect(&light_dir, &normal), &ray.direction).max(0.0),
                material.specular_exponent,
            );
    }
    diffuse_light_intensity * material.diffuse_color * material.albedo[0]
        + specular_light_intensity * Vec3::new(1.0, 1.0, 1.0) * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3]
}

pub fn render_scene(scene: &Scene, params: &CameraParams) -> DynamicImage {
    let mut framebuffer = DynamicImage::new_rgb8(params.width, params.height);

    let width = params.width as f32;
    let height = params.height as f32;

    for j in 0..params.height {
        for i in 0..params.width {
            let x = (2.0 * ((i as f32) + 0.5) / width - 1.0) * f32::tan(params.fov / 2.0) * width
                / height;
            let y = -(2.0 * ((j as f32) + 0.5) / height - 1.0) * f32::tan(params.fov / 2.0);
            let direction = Vec3::new(x, y, -1.0).normalize();
            let ray = Ray {
                origin: Vec3::zero(),
                direction,
            };
            let color = cast_ray(scene, &ray, 0);

            let pixel_color =
                Rgb([color.x, color.y, color.z].map(|x| (x.clamp(0.0, 1.0) * 255.0) as u8));

            framebuffer
                .as_mut_rgb8()
                .unwrap()
                .put_pixel(i, j, pixel_color);
        }
    }
    framebuffer
}
