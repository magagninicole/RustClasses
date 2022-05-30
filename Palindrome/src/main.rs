fn Palindrome(sentence: String) -> (bool){
    let mut leng = sentence.len();
    leng = leng-2;
    let mut aux = true;
    let mut i = 0; 
    while(i <= ((leng/2)-1)){
        if(sentence.chars().nth(i) == sentence.chars().nth(leng-i)){
            aux = true;
        }else{
            aux = false;
        }
      i = i+1;
    }
return aux;
}

fn main() {

    println!("Enter the sentence: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Fail");

    let mut itis = Palindrome(input);

    if(itis == true){
        println!("It's a palindrome");
    }else{ 
        println!("It's not a palindrome");
    }
}
