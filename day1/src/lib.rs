fn calculate_distance_between_lists(input: &str) -> i32 {

    let lists = create_santas_lists(input);

    let mut sum = 0;
    for i in 0..lists.left_list.len() {
        let difference = lists.left_list[i] - lists.right_list[i];
        sum += difference.abs();
    }

    return sum
}

fn calculate_similarity_between_lists(input: &str) -> i32 {

    let lists = create_santas_lists(input);

    let mut similarity_sum = 0;
    for element in &lists.left_list {
        let mut count = 0;
        for other_element in &lists.right_list {
            if element == other_element {
                count = count + 1;
            }
        }
        similarity_sum += element * count;
    }

    return similarity_sum;
}

fn create_santas_lists(input: &str) -> SantasLists {

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let pair_of_numbers: Numbers = read_line_of_strings(line);
        left_list.push(pair_of_numbers.first_number);
        right_list.push(pair_of_numbers.second_number);
    }

    left_list.sort();
    right_list.sort();

    println!("bob {}", left_list[0].to_string());

    return SantasLists {left_list, right_list};
}

struct SantasLists {
    left_list: Vec<i32>,
    right_list: Vec<i32>
}

struct Numbers {
    first_number: i32,
    second_number: i32
}

fn read_line_of_strings(line: &str) -> Numbers {
    let position_of_first_space = line.find(' ').unwrap();
    let number_for_first_list = &line[..position_of_first_space];
    let number_for_second_list = &line[position_of_first_space..].trim();

    return Numbers {first_number: number_for_first_list.parse().unwrap(), second_number: number_for_second_list.parse().unwrap()};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result_is_eleven() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let distance_between_lists = calculate_distance_between_lists(input);

        assert_eq!(distance_between_lists, 11);
    }

    #[test]
    fn example_similarity_is_thirtyone() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let similarity = calculate_similarity_between_lists(input);

        assert_eq!(similarity, 31);
    }
}
