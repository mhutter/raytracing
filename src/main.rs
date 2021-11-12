use std::io::{stderr, Write};

fn main() -> Result<(), std::io::Error> {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;
    let stderr = stderr();

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(&stderr, "\rScanlines remaining: {} of {} ", j, IMAGE_HEIGHT)?;
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / width;
            let g = j as f64 / height;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    write!(&stderr, "\nDone\n")
}
