use std::io::{stderr, Write};

use raytracing::vec3::Color;

fn main() -> Result<(), std::io::Error> {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;
    let stderr = stderr();
    let mut color = Color::new(0, 0, 0.25);

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(&stderr, "\rScanlines remaining: {} of {} ", j, IMAGE_HEIGHT)?;
        for i in 0..IMAGE_WIDTH {
            color.0 = i as f64 / width;
            color.1 = j as f64 / height;

            println!("{}", color);
        }
    }

    write!(&stderr, "\nDone\n")
}
