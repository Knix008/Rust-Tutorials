use std::collections::HashMap;

fn fibo(fibohash:&mut HashMap<i64, i64>, n:i64) -> i64 {
    let result:i64;

    /* Check n == 0 or n == 1 */
    if (n == 0)||(n == 1) {
        return n;
    }

    result = fibo(fibohash, n - 2) + fibo(fibohash, n - 1);
    fibohash.insert(n, result);
    return result;
}

fn main() {
    let n:i64 = 20;
    let mut i:i64 = 0;
    let mut fibohash = HashMap::new();

    fibohash.insert(0, 0);
    fibohash.insert(1, 1);

    // Calculate fibonacci number.
    fibo(&mut fibohash, n);
    while i <= n {
        println!("{} {:?}", i, fibohash.get(&i));
        i += 1;
    }
    return;
}

