use std::io::{stderr, Write};

use raytracing::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Vec3::new(0, 0, -1), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0, 0, -1)).unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * unit_direction.y() + 1.0;
    (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::io::Error> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

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
            let color = ray_color(ray);
            println!("{}", color);
        }
    }

    write!(&stderr, "\nDone\n")
}
