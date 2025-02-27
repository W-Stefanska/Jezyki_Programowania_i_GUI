fn main() {
    println!("\n\nZadanie 1\n");
    let (a, b) = (7332, 5064);
    println!("Najwyzszy wspolny dzielnik liczb {} i {}: {}", a, b, nwd(a, b));

    println!("\n\nZadanie 2\n");
    tryg();
}


// Zadanie 2
fn nwd(mut a:i32, mut b: i32) -> i32 {
    if b == 0 { return a }
    let res = a%b;
    if res == 0 {
        return b;
    }
    else {
        a = b;
        b = res;
        return nwd(a, b);
    }
}

#[test]
fn test_nwd() {
    assert_eq!(nwd(5, 6), 1);
    assert_eq!(nwd(12, 16), 4);
    assert_eq!(nwd(100, 25), 25);
    assert_eq!(nwd(0, 7), 7);
    assert_eq!(nwd(7, 0), 7);
}

// Zadanie 3
fn tryg() {
    println!("| {0:^2} | {1:^5} | {2:^5} | {3:^5} |", "Kąt", "Sin", "Cos", "Tan");
    for angle in 0 .. 46 {
        let x: f32 = angle as f32;
        let s = x.to_radians().sin();
        let c = x.to_radians().cos();
        let t = x.to_radians().tan();
        println!("| {angle:3} | {s:5.2} | {c:5.2} | {t:5.2} |");
    }
    
}