use std::{
    f64::INFINITY,
    io::{stderr, stdout, BufWriter, Write},
};

use rand::Rng;
use raytracing::{
    camera::Camera,
    hittable::{Hittable, HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal, ScatterResult},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

const FACTOR: f64 = 1.0;

fn ray_color(ray: Ray, world: &impl Hittable, depth: u8) -> Color {
    if depth == 0 {
        // exceeded the ray bounce limit, no light is gathered.
        return Color::new(0, 0, 0);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        return match hit.material.scatter(ray, &hit) {
            ScatterResult::Scattered(scattered, attenuation) => {
                attenuation * ray_color(scattered, world, depth - 1)
            }
            ScatterResult::Absorbed(_) => Color::new(0, 0, 0),
        };
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::io::Error> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = (400.0 * FACTOR) as i32;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: u8 = 50;

    // World
    let world: HittableList = vec![
        // Ground
        Box::new(Sphere {
            center: Vec3::new(0, -100.5, -1),
            radius: 100.0,
            material: Lambertian::new(Color::new(0.8, 0.8, 0.0)),
        }),
        // Spheres - center
        Box::new(Sphere {
            center: Vec3::new(0, 0, -1),
            radius: 0.5,
            material: Lambertian::new(Color::new(0.1, 0.2, 0.5)),
        }),
        // Spheres - left
        Box::new(Sphere {
            center: Vec3::new(-1, 0, -1),
            radius: 0.5,
            material: Dielectric::new(1.5),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1, 0, -1),
            radius: -0.4,
            material: Dielectric::new(1.5),
        }),
        // Spheres - right
        Box::new(Sphere {
            center: Vec3::new(1, 0, -1),
            radius: 0.5,
            material: Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
        }),
    ];

    // Camera
    let camera = Camera::new(
        Point3::new(-2, 2, 1),
        Point3::new(0, 0, -1),
        Vec3::new(0, 1, 0),
        20.0,
        ASPECT_RATIO,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let stderr = stderr();
    let mut stdout = BufWriter::new(stdout());
    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(&stderr, "\rScanlines remaining: {} of {} ", j, IMAGE_HEIGHT)?;
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::thread_rng().gen::<f64>()) / width;
                let v = (j as f64 + rand::thread_rng().gen::<f64>()) / height;
                let ray = camera.get_ray(u, v);
                color += ray_color(ray, &world, MAX_DEPTH);
            }
            color.write(&mut stdout, SAMPLES_PER_PIXEL)?;
        }
    }
    stdout.flush()?;
    write!(&stderr, "\nDone\n")
}
