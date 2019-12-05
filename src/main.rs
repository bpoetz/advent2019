use std::collections::HashMap;
// rules:
// 6 digits
// at least two adjacent #'s are the same
// from left to right
// lb 246515 739105
// this module is proof i do not know how integers work in rust yet

type Password = [i8;6];    //this is just an alias, we can't add methods to this as in a struct
                            // could do struct Password ([i8;6]) or 
                            // struct Password(i8, i8, i8, i8, i8, i8)

fn check_password (password: Password)-> bool {
    let mut biggest = &password[0];
    let mut consecutive_digits:HashMap<i8, i8> = HashMap::new();

    for (i,x) 
    in password
        .iter()
        .enumerate(){
        if i>0 && x==&password[i-1]{
            //consecutive digit found.
            let count = consecutive_digits
                .entry(*x) //return the hash map value for key x
                .or_insert(0); // or initialize the value at key x to 0, then return it to count
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
        .map(|&x| x == 1__i8) // check if that value was unique
        .fold(false, |result, x| result || x) // check if any of those comparisons were true
}

fn number_to_password(num: i32) -> Password {
    let mut password:Password = [0,0,0,0,0,0];
    // magic #s are good, actually
    for i in 1..7{
        password[6-i] = ((num % 10_i32.pow(i as u32) ) / 10_i32.pow((i-1) as u32)) as i8;
    }
    password
}
fn main(){
    let lower_bound = 246515;
    let upper_bound = 739105;
    
    let valid_passwords = (lower_bound..(upper_bound + 1))
        .map(|p| check_password(number_to_password(p)))
        .filter(|x| *x)
        .count();
    
    println!("{:?}",valid_passwords);
}
