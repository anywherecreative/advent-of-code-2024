use std::fs::read_to_string;

fn main() {
    day1();
    day2();
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

    output
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn day_3() {
        let input: String = fs::read_to_string("inputs/test/day3.txt").unwrap();
        let result = day3(input);
        assert_eq!(result, 161);
    }
}