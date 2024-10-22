use math::tuple::Tuple4D;
fn main() {
    println!("Hello, world!");
    //position and velocity
    let mut projectile: [Tuple4D; 2] = [
        Tuple4D::new(0.0, 1.0, 0.0, 1.0),
        Tuple4D::new(1.0, 1.0, 0.0, 0.0),
    ];

    //gravity and wind
    let env: [Tuple4D; 2] = [
        Tuple4D::new(0.0, -0.1, 0.0, 0.0),
        Tuple4D::new(-0.01, 0.0, 0.0, 0.0),
    ];

    loop {
        if projectile[0].y <= 0.0 {
            break;
        }
        projectile = tick(&env, &projectile);
        println!("Position {:.2?}", projectile[0]);
    }
}

fn tick(env: &[Tuple4D; 2], proj: &[Tuple4D; 2]) -> [Tuple4D; 2] {
    let position = proj[0] + proj[1];
    let velocity = proj[1] + env[0] + env[1];
    return [position, velocity];
}
