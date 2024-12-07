use regex::Regex;

pub fn mul(left: i32, right: i32) -> i32 {
    left * right
}

pub fn find_first_pattern_in_string(input_string: &str) -> Option<&str> {
    let actual_regex = Regex::new("mul\\([0-9]{1,9},[0-9]{1,9}\\)|do\\(\\)|don't\\(\\)").unwrap();

    let captures = actual_regex.captures(input_string);

    return match captures {
        None => None,
        Some(i) => Some(i.get(0).unwrap().as_str())
    };
}

pub fn find_all_patterns(input: &str) -> Vec<&str> {
    let mut patterns: Vec<&str> = Vec::new();
    let mut input_copy = input;

    loop {
        match find_first_pattern_in_string(input_copy) {
            None => break,
            Some(pattern) => {
                patterns.push(pattern);
                input_copy = &input_copy[input_copy.find(pattern).unwrap()+pattern.len()..];
            }
        }
    }

    return patterns;
}

pub fn calculate_result_of_multiplications(input: &str) -> i32 {
    let patterns = find_all_patterns(input);
    let mut sum = 0;
    let mut is_multiplying_active: bool = true;

    for pattern in patterns.into_iter() {

        if (pattern == "do()") {
            is_multiplying_active = true;
            continue;
        }
        if (is_multiplying_active) {
            if (pattern == "don't()") {
                is_multiplying_active = false;
                continue;
            }
        }
        else {
            continue;
        }

        //println!("Pattern: {:?}", pattern);

        let left_number_as_str = &pattern[pattern.find("(").unwrap()..pattern.find(",").unwrap()];
        //println!("Left number as string : {}", left_number_as_str);
        let left_number = pattern[pattern.find("(").unwrap()+1..pattern.find(",").unwrap()].parse().unwrap();
        //println!("Right number as string : {}", &pattern[pattern.find(",").unwrap()..pattern.find(")").unwrap()]);
        let right_number = pattern[pattern.find(",").unwrap()+1..pattern.find(")").unwrap()].parse().unwrap();

        sum += mul(left_number, right_number);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_works() {
        let result = mul(3, 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn string_containing_mul_returns_first_one() {
        let input = "xmul(2,4)%";

        let result = find_first_pattern_in_string(input);

        assert_eq!(Some("mul(2,4)"), result);
    }
    
    #[test]
    fn string_containing_mul_returns_first_one_2() {
        let input = "do_not_mul(55,5)";

        let result = find_first_pattern_in_string(input);

        assert_eq!(Some("mul(55,5)"), result);
    }
    
    #[test]
    fn string_containing_bad_mul_returns_none() {
        let input = "mul[3,7]";

        let result = find_first_pattern_in_string(input);

        assert_eq!(None, result);
    }
    
    #[test]
    fn string_containing_bad_mul_returns_none_2() {
        let input = "mul(32,64]";

        let result = find_first_pattern_in_string(input);

        assert_eq!(None, result);
    }

    #[test]
    fn string_containing_nothing_returns_nothing() {
        let input = "bob";

        let result = find_first_pattern_in_string(input);

        assert_eq!(None, result);
    }

    #[test]
    fn string_with_many_patterns_find_all_patterns() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)";

        let result = find_all_patterns(input);

        assert_eq!(result, ["mul(2,4)", "mul(5,5)"]);
    }

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = calculate_result_of_multiplications(input);

        assert_eq!(48, result);
    }
}
