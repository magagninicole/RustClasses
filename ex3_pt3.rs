fn print_distance(point: (f32,f32)) -> f32{ 
   let mut n1 = point.0;
   let mut n2 = point.1; 

  let point =  ((n1-0.0).powf(2.0) + (n2 - 0.0).powf(2.0)).sqrt();
  return point;
}

fn main(){
  
  println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));
}