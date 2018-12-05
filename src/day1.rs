

pub fn run() {
    println!("day1 is running!");

    let content = include_str!("../input/day1.txt");

    let vec: Vec<_> = content.lines().collect();

    let mut ivec: Vec<i32> = Vec::new();

    for s in vec.iter() {
        ivec.push(s.parse().unwrap());
    }

    let sum: i32 = ivec.iter().sum();

    // solution for Part 1
    println!(" The sum of the vector is {}", sum);


    // Part 2
    // Do the sum and store the intermediate result in a hashmap, with the partial sum as index
    // 1 if the sum happened once
    // 2 if the sum happened twice
    // Stopped when the first 2 is reached
}
