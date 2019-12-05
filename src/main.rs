use std::collections::HashMap;
// rules:
// 6 digits
// at least two adjacent #'s are the same
// from left to right
// lb 246515 739105
//
type Password = [i32;6];

fn check_password (pass: Password)-> bool {
    let mut biggest = &pass[0];
    let mut consecutive_digits:HashMap<i32, i32> = HashMap::new();
    let mut has_consecutive: bool = false;
    
    for (i,x) in pass.iter().enumerate(){
        if i>0 && x==&pass[i-1]{
            let count = consecutive_digits.entry(*x).or_insert(0);
            *count +=1;
            
            has_consecutive = true;
        } 
        
        if x>=biggest {
            biggest = x;
        }
        else{
            return false;
        }
    
    }
    let at_least_one_unique: bool = consecutive_digits
        .values()
        .map(|&x| x == 1__i32)
        .fold(false, |result, x| result || x);
    
    has_consecutive && at_least_one_unique
}

fn password_to_number(pass: Password) -> i32{
    pass
      .iter()
      .enumerate()
      .map(|(i,x)| x * 10_i32.pow((5 - i) as u32))
      .sum()
}

fn number_to_password(num:i32) -> Password {
    let mut pass = [0,0,0,0,0,0];
    for i in 1..7{
        pass[6-i] = (num % 10_i32.pow(i as u32) ) / 10_i32.pow((i-1) as u32);
    }
    pass
}
fn main(){
    let lb: Password = [2,4,6,5,1,5];
    let ub: i32 = 739105;
    let mut c: i32 = 0;
    let mut pass: Password = lb;

    println! ("{:?}", number_to_password(ub));
    while password_to_number(pass) < ub {
        if check_password(pass){
            c = c + 1;
        };
        pass = number_to_password(password_to_number(pass) + 1);
    }
    println!("{:?}",c);
    
}
