fn main() {

    //-------------------------------------
    //Define two tuples calls p1 and p2 which will represents points and will have two values, one for the x-axis and one for y-axis.
    // Write a program to display the absolute difference of the values of x-axis and the y-axis on the terminal.
    //-----------------------------------------------

    let my_tuple = (3.5,2.2);
    let (p1, p2) = my_tuple;

    let result = (p1 as f64 - p2 as f64).abs();

    println!("The absolute difference of x-axis and y-axis is: {}", result);

//2- In this question, we will implement the same problem as disscussed in Question 1 but wtih the help of arrays. 

    let mut p1: [f64; 2] = [3.5, -4.3];
    let mut p2: [f64; 2] = [2.2, 9.0];

    let mut result: [f64;2] = [(p1[0] - p2[0]).abs(), (p1[1] - p2[1]).abs()];

    println!("The result is {:?}", result);


//The distance between two points p1 with coordinates (x1, y1) 
//and p2 with coordinates (x2,y2) is computed using the formula sqrt ((x1 - x2)^2 - (y1-y2)^2 )   
//where ^ = is used to indicate the exponent.
// Write a program which will first declare two points p1 and p2 using tuples and will Initialize the tuple p1 from 
//the values of (4.0, 3.0) and the tuple p2 from the values of (5.0, 4.5). Next, write a statement for determining their distance and then display the result to the terminal terminal. 

    let p1 = (4.0,3.0);

    let p2 = (5.0,4.5);

    let (x1, y1) = p1;

    let (x2,y2) = p2;


    let mut result:f64=(((p1.0 - p2.0) as f64).powf(2.0)).abs();

    let mut result2:f64 = ((((p1.1 - p2.1) as f64).powf(2.0))).abs();

    let result: f64 = ((result - result2)).abs();
    let result: f64 = (result.sqrt()).abs();

    println!("The distance between the coordinates is: {}", result);
}
