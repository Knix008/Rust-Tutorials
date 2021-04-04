fn fibo(result:&mut [i64], n : usize) -> i64 {
    let mut temp : i64 = 0;
    if n == 0 {
        result[ n ] = temp;
        return 0;
    }
    if n == 1 {
        temp = 1;
        result[ n ] = temp;
        return 1;
    }

    if result[ n ] != 0 {
        temp = result[ n ];
        return temp;
    }
    temp = fibo(result, n - 2) + fibo(result, n - 1);
    result[ n ] = temp;
    return temp;
}

fn main() {
    let n: usize = 20;
    let mut result: [i64; 100] = [ 0 ; 100 ];

    // Calculate fibonacci number.
    fibo(&mut result, n);

    println!("");
    for i in 0..n {
        println!("{}", result[ i ]);
    }
    println!("");
}

