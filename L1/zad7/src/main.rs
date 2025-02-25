struct Rectangle {
    a:f32, 
    b:f32,
    position: [f32; 2], 
}

fn main() {
    let mut rec = Rectangle{
        a: 5.0,
        b: 10.0,
        position: [0.0, 0.0],
    };
    println!("Rectangle:\na: {}\nb: {}\npos: {}, {}\n", rec.a, rec.b, rec.position[0], rec.position[1]);
    flip(&mut rec);
    println!("Flipped by 90*:\na: {}\nb: {}\n", rec.a, rec.b);
    println!("Area:\na: {}\nb: {}\narea: {}\n", rec.a, rec.b, area(&rec));
    println!("Perimeter:\na: {}\nb: {}\nperimeter: {}\n", rec.a, rec.b, perimeter(&rec));
    scale(&mut rec, 3.5);
    println!("Scaled by 3.5:\na: {}\nb: {}\n", rec.a, rec.b);
    println!("New area:\na: {}\nb: {}\narea: {}\n", rec.a, rec.b, area(&rec));
    println!("New perimeter:\na: {}\nb: {}\nperimeter: {}\n", rec.a, rec.b, perimeter(&rec));
    move_to(&mut rec, [2.0, 3.0]);
    println!("Moved to [2, 3]:\nx: {}\ny: {}\n", rec.position[0], rec.position[1]);
    move_by(&mut rec, [-5.0, 3.0]);
    println!("Moved by [-5, 3]:\nx: {}\ny: {}\n", rec.position[0], rec.position[1]);
}

fn area(r: &Rectangle) -> f32 {
    return r.a * r.b;
}

fn perimeter(r: &Rectangle) -> f32 {
    return 2.0*r.a + 2.0*r.b;
}

fn scale(r: &mut Rectangle, scale: f32) {
    r.a *= scale;
    r.b *= scale;
}

fn flip(r: &mut Rectangle) {
    std::mem::swap(&mut r.a, &mut r.b);
}

fn move_to(r: &mut Rectangle, localization: [f32; 2]) {
    r.position = localization;
}

fn move_by(r: &mut Rectangle, two_d_vector: [f32; 2]) {
    r.position[0] += two_d_vector[0];
    r.position[1] += two_d_vector[1];
}

#[test]
fn test_scale() {
    let mut rec = Rectangle{
        a: 2.0,
        b: 4.0,
        position: [1.0, 3.0]
    };
    scale(&mut rec, 0.5);
    assert_eq!(rec.a, 1.0);
    assert_eq!(rec.b, 2.0);
    
}

#[test]
fn test_area() {
    let rec = Rectangle{
        a: 2.0,
        b: 4.0,
        position: [1.0, 3.0]
    };
    assert_eq!(area(&rec), 8.0);
}

#[test]
fn test_perimeter() {
    let rec = Rectangle{
        a: 2.0,
        b: 4.0,
        position: [1.0, 3.0]
    };
    assert_eq!(perimeter(&rec), 12.0);
}

#[test]
fn test_move_to() {
    let mut rec = Rectangle{
        a: 2.0,
        b: 4.0,
        position: [1.0, 3.0]
    };
    move_to(&mut rec, [0.0, 0.0]);
    assert_eq!(rec.position[0], 0.0);
    assert_eq!(rec.position[1], 0.0);
}

#[test]
fn test_move_by() {
    let mut rec = Rectangle{
        a: 2.0,
        b: 4.0,
        position: [1.0, 3.0]
    };
    move_by(&mut rec, [-2.0, 4.0]);
    assert_eq!(rec.position[0], -1.0);
    assert_eq!(rec.position[1], 7.0);
}