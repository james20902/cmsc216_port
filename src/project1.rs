fn classify_triangle(side1: u8, side2: u8, side3: u8) -> i8 {
    let t1 = (side1 + side2) < side3;
    let t2 = (side2 + side3) < side1;
    let t3 = (side1 + side3) < side2;

    if (t1 && t2 && t3) || (side1 <= 0 || side2 <= 0 || side3 <= 0) {
        return -1;
    }

    if ((side1 + side2 + side3) / 3) == side3 {
        return 3;
    }

    if side1 == side2 || side2 == side3 || side3 == side1 {
        return 2;
    }

    1
}

fn reverse_digits(mut n: i64) -> i64 {
    let mut reversed_result : i64 = 0;

    if n % 10 == 0 {
        return -1;
    }

    while n > 0 {
        reversed_result = (reversed_result * 10) + n % 10;
        n /= 10;
    }

    reversed_result
}

fn catalan(n : u8) -> u64 {
    if n == 0 {
        return 1;
    }
    
    // these types are fixed-width, and unlike C, rust doesn't handle any 
    // sort of casting through a heirarchy defined by the standard,
    // so we need to explicitly cast all values to the common denominator.
    let numerator: u64 = (((4 * n) - 2) as u64) * catalan(n - 1); 
    numerator / ((n + 1) as u64)
}


fn main() {
    println!("classify triangle is 1, 3, 3: {}", classify_triangle(1, 3, 3));
    println!("reverse_digits for 12345 is: {}", reverse_digits(12345));
    println!("catalan result for 10 is: {}", catalan(10));
}
