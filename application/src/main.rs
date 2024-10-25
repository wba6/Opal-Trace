use canvas::canvas::Canvas;
use math::color::Color;
use math::point::Point3D;
use math::vector::Vector3D;

struct Projectile {
    location: Point3D,
    direction: Vector3D,
}

struct Env {
    gravity: Vector3D,
    wind: Vector3D,
}

fn main() {

    let mut canvas = Canvas::new(1920, 1080);

    //position and velocity
    let mut projectile: Projectile = Projectile {
        location: Point3D::new(0.0, 1.0, 0.0),
        direction: Vector3D::new(10.0, 10.0, 0.0),
    };

    //gravity and wind
    let env: Env = Env {
        gravity: Vector3D::new(0.0, -0.1, 0.0),
        wind: Vector3D::new(-0.01, 0.0, 0.0),
    };

    loop {
        if projectile.location.y() <= 0.0 {
            break;
        }
        projectile = tick(&env, &projectile);
        canvas.write_pixel(projectile.location.x() as u32, projectile.location.y() as u32, &Color::new(255.0, 70.0, 70.0));
        println!("Position {:.2?}", projectile.location);
    }

    canvas.write_to_ppm_file("test.ppm".parse().unwrap());
}

fn tick(env: &Env, proj: &Projectile) -> Projectile {
    let position = &proj.location + &proj.direction;
    let velocity = proj.direction + env.gravity + env.wind;
    return Projectile {
        location: position,
        direction: velocity,
    };
}
