


fn main(){
let mut intcode: [usize;121] = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,2,19,6,23,2,13,23,27,1,9,27,31,2,31,9,35,1,6,35,39,2,10,39,43,1,5,43,47,1,5,47,51,2,51,6,55,2,10,55,59,1,59,9,63,2,13,63,67,1,10,67,71,1,71,5,75,1,75,6,79,1,10,79,83,1,5,83,87,1,5,87,91,2,91,6,95,2,6,95,99,2,10,99,103,1,103,5,107,1,2,107,111,1,6,111,0,99,2,14,0,0];

for i in 0..121{ 
    if i % 4 == 0{
        let val = match intcode[i] {
        1 => intcode[intcode[i+1]] + intcode[intcode[i+2]],
        2 => intcode[intcode[i+1]] * intcode[intcode[i+2]],
        99 => break,
        _ => panic!("{}, intcode[i]")
        };
        println!("setting pos {} to {}", intcode[i+3], val);
        intcode[intcode[i+3]] = val;

    }
}
println!("{:?}", intcode[0]);
}
