use std::ptr::eq;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn concatenate(left: u64, right: u64) -> u64 {
    let mut left_string = left.to_string();
    let right_string = right.to_string();

    left_string.push_str(&right_string);

    return left_string.parse::<u64>().unwrap();
}

#[derive(Clone, Debug, PartialEq)]
struct Equation {
    result: u64,
    members: Vec<u64>,
}

pub fn read_input(input: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let eq_result = line.chars()
            .take_while(|&ch| ch != ':')
            .collect::<String>();

        let result_as_u64 = eq_result.parse::<u64>().unwrap();

        let members = line.split(' ')
            .skip_while(|&string| string.contains(':'))
            .map(|member| member.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        equations.push(Equation {result: result_as_u64, members});
    }

    return equations;
}

pub fn calculate_one_equation(members : &Vec<u64>) -> Vec<u64> {
    let mut results = Vec::new();

    for member in members {
        if results.is_empty() {
            results.push(*member);
            continue;
        }

        let mut results_for_this_member = Vec::new();

        for result in results {
            results_for_this_member.push(add(result, *member));
            results_for_this_member.push(multiply(result, *member));
            results_for_this_member.push(concatenate(result, *member));
        }
        results = results_for_this_member;
    }

    return results;
}

pub fn calculate_calibration(input: &str) -> u64 {
    let equations = read_input(input);
    let mut valid_equations: Vec<Equation> = Vec::new();

    for equation in equations {
        let results = calculate_one_equation(&equation.members);
        for result in results {
            if equation.result == result {
                valid_equations.push(equation.clone());
                break;
            }
        }
    }

    return valid_equations.iter().map(|equation| equation.result).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn multiply_works() {
        let result = multiply(2, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn concatenate_works() {
        let result = concatenate(2, 3);
        assert_eq!(result, 23);
    }

    #[test]
    fn reading_equation_returns_the_equation() {
        let input = "3267: 81 40 27";

        let result = read_input(input);

        assert_eq!(Equation { result: 3267 as u64, members: vec![81, 40, 27] }, result[0]);
    }

    #[test]
    fn calculating_simple_equation_returns_all_possibilities() {
        let equation_members: Vec<u64> = vec![1, 2, 3];

        let result = calculate_one_equation(&equation_members);

        assert_eq!(vec![6, 9, 33, 5, 6, 23, 15, 36, 123], result);
    }

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = calculate_calibration(input);

        assert_eq!(result, 11387);
    }
}
