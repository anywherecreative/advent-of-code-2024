use std::fs::read_to_string;

fn main() {
    day1();
}

fn day1() {
    let mut left : Vec<i32> = Vec::new();
    let mut right : Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    let mut p2_total : i32 = 0;

    for line in read_to_string("inputs/day1.txt").unwrap().lines() {
        let res: Vec<&str> = line.split("   ").collect();
        left.push(res.get(0).unwrap().parse::<i32>().unwrap());
        right.push(res.get(1).unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    for (index, value) in left.iter().enumerate() {
        total = total + (value - right[index]).abs();
        for v2 in right.iter() {
            if value == v2 {
                p2_total = p2_total + value;
            }
        }
    }

    println!("Day 1 P1: {}",total);
    println!("Day 1 P2: {}",p2_total);
}
