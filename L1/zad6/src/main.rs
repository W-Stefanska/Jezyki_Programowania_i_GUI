fn main() {
    let mut tab = [2, 3, 9, 0, 5, 2, 4, 6, 19, 31, 2, 5, 1, 6, 7, 8, 9, 10, 11];
    
    println!("\n\nZadanie 1\nSuma zawartosci tabeli: ");
    println!("{}", sum(&tab));
    
    let max_ascend_result = max_ascend(&tab);
    
    println!("\n\nZadanie 2\nNajdluzszy ciag rosnacy: ");
    for i in 0..max_ascend_result.len() {
        print!("{}, ", max_ascend_result[i]);
    };
    println!("\n");

    accumulate(&mut tab);
    println!("\n\nZadanie 3\nRezultat funkcji accumulate: ");
    for i in 0..tab.len() {
        print!("{}, ", tab[i]);
    };
    println!("\n");

}

fn sum(t: &[i32]) -> i32 {
    let mut sum = 0;
    for n in t {
        sum += n;
    }
    return sum;
}

fn max_ascend(t: &[i32]) -> &[i32] {
    let mut max: usize = 1;
    let mut current: usize = 1;
    let mut start: usize = 0;
    for n in 1..t.len() {
        if t[n] >= t[n - 1] {
            current += 1;
        } else {
            if current > max {
                max = current;
                start = n - current;
            }
            current= 1;
        }
    }
    if current > max {
        max = current;
        start = t.len() - current;
    }
    //println!("Max: {}, Current: {}, Start: {}", max, current, start);
    return &t[start..(start+max)];
}

fn accumulate(t: &mut [i32]) {
    for i in 0..t.len() {
        let sum = sum(&t[0..i]);
        t[i] += sum;
    }
}