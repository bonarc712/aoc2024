#[derive(Clone)]
struct PageOrder {
    page_before: String,
    page_after: String
}

fn read_page_numbers(line: &str) -> PageOrder {
    let mut numbers_in_line = line.split("|");
    return PageOrder { page_before : String::from(numbers_in_line.next().unwrap()), 
        page_after : String::from(numbers_in_line.next().unwrap()) }
}

fn read_page_number_list(line: &str) -> Vec<&str> {
    let mut numbers_vector = Vec::new();
    let mut numbers_in_line = line.split(",");

    loop {
        let next_number = numbers_in_line.next();
        match next_number {
            Some(number) => {
                numbers_vector.push(number)
            }
            None => return numbers_vector
        }
    }
}

fn get_middle_page_number(line: &str) -> i32 {
    let numbers = read_page_number_list(line);
    let middle_index = (numbers.len() - 1) / 2;

    return numbers[middle_index].parse::<i32>().unwrap();
}

fn get_middle_page_number_vec(line: Vec<&str>) -> i32 {
    let middle_index = (line.len() - 1) / 2;

    return line[middle_index].parse::<i32>().unwrap();
}

fn get_sum_of_correct_page_lists(input: &str) -> i32 {
    let mut rule_mode = true;
    let mut page_order_rules = Vec::new();
    let mut number_list_vector = Vec::new();

    for line in input.lines() {
        if line.trim() == "" { // Empty
            rule_mode = false;
            continue;
        }
        if rule_mode { // Read rule
            page_order_rules.push(read_page_numbers(line))
        }
        else { // Number rule
            number_list_vector.push(read_page_number_list(line));
        }
    }

    let mut sum = 0;
    let mut incorrectly_ordered_updates = Vec::new();

    //println!("Number list : {:?}", number_list_vector);
    for number_list in number_list_vector {
        if !test_numbers_for_rules(page_order_rules.clone(), number_list.clone()) {
            //sum += get_middle_page_number_vec(number_list)
            incorrectly_ordered_updates.push(number_list);
        }
    }

    for mut update in incorrectly_ordered_updates {
        let mut sorted_update = bubble_sort_pages(page_order_rules.clone(), &mut update);
        sum += get_middle_page_number_vec(sorted_update.to_vec());
    }

    return sum;
}

fn test_numbers_for_rules(rules: Vec<PageOrder>, number_list: Vec<&str>) -> bool {
    for i in 0..number_list.len()-1 {
        for j in i+1..number_list.len() {
            let first_number = number_list[i];
            let second_number = number_list[j];

            for page_order in rules.iter() {
                if page_order.page_after == first_number && page_order.page_before == second_number {
                    return false;
                }
            }
        }
    }
    return true;
}

fn bubble_sort_pages<'a>(rules: Vec<PageOrder>, number_list: &'a mut Vec<&'a str>) -> &'a mut Vec<&'a str> {
    for i in 0..number_list.len()-1 {
        for j in i+1..number_list.len() {
            for page_order in rules.iter() {
                if page_order.page_after == number_list[i] && page_order.page_before == number_list[j] {
                    number_list.swap(i, j);
                }
            }
        }
    }

    return number_list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_page_numbers_should_be_read_correctly() {
        let input = "1|2";

        let result = read_page_numbers(input);

        assert_eq!(result.page_before, "1");
        assert_eq!(result.page_after, "2");
    }

    #[test]
    fn given_page_number_list_should_be_read_correctly() {
        let input = "1,2,3";

        let result = read_page_number_list(input);

        assert_eq!(result[1], "2");
    }

    #[test]
    fn given_page_number_list_get_middle_page_as_integer() {
        let input = "5,1,3,0,5,2,1,6,4";

        let result = get_middle_page_number(input);

        assert_eq!(result, 5);
    }

    #[test]
    fn given_page_number_list_as_vector_get_middle_page_as_integer() {
        let mut input = Vec::new();
        input.push("4");
        input.push("2");
        input.push("5");
        input.push("0");
        input.push("1");

        let result = get_middle_page_number_vec(input);

        assert_eq!(result, 5);
    }

    #[test]
    fn given_some_rules_an_incorrect_number_list_is_marked_as_incorrect() {
        let mut rules_input = Vec::new();
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("2") });

        let mut numbers_input = Vec::new();
        numbers_input.push("2");
        numbers_input.push("1");

        let result = test_numbers_for_rules(rules_input, numbers_input);

        assert_eq!(result, false);
    }

    #[test]
    fn given_an_incorrectly_sorted_list_then_is_sorted_correctly() {
        let mut rules_input = Vec::new();
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("2") });
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("3") });
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("4") });
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("5") });
        rules_input.push(PageOrder { page_before : String::from("2"), page_after : String::from("3") });
        rules_input.push(PageOrder { page_before : String::from("2"), page_after : String::from("4") });
        rules_input.push(PageOrder { page_before : String::from("2"), page_after : String::from("5") });
        rules_input.push(PageOrder { page_before : String::from("3"), page_after : String::from("4") });
        rules_input.push(PageOrder { page_before : String::from("3"), page_after : String::from("5") });
        rules_input.push(PageOrder { page_before : String::from("4"), page_after : String::from("5") });

        let mut numbers_input = Vec::new();
        numbers_input.push("4");
        numbers_input.push("5");
        numbers_input.push("1");
        numbers_input.push("3");
        numbers_input.push("2");

        let result = bubble_sort_pages(rules_input, &mut numbers_input);

        let mut numbers_expected = Vec::new();
        numbers_expected.push("1");
        numbers_expected.push("2");
        numbers_expected.push("3");
        numbers_expected.push("4");
        numbers_expected.push("5");

        assert_eq!(result, &mut numbers_expected);
    }

    #[test]
    fn given_some_rules_a_correct_number_list_is_marked_as_correct() {
        let mut rules_input = Vec::new();
        rules_input.push(PageOrder { page_before : String::from("1"), page_after : String::from("2") });

        let mut numbers_input = Vec::new();
        numbers_input.push("1");
        numbers_input.push("2");

        let result = test_numbers_for_rules(rules_input, numbers_input);

        assert_eq!(result, true);
    }

    #[test]
    fn given_an_input_it_should_return_correct_value() {
        let input = "1|2
1|3
2|3

1,3,2";

        let result = get_sum_of_correct_page_lists(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn given_example_should_give_correct_number() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = get_sum_of_correct_page_lists(input);

        assert_eq!(result, 123);
    }
}
