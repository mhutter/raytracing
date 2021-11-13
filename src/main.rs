use std::{
    f64::INFINITY,
    io::{stderr, stdout, BufWriter, Write},
};

use rand::Rng;
use raytracing::{
    camera::Camera,
    hittable::{HitRecord, Hittable, HittableList, Sphere},
    ray::Ray,
    vec3::{Color, Vec3},
};

const FACTOR: f64 = 1.0;

fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1, 1, 1));
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

    // World
    let world: HittableList = vec![
        // sphere
        Box::new(Sphere {
            center: Vec3::new(0, 0, -1),
            radius: 0.5,
        }),
        // ground
        Box::new(Sphere {
            center: Vec3::new(0, -100.5, -1),
            radius: 100.0,
        }),
    ];

    // Camera
    let camera: Camera = Camera::default();

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
                color += ray_color(ray, &world);
            }
            color.write(&mut stdout, SAMPLES_PER_PIXEL)?;
        }
    }
    stdout.flush()?;
    write!(&stderr, "\nDone\n")
}
