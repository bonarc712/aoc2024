fn get_amount_of_safe_reports(input: &str) -> i32 {

    let reports = create_reports(input);

    let safe_reports_amount = read_reports(reports);

    return safe_reports_amount;
}

fn read_line_of_strings(line: &str) -> Vec<i32> {
    return line
        .split(' ')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
}

fn create_reports(input: &str) -> Vec<Vec<i32>> {

    let mut report_list: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let report = read_line_of_strings(line);
        report_list.push(report);
    }

    return report_list;
}

fn read_reports(report_list: Vec<Vec<i32>>) -> i32 {

    let mut amount_of_safe_reports = 0;

    for report in report_list.iter() {
        let result = is_report_safe(report);
        if result {
            amount_of_safe_reports += 1;
        }
    }

    return amount_of_safe_reports;
}

fn is_report_safe(report: &Vec<i32>) -> bool {

    if report.len() == 0 || report.len() == 1 {
        return true
    }
    
    let mut previous_number: Option<i32> = None;
    let mut is_ascending: Option<bool> = None;
    let mut asc_count = 0;
    let mut desc_count = 0;

    for number in report.iter() {
        match previous_number {
            None => {
                previous_number = Some(*number);
                continue;
            }
            Some(prev_number) => {
                if *number > prev_number {
                    asc_count += 1;
                }
                else if *number < prev_number {
                    desc_count += 1;
                }
            }
        }
        previous_number = Some(*number);
    }
    
    //println!("Asc count {} and desc count {}", asc_count, desc_count);
    is_ascending = Some(asc_count > desc_count);

    let mut bad_indices: Vec<usize> = Vec::new();
    for i in 0..report.len() - 1 {

        match is_ascending {
            None => panic!(),
            Some(asc) => {
                let first_number = report[i];
                let second_number = report[i+1];

                if asc && first_number > second_number {
                    bad_indices.push(i);
                    bad_indices.push(i+1);
                }
                else if !asc && second_number > first_number {
                    bad_indices.push(i);
                    bad_indices.push(i+1);
                }

                let mut difference = second_number - first_number;
                difference = difference.abs();

                if difference < 1 || difference > 3 {
                    bad_indices.push(i);
                    bad_indices.push(i+1);
                }
            }
        }
    }

    if bad_indices.len() == 0 {
        return true;
    }

    if bad_indices.len() > 4 {
        return false;
    }

    
    println!("{:?}", report);
    println!("{:?}", bad_indices);
    for i in 0..bad_indices.len() {
        let mut report_copy = report.to_vec();
        report_copy.remove(bad_indices[i]);
        println!("Removing index {} ", bad_indices[i]);
        println!("New report {:?}", report_copy);
        if is_report_safe_strict(&report_copy, is_ascending.unwrap()) {
            return true;
        }
    }
    return false;
}

fn is_report_safe_strict(report: &Vec<i32>, asc: bool) -> bool {
    for i in 0..report.len() - 1 {

        let first_number = report[i];
        let second_number = report[i+1];

        if asc && first_number > second_number {
            return false;
        }
        else if !asc && second_number > first_number {
            return false;
        }

        let mut difference = second_number - first_number;
        difference = difference.abs();

        if difference < 1 || difference > 3 {
            return false;
        }
    }
    println!("Some report works");
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn given_safe_with_bad_level_1() {
        let report = vec![1, 2, 3, 4, 4];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_2() {
        let report = vec![1, 4, 7, 7, 10];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_3() {
        let report = vec![1, 4, 7, 5, 10];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_4() {
        let report = vec![1, 3, 2, 4, 5];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_5() {
        let report = vec![1, 3, 2, 6, 8];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_6() {
        let report = vec![1, 3, 2, 3, 6];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_7() {
        let report = vec![10, 1, 2, 3, 4];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_safe_with_bad_level_8() {
        let report = vec![1, 6, 4, 2, 1];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    #[test]
    fn given_safe_with_bad_level_9() {
        let report = vec![9, 8, 7, 6, 1];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_not_safe_with_two_bad_levels1() {
        let report = vec![59, 61, 63, 66, 64, 66, 66];

        let result = is_report_safe(&report);

        assert_eq!(false, result);
    }

    //#[test]
    fn given_a_safe_ascending_report_if_safe_then_report_is_safe() {
        let report = vec![1, 3, 6, 7, 9];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_a_report_with_one_element_if_safe_then_report_is_safe() {
        let report = vec![6];

        let result = is_report_safe(&report);

        assert_eq!(true, result);
    }

    //#[test]
    fn given_a_line_reading_line_should_return_numbers_in_vec() {
        let input = "7 6 4 2 1";

        let result = read_line_of_strings(input);

        assert_eq!(result, [7, 6, 4, 2, 1]);
    }

    //#[test]
    fn given_two_lines_report_list_is_created() {
        let input = "7 6
1 2";

        let result = create_reports(input);

        assert_eq!(result, [[7, 6], [1, 2]]);
    }

    //#[test]
    fn given_example_two_reports_are_safe() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = get_amount_of_safe_reports(input);
        
        assert_eq!(result, 4);
    }
}
