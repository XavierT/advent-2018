

pub fn run(){
    println!("day1 is running!");

    let content = include_str!("../input/day1.txt");

    let vec:Vec<_> = content.lines().collect();

    let mut ivec:Vec<i32> = Vec::new();

    for s in vec.iter()
    {
        ivec.push( s.parse().unwrap());
    }

    let sum:i32 = ivec.iter().sum();

    println!(" The sum of the vector is {}",sum);
}