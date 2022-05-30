//Complete the following code given below by filling in the code corresponding to the  comments

fn area(length:u32, width:u32) -> u32 {

    // Step 7: write the definition of the area here which is length * width and return the result

    width * length

 }
 fn main(){

    // Step 1: Write a print statement for asking user to input the width of a rectangle

    let mut n = String::new();

     

    println!("What is the width of a rectangle?");

    std::io::stdin()
    .read_line(&mut n)  // INPUT
    .expect("Fail");

    let n:f64 = n.trim().parse().expect("Invalid input");
    println!("The width is: {:?}", n);


    // Step 2: Write code for taking the input from the user of type u32 and store it in the variable of width

   
    let mut n = String::new();

    println!("What is the width of a rectangle?");

    std::io::stdin()
    .read_line(&mut n)  // INPUT
    .expect("Fail");

    let n:u32 = n.trim().parse().expect("Invalid input");
    println!("The width is: {:?}", n);


    // Step 3: Write a print statement for asking the user to input the length of a rectangle
  let mut m = String::new();

    println!("What is the length of a rectangle?");

    std::io::stdin()
    .read_line(&mut m)  // INPUT
    .expect("Fail");

    let m:f64 = m.trim().parse().expect("Invalid input");
    println!("The length is: {:?}", m);

 

    // Step 4: Write code for taking the input from the user of type u32 and store it in the variable of length

    let mut m = String::new();



    println!("What is the length of a rectangle?");
    std::io::stdin()
    .read_line(&mut m)  // INPUT
    .expect("Fail");

    let m:u32 = m.trim().parse().expect("Invalid input");
    println!("The length is: {:?}", m);



    let resultant_area = {

        // Step 5: call a function area() inside here with inputs of width and length which will return the area
         // Step 6: write code to print the resultant_area variable to the terminal

       
        let area_total =  area(n,m); 

        println!("The area is: {}", area_total);
    };

 }

