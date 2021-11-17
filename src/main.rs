use std::{
    f64::INFINITY,
    io::{stderr, stdout, BufWriter, Write},
};

use rand::Rng;
use rayon::prelude::*;
use raytracing::{
    camera::Camera,
    hittable::{Hittable, HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal, ScatterResult},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

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

fn random_scene() -> impl Hittable {
    let mut world: HittableList = Vec::new();
    let mut rng = rand::thread_rng();

    // Floor
    world.push(Box::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center - Vec3::new(4, 0.2, 0)).length() > 0.9 {
                let sphere: Box<dyn Hittable + Sync> = match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo)))
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)))
                    }
                    _ => {
                        // Glass
                        Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5)))
                    }
                };

                world.push(sphere);
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Dielectric::new(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}

fn main() -> Result<(), std::io::Error> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: u8 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13, 2, 3);
    let lookat = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
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
            let color: Color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f64 + rand::thread_rng().gen::<f64>()) / width;
                    let v = (j as f64 + rand::thread_rng().gen::<f64>()) / height;
                    let ray = camera.get_ray(u, v);
                    ray_color(ray, &world, MAX_DEPTH)
                })
                .sum();
            color.write(&mut stdout, SAMPLES_PER_PIXEL)?;
        }
    }
    stdout.flush()?;
    write!(&stderr, "\nDone\n")
}
