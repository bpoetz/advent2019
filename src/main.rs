use std::collections::HashMap;
// rules:
// 6 digits
// at least two adjacent #'s are the same
// from left to right
// lb 246515 739105
//
type Password = [i32;6];//this is just an alias, we can't add methods to this as in a struct
// could do struct Password ([i32;6]) or struct Password(i32, i32, i32, i32, i32, i32)
fn check_password (password: Password)-> bool {
    let mut biggest = &pass[0];
    let mut consecutive_digits:HashMap<i32, i32> = HashMap::new();

    for (i,x) 
    in password
        .iter()
        .enumerate(){
        if i>0 && x==&pass[i-1]{
            //consecutive digit found.
            let count = consecutive_digits
                .entry(*x) //return the hash map value for key x
                .or_insert(0); // or initialize the key to 0
            *count +=1; // either way, we increment it.
        } 

        if x>=biggest {
            biggest = x;
        }
        else{
            return false;
        }
    }
    consecutive_digits // for each consecutive hash map entry
        .values() // pick just the value (don't care which digit it is)
        .map(|&x| x == 1__i32) // check if that value was unique
        .fold(false, |result, x| result || x) // check if any of those comparisons were true
}

fn number_to_password(num:i32) -> Password {
    let mut password = [0,0,0,0,0,0];
    // magic #s are good, actually
    for i in 1..7{
        password[6-i] = (num % 10_i32.pow(i as u32) ) / 10_i32.pow((i-1) as u32);
    }
    password
}
fn main(){
    let lb: i32 = 246515;
    let ub: i32 = 739105;
    let mut c: i32 = 0;
    let mut password: i32 = lb;

    while password < ub {
        if check_password(number_to_password(password)){
            c = c + 1;
        };
        password = password + 1;
    }
    println!("{:?}",c);

}
