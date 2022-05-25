fn main() {
    
    /*let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Fail");
    let mut input:u8 = input.trim().parse().expect("Erro");
    let mut j = 0;
    let mut var = &input;
    let mut var2 = &input;
    let mut out = 0;
    let mut out2 = 0;

    for  i in 0..var+1{
     out = out + i;
    }

  out = out.pow(2);
  println!("the square of sum is: {}", out);

    for i in 0..var2+1{
        out2 = out2 + i.pow(2);
    }
    println!("the sum of squares is: {}", out2);

    let mut dif = out-out2;
    println!("The difference is: {}", dif);
    */
    /*let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Fail");
    let mut input:i32 = input.trim().parse().expect("Erro");
    let mut var = &input;
    let mut var2 = &input;
    let mut out: Vec<i32>= Vec::new();
    let mut out2: Vec<i32>= Vec::new();;
    let mut out3:i32 = 0;


    for  i in 0..var+1{
        if(i%3 == 0){
            out.push(i);
        }else if(i%5 == 0){
            out2.push(i);
        }else{
            continue;
        }
    }

    println!("Multiples of 3: {:?}", out);
    println!("Multiples of 5: {:?}", out2);

    for   i in out.iter(){
        out3 =  out3 + i;
    }
    for i in out2.iter(){
        out3 = out3 + i;
    }

    println!("The sum of the multiples of 3 and 5 is:{} ", out3);
   */
  
  println!("How many hours is the production time?");
  let mut time = String::new();
  std::io::stdin().read_line(&mut time).expect("Fail");
  let mut time:i32 = time.trim().parse().expect("Erro");
  let mut var = &time;


 println!("What is the speed of the production?");
 let mut speed = String::new();
  std::io::stdin().read_line(&mut speed).expect("Fail");
  let mut speed:i32 = speed.trim().parse().expect("Erro");
  let mut var = &speed;
 

  let mut car_welldone = production_rate_per_hour(time, speed);

  println!("The number of cars successfully produced per hour is: {}", car_welldone);

  let mut car_welldone_s = Cars_produced_per_minutes(time, speed);

  println!("The number of cars successfully produced per minute is: {}", car_welldone_s);

}

fn production_rate_per_hour(hours: i32,  speed: i32)->(f64){
    let mut out = (speed * 221);   
    out = out * hours;
    let mut success:f64 = 0.0;

    if(speed>= 1 && speed <=4){
        success = (out as f64);
    }else if(speed>=5 && speed <= 8){
        success = (out as f64) *0.9;
    }else if(speed>= 9){
        success = (out as f64) * 0.77;
    }

    success = success/(hours as f64);
    return success;
}

fn Cars_produced_per_minutes(hours: i32,  speed: i32)->(f64){
    let mut out = (speed * 221);   
    out = out * hours;
    let mut success_per_minute:f64 = 0.0;

    if(speed>= 1 && speed <=4){
        success_per_minute = (out as f64);
    }else if(speed>=5 && speed <= 8){
        success_per_minute = (out as f64) *0.9;
    }else if(speed>= 9){
        success_per_minute = (out as f64) * 0.77;
    }
    success_per_minute = success_per_minute/(hours as f64);
    success_per_minute = success_per_minute/60.0;
    return  success_per_minute;

}