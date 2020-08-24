use std::f32;
use clap::{App, load_yaml, value_t};
use minifb::{Key, Window, WindowOptions};
use rand;

mod vec3;
mod ray;
mod world;
mod hitable;
mod camera;

use vec3::{Vec3, unit_vector};
use ray::{Ray};
use world::{Sphere};
use hitable::{Hitable, HitableList, HitRecord};
use camera::{Camera};
use rand::{Rng};

const DEFAULT_WIDTH: usize = 960;
const DEFAULT_HEIGHT: usize = 480;


fn to_bgra(r: u32, g: u32, b: u32, a: u32) -> u32 {
    a << 24 | r << 16 | g << 8 | b
}

fn color(r: Ray, world: &HitableList) -> Vec3
{

    let mut rec: HitRecord = HitRecord::new(0.0,
                                            Vec3::new(0.0,0.0,0.0),
                                            Vec3::new(0.0,0.0,0.0));
    if world.hit(r,0.0001 ,f32::MAX, &mut rec)
    {
        return 0.5*Vec3::new(rec.normal.x()+1.0,
                             rec.normal.y()+1.0,
                             rec.normal.z()+1.0);
    }
    else
    {
        let unit_dir = unit_vector(r.direction);
        let t = 0.5 * (unit_dir.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0,1.0,1.0) + t*Vec3::new(0.5,0.7,1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let _config = matches.value_of("config").unwrap_or("default.conf");

    // get the output frame height/widths
    let ny: usize = value_t!(matches, "height", usize).unwrap_or(DEFAULT_HEIGHT);
    let nx: usize = value_t!(matches, "width", usize).unwrap_or(DEFAULT_WIDTH);
    let ns: usize = 10;


    println!("Size: {}x{} (HxW)", ny, nx);
    
    let mut output_buffer: Vec<Vec<u32>> = vec![vec![0; nx]; ny];
    
    let _lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let _horizontal = Vec3::new(4.0, 0.0, 0.0);
    let _vertical = Vec3::new(0.0, 2.0, 0.0);
    let _origin = Vec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(_origin, _lower_left_corner, _horizontal, _vertical);

    let scale = 255.99;

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    let world: HitableList = HitableList::new(list);

    for j in (0..ny).rev()
    {
        for i in 0..nx
        {
            let mut col = Vec3::new(0.0,0.0,0.0);
            for _ in 0..ns
            {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world);
            }
            col /= ns as f32;
            output_buffer[(ny - 1) - j][i] = to_bgra((scale * col[0]) as u32,
                                         (scale * col[1]) as u32,
                                         (scale * col[2]) as u32,
                                         0);
        }
    }

    let mut window = Window::new(
        "Raytracer - ESC to exit",
        nx,
        ny,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way

        let flatten_array: Vec<u32> = output_buffer
                        .iter()
                        .flat_map(|array| array.iter())
                        .cloned()
                        .collect();
        window.update_with_buffer(&flatten_array, nx,ny).unwrap();
    }
}
