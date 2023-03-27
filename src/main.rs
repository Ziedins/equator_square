use std::{io, f32::consts::PI};

fn main() {
    println!("This is equator square calculator!");

    let side_length: f32;

    loop {

        println!("Please enter square side length : ");
        let mut side_length_input = String::new();
        io::stdin()
            .read_line(&mut side_length_input)
            .expect("Failed to read input");

        side_length = match side_length_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please enter positive number for side length!"); continue},
        };

        println!("Entered side length is {}", side_length);
        break;
    }

    let equator_length: f32;

    loop {

        println!("Enter circumference of sphere (equator length) : ");
        let mut equator_length_input = String::new();
        io::stdin()
            .read_line(&mut equator_length_input)
            .expect("Failed tp read input");

        equator_length = match equator_length_input.trim().parse() {
           Ok(num) => num,
           Err(_) => {println!("Please enter positive number for circumference!"); continue;}
        };

        println!("Entered circumference : {}", equator_length);
        break;
    }

    let equator_radius: f32 = equator_length / (2. * PI);
    let arc_angle: f32 = (side_length * 360.) / equator_length;
    let smaller_circle_radius: f32 = equator_radius * (90. - arc_angle).to_radians().sin();
    let smaller_circle_circumfrence: f32 = 2. * PI * smaller_circle_radius;
    let smaller_circle_arc_angle: f32 = (side_length * 360.) / smaller_circle_circumfrence;
    let equator_arc_length: f32 = smaller_circle_arc_angle * (PI / 180.) * equator_radius;

    let distance_from_original_location: f32 = equator_arc_length - side_length;

    println!("The distance from the original location on the equator is : {}", distance_from_original_location);
}
