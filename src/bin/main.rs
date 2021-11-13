use std::{
    f64::INFINITY,
    io::{stderr, Write},
};

use raytracing::{
    hittable::{HitRecord, Hittable, HittableList, Sphere},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

const FACTOR: f64 = 1.0;

fn ray_color(ray: Ray, world: &HittableList) -> Color {
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
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new(0, 0, 0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0, 0);
    let vertical = Vec3::new(0, VIEWPORT_HEIGHT, 0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, FOCAL_LENGTH);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let stderr = stderr();
    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(&stderr, "\rScanlines remaining: {} of {} ", j, IMAGE_HEIGHT)?;
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / width;
            let v: f64 = j as f64 / height;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray { origin, direction };
            let color = ray_color(ray, &world);
            println!("{}", color);
        }
    }

    write!(&stderr, "\nDone\n")
}
