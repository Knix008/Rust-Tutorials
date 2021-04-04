fn change_value(arr: &mut [i32]) {
    arr[1] = 10;
}

fn main() {
    let mut arr: [i32; 4] = [0; 4];
    change_value(&mut arr);
    println!("this is {}", arr[1]);
}