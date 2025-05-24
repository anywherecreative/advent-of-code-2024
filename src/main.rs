use std::fs;
use std::fs::read_to_string;
// use std::io::BufRead;
use regex::Regex;

fn main() {
    // day1();
    // day2();
    // println!("Day 3 P1: {}",day3(read_to_string("inputs/day3.txt").unwrap()));
    // println!("Day 3 P2: {}",day3_part2(read_to_string("inputs/day3.txt").unwrap()));
    // let input_order_rules: String = fs::read_to_string("inputs/day5_order_rules.txt.txt").unwrap();
    // let input_manuals: String = fs::read_to_string("inputs/day5_manuals.txt").unwrap();
    // println!("Day 5 P1: {}",day5(input_order_rules,input_manuals));
    
    // let file_content =  fs::read_to_string("inputs/day5_part2.txt").unwrap();
    // let day5_input= file_content.split("\n\n").collect::<Vec<&str>>();
    // let (input_order_rules, input_manuals) = day5_input.split_at(1);
    // let result = day5_part2(input_order_rules[0].to_string(),input_manuals[0].to_string());
    // println!("Result: {}", result);
    
    let result = day6(fs::read_to_string("inputs/day6.txt").unwrap());
    println!("Result: {}", result);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn check_line_day_2(res: &Vec<&str>) -> bool {
    let mut previous_number = 0;
    let mut increasing: bool = false;
    for (index, num) in res.iter().enumerate() {
        let num = num.parse::<i32>().unwrap();
        if index == 0 {
            continue;
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn day3_part2(input: String) -> i32 {
    //break the string
    let formatted_input  = ("do()".to_owned() + &*input + "don't()").replace('\n', "").replace('\r', "");
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

#[allow(dead_code)]
fn day5(input_order_rules: String, input_manuals: String) -> i32 {
    let mut output = 0;
    for manual in input_manuals.lines() {
        let mut passing = true;
        for rule in input_order_rules.lines() {
            let (left, right) = split_to_tuple(rule, '|');
            if let Some(left_num) = manual.find(left) {
                if let Some(right_num) = manual.find(right) {
                    if left_num > right_num {
                        //get half the length
                        passing = false;
                        break; //fail
                    }
                }
            }
        }

        if passing {
            let parts = manual.split(",").collect::<Vec<&str>>();
            output += parts[parts.len()/2].parse::<i32>().unwrap()
        }
    }

    output.try_into().unwrap()
}

fn day5_part2(input_order_rules: String, input_manuals: String) -> i32 {

    let mut output = 0;
    let mut manuals: Vec<Vec<&str>> = Vec::new();
    for (i,manual) in input_manuals.lines().into_iter().enumerate() {


        // let mut passing = true;
       manuals.push(manual.split(",").collect::<Vec<&str>>());

        let mut changed = true;
        let mut add = false;
        while changed == true {
            changed = false;
            for rule in input_order_rules.lines() {
                let (left, right) = split_to_tuple(rule, '|');

                if let Some(left_num) = manuals[i].iter().position(|&l| l == left) {
                    if let Some(right_num) = manuals[i].iter().position(|&r| r == right) {
                        if left_num > right_num {
                            let left = manuals[i][left_num];
                            manuals[i][left_num] = manuals[i][right_num];
                            manuals[i][right_num] = left;
                            changed = true;
                            add = true;
                        }
                    }
                }
            }
        }


        if add == true {
             output += manuals[i][(manuals[i].len() / 2 + (manuals[i].len() % 2))-1].parse::<i32>().unwrap()
        }
    }

    output.try_into().unwrap()
}

fn split_to_tuple(s: &str, delimiter: char) -> (&str, &str) {
    let mut parts = s.split(delimiter);
    let left = parts.next().unwrap_or("");
    let right = parts.next().unwrap_or("");
    (left, right)
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
enum Tiles {
    Obstacle,
    Guard(Direction),
    Empty,
    Entered,
}

struct Guard {
    direction: Direction,
    position: (usize, usize),
    on_screen: bool,
}

fn day6(input: String) -> i32 {
    let mut guard: Guard = Guard {
        direction: Direction::North,
        position: (0,0),
        on_screen: false,
    };

    let mut level: Vec<Vec<Tiles>> = Vec::new();

    for (i,row) in input.lines().into_iter().enumerate() {
        let mut row_vec: Vec<Tiles> = Vec::new();
        for (j,tile) in row.chars().enumerate() {
            if tile == '.' {
                row_vec.push(Tiles::Empty);
            }
            else if tile == '#' {
                row_vec.push(Tiles::Obstacle);
            }
            else if tile == '^' {
                guard = Guard {
                    direction: Direction::North,
                    position: (i, j),
                    on_screen: true,
                };
                row_vec.push(Tiles::Guard(Direction::North));
            }
            else if tile == '>' {
                guard = Guard {
                    direction: Direction::East,
                    position: (i, j),
                    on_screen: true,
                };
                row_vec.push(Tiles::Guard(Direction::East));
            }
            else if tile == 'v' {
                guard = Guard {
                    direction: Direction::South,
                    position: (i, j),
                    on_screen: true,
                };
                row_vec.push(Tiles::Guard(Direction::South));
            }
            else {
                guard = Guard {
                    direction: Direction::West,
                    position: (i, j),
                    on_screen: true,
                };
                row_vec.push(Tiles::Guard(Direction::West));
            }

        }
        level.push(row_vec);
    }

    while guard.on_screen == true {
        match guard.direction {
            Direction::North => {
                if guard.position.0 == 0 {
                    guard.on_screen = false;
                    break;
                }
                if level[guard.position.0-1][guard.position.1] == Tiles::Obstacle {
                    guard.direction = Direction::East;
                    continue;
                }
                level[guard.position.0-1][guard.position.1] = Tiles::Guard(Direction::North);
                level[guard.position.0][guard.position.1] = Tiles::Entered;
                guard.position.0 -= 1;
            }
            Direction::East => {
                if guard.position.1 == level[0].len() - 1 {
                    guard.on_screen = false;
                    break;
                }
                if level[guard.position.0][guard.position.1+1] == Tiles::Obstacle {
                    guard.direction = Direction::South;
                    continue;
                }
                level[guard.position.0][guard.position.1+1] = Tiles::Guard(Direction::East);
                level[guard.position.0][guard.position.1] = Tiles::Entered;
                guard.position.1 += 1;
            }
            Direction::South => {
                if guard.position.0 == level.len() - 1 {
                    guard.on_screen = false;
                    break;
                }
                if level[guard.position.0+1][guard.position.1] == Tiles::Obstacle {
                    guard.direction = Direction::West;
                    continue;
                }
                level[guard.position.0+1][guard.position.1] = Tiles::Guard(Direction::South);
                level[guard.position.0][guard.position.1] = Tiles::Entered;
                guard.position.0 += 1;
            }
            Direction::West => {
                if guard.position.1 == 0 {
                    guard.on_screen = false;
                    break;
                }
                if level[guard.position.0][guard.position.1-1] == Tiles::Obstacle {
                    guard.direction = Direction::North;
                    continue;
                }
                level[guard.position.0][guard.position.1-1] = Tiles::Guard(Direction::West);
                level[guard.position.0][guard.position.1] = Tiles::Entered;
                guard.position.1 -= 1;
            }
        }
    }

    let mut tiles_covered = 0;
    for row in &level {
        for tile in row {
            if let &Tiles::Entered | &Tiles::Guard(_) = tile {
                tiles_covered += 1;
            }
        }
        println!();
    }

    tiles_covered
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
    fn test_day5() {
        let file_content =  fs::read_to_string("inputs/tests/day5_order_rules.txt").unwrap();
        let day5_input= file_content.split("\n\n").collect::<Vec<&str>>();
        let (input_order_rules, input_manuals) = day5_input.split_at(1);
        let result = day5(input_order_rules[0].to_string(),input_manuals[0].to_string());
        assert_eq!(result, 143);
    }

    #[test]
     fn test_day5_part2() {
        let file_content =  fs::read_to_string("inputs/tests/day5_order_rules.txt").unwrap();
        let day5_input= file_content.split("\n\n").collect::<Vec<&str>>();
        let (input_order_rules, input_manuals) = day5_input.split_at(1);
        let result = day5_part2(input_order_rules[0].to_string(),input_manuals[0].to_string());
        assert_eq!(result, 123);
    }

    #[test]
    fn test_day6() {
        let input: String = fs::read_to_string("inputs/tests/day6.txt").unwrap();
        let result = day6(input);
        assert_eq!(result, 41);
    }
}