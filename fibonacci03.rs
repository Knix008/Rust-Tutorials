fn fibo(vec:&mut Vec<i64>, n: usize) {
    /* if n == 0 or n == 1, then make it manually. */
    if n == 0 {
        vec.push(0);
        return;
    }
    if n == 1 {
        vec.push(0);
        vec.push(1);
        return;
    }

    /* if n >= 2, then use previous calculation result. */
    vec.push(0);
    vec.push(1);
    for i in 2..n {
        vec.push(vec[i - 2] + vec[ i - 1 ]);
    }
    return;
}

fn main() {
    let n: usize = 20;
    let mut vec:Vec<i64> = Vec::new();

    // Calculate fibonacci number.
    fibo(&mut vec, n);
    println!("{:?}", vec);
    return;
}

