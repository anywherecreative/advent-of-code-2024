use std::fs::read_to_string;
use regex::Regex;

fn main() {
    day1();
    day2();
    println!("Day 3 P1: {}",day3(read_to_string("inputs/day3.txt").unwrap()));
    println!("Day 3, P2: {}",day3_part2(read_to_string("inputs/day3.txt").unwrap()));
    println!("Day 4, P1: {}",day4(read_to_string("inputs/tests/day4.txt").unwrap()));
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

fn day2() {
    let mut total_safe: i32 = 0;
    let mut total_safe_p2: i32 = 0;
    for line in read_to_string("inputs/day2.txt").unwrap().lines() {
        let res: Vec<&str> = line.split(" ").collect();
        let v1 = check_line_day_2(&res);
        let mut v2 = false;
        if !v1 {
            for (index, _) in res.iter().enumerate() {
                let mut new_res = res.clone();
                new_res.remove(index);
                if check_line_day_2(&new_res) {
                    v2 = true;
                    break;
                }
            }
        }

        if v1 {
            total_safe +=1;
            total_safe_p2 +=1;
        }
        if v2 {
            total_safe_p2 +=1;
        }
    }

    println!("Day 2 P1: {}", total_safe);
    println!("Day 2 P2: {}", total_safe_p2);
}

fn check_day_2(previous: i32, current:i32, increasing: bool) -> bool{
    if previous > current && increasing {
        return false
    } else if previous < current && !increasing {
        return false
    } else if previous == current {
        return false
    }

    if (previous - current).abs() > 3 {
        return false
    }

    true
}

fn check_line_day_2(res: &Vec<&str>) -> bool {
    let mut previous_number = 0;
    let mut increasing: bool = false;
    for (index, num) in res.iter().enumerate() {
        let num = num.parse::<i32>().unwrap();
        if index == 0 {
            previous_number = num;
        } else if index == 1 {
            if previous_number > num {
                increasing = false;
            } else if previous_number < num {
                increasing = true;
            } else {
                return false;
            }

            if (previous_number - num).abs() > 3 {
                return false;
            }
        } else {
            if !check_day_2(previous_number,num,increasing) {
                return false;
            }
        }

        previous_number = num;
    }

    true
}

fn day3(input: String) -> i32 {
    let mut output = 0;
    let instructions_match = Regex::new(r"mul\([0-9]*\,[0-9]*\)").unwrap();
    let numbers_match = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    for instruction in instructions_match.captures_iter(input.as_str()) {
        let raw_numbers = instruction.get(0).unwrap().as_str();

        let captures = numbers_match.captures(raw_numbers).unwrap();
        let first_number = captures.get(1).unwrap().as_str();
        let second_number = captures.get(2).unwrap().as_str();

        output += first_number.parse::<i32>().unwrap() * second_number.parse::<i32>().unwrap();

    }
    output
}

fn day3_part2(input: String) -> i32 {
    //break the string
    let mut formatted_input  = ("do()".to_owned() + &*input + "don't()").replace('\n', "").replace('\r', "");;
    let mut output = 0;
    let instructions_match = Regex::new(r"mul\([0-9]*\,[0-9]*\)").unwrap();
    let numbers_match = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let control_match = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();

    for allowed in control_match.captures_iter(formatted_input.as_str()) {
        let res = allowed.get(0).unwrap().as_str();
        for instruction in instructions_match.captures_iter(res) {
            let raw_numbers = instruction.get(0).unwrap().as_str();

            let captures = numbers_match.captures(raw_numbers).unwrap();
            let first_number = captures.get(1).unwrap().as_str();
            let second_number = captures.get(2).unwrap().as_str();

            output += first_number.parse::<i32>().unwrap() * second_number.parse::<i32>().unwrap();

        }
    }

    output
}

fn day4(input: String) -> i32 {
    let mut output = 0;
    let line_length = input.find('\n').unwrap();
    let mut formatted_input  =input.replace('\n', "").replace('\r', "").to_lowercase();
    let file_length = formatted_input.chars().count();

    for (index, letter) in formatted_input.chars().enumerate() {
        //only check for x characters
        if letter != 'x' {
            continue;
        }

        //check to the right
        //got to check if we've exceeded line length
        // use modulus operator? line mod index
        if ((index % line_length) + 4) < line_length &&  ((index % line_length) + 4) < file_length {
            if formatted_input[index..index+4].to_string() == "xmas" {
                output+=1;
            }

        }
        if (index % line_length) >= 3 {
            if formatted_input[index-3..index+1].to_string() == "samx" {
                println!("run");
                output+=1;
            }
        }

        //check down
        let xmas_last_index = (index % line_length) + (line_length * 3);
        if xmas_last_index < file_length {
            let check = "x" + formatted_input[index + line_length] + formatted_input[index + (line_length * 2)] + formatted_input[index + (line_length * 3)];
            if(check == "xmas") {
                output+=1;
            }
        }

        //check up

        //check diagonal right

        //check diagonal left

    }
    output
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day3() {
        let input: String = fs::read_to_string("inputs/tests/day3.txt").unwrap();
        let result = day3(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_day3_part2() {
        let input: String = fs::read_to_string("inputs/tests/day3.txt").unwrap();
        let result = day3_part2(input);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_day4() {
        let input: String = fs::read_to_string("inputs/tests/day4.txt").unwrap();
        let result = day4(input);
        assert_eq!(result, 18);
    }
}