
fn expected_minutes_in_oven(mut num1: i32) ->i32{
     num1 = 40;
     return num1;
    }

fn reamining_minutes_in_oven(mut time: i32,  mut minutes: i32) ->i32{
    time = time - minutes;
    return time;
}

fn preparation_time_in_minutes(num1: i32) ->i32{
    num1*2
}

fn elapsed_time_in_minutes(num1: i32, num2: i32)-> i32{
    num1 + num2
}
fn main() {
    let mut minutes = 30;
    let  mut time = expected_minutes_in_oven(10);
    let mut remain = reamining_minutes_in_oven(time, minutes);
    let mut layers = String::new();
     


    println!("The total time in the oven is {}", time);
    println!("The remaining time in the oven is {}", remain);

    println!("How many layers does the pizza have? ");

    std::io::stdin()
    .read_line(&mut layers)
    .expect("Erro");
     
    
    let layers:i32 = layers.trim().parse().expect("Invalid input");


    let mut time_prep = preparation_time_in_minutes(layers);

    println!("The pizza will take {} minutes", time_prep);

    let mut remain_calc = (40 - remain);

    let mut work_time = elapsed_time_in_minutes(time_prep, remain_calc);

    println!("I have been working on the pizza for {} minutes", work_time);
}

