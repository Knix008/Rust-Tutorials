fn main(){
   let mut b:(i32,bool,f64) = (110,true,10.9);
   print01(b);
   print02(&mut b);
   print01(b)
}

//pass the tuple as a parameter
fn print01(x:(i32,bool,f64)) {
   println!("Inside print01 method");
   println!("{:?}",x);
}

fn print02(x:&mut (i32,bool,f64)) {
    println!("Inside print02 method");
    x.0 = 1;
    x.1 = false;
    x.2 = 0.0; 
    println!("{:?}", x);
}

