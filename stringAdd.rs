fn main(){
   let n1 = "Tutorials".to_string();
   let n2 = "Point".to_string();

   let n3 = n1 + &n2; // n2 reference is passed
   let n4 = n1 + n2;
   println!("{}",n3);
   println!("{}",n4);
}

