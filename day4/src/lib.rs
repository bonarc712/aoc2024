use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

pub fn find_patterns_xmas(input: &str) -> i32 {
    let lines = read_all_lines(input);
    let mut total_patterns = 0;
    let mut all_as_in_mases: Vec<APosition> = Vec::new();

    for i in 0..lines.len() {
        let current_sub_vec: Vec<char> = lines.get(i).unwrap().clone();

        for j in 0..current_sub_vec.len() {
            if current_sub_vec[j] == 'M' {
                for direction in Direction::iter() {
                    let element = get_element_in_direction_with_step(direction, j, i, lines.clone(), 1);
                    if element == Some('A') {
                        let element2 = get_element_in_direction_with_step(direction, j, i, lines.clone(), 2);
                        if element2 == Some('S') {
                            let aposition = get_position_of_a(direction, j, i);
                            if all_as_in_mases.contains(&aposition) {
                                total_patterns += 1;
                            }
                            else {
                                all_as_in_mases.push(aposition);
                            }
                        }
                    }
                }
            }
        }
    }



    return total_patterns;
}

pub fn read_line(line: &str) -> Vec<char> {
    return line.chars().collect();
}

pub fn read_all_lines(lines: &str) -> Vec<Vec<char>> {
    let mut lines_vec = Vec::new();
    for line in lines.lines() {
        lines_vec.push(read_line(line));
    }
    return lines_vec;
}

pub fn get_element_in_direction(dir: Direction, pivot_x: usize, pivot_y: usize, lines: Vec<Vec<char>>) -> Option<char> {
    return get_element_in_direction_with_step(dir, pivot_x, pivot_y, lines, 1)
}

pub fn get_element_in_direction_with_step(dir: Direction, pivot_x: usize, pivot_y: usize, lines: Vec<Vec<char>>, step: i32) -> Option<char> {
    let pivot_x_as_i32: i32 = pivot_x as i32;
    let pivot_y_as_i32: i32 = pivot_y as i32;
    let mut vertical_diff = 0;
    let mut horizontal_diff = 0;
    match dir {
        //Direction::North => vertical_diff = -step,
        Direction::NorthEast => {
            vertical_diff = -step;
            horizontal_diff = step;
        },
        //Direction::East => horizontal_diff = step,
        Direction::SouthEast => {
            vertical_diff = step;
            horizontal_diff = step;
        },
        //Direction::South => vertical_diff = step,
        Direction::SouthWest =>  {
            vertical_diff = step;
            horizontal_diff = -step;
        },
        //Direction::West => horizontal_diff = -step,
        Direction::NorthWest =>  {
            vertical_diff = -step;
            horizontal_diff = -step;
        },
    }

    if pivot_y_as_i32 + vertical_diff < 0 || pivot_y_as_i32 + vertical_diff >= lines.len() as i32 {
        return None
    }

    if pivot_x_as_i32 + horizontal_diff < 0 || pivot_x_as_i32 + horizontal_diff >= lines.get(0).unwrap().len() as i32 {
        return None
    }

    let target_y = (pivot_y_as_i32 + vertical_diff) as usize;
    let target_x = (pivot_x_as_i32 + horizontal_diff) as usize;

    return Some(lines[target_y][target_x]);
}

pub fn get_position_of_a(dir: Direction, pivot_x: usize, pivot_y: usize) -> APosition {
    let pivot_x_as_i32: i32 = pivot_x as i32;
    let pivot_y_as_i32: i32 = pivot_y as i32;
    let step = 1;
    let mut vertical_diff = 0;
    let mut horizontal_diff = 0;
    match dir {
        //Direction::North => vertical_diff = -step,
        Direction::NorthEast => {
            vertical_diff = -step;
            horizontal_diff = step;
        },
        //Direction::East => horizontal_diff = step,
        Direction::SouthEast => {
            vertical_diff = step;
            horizontal_diff = step;
        },
        //Direction::South => vertical_diff = step,
        Direction::SouthWest =>  {
            vertical_diff = step;
            horizontal_diff = -step;
        },
        //Direction::West => horizontal_diff = -step,
        Direction::NorthWest =>  {
            vertical_diff = -step;
            horizontal_diff = -step;
        },
    }

    let target_y = (pivot_y_as_i32 + vertical_diff) as usize;
    let target_x = (pivot_x_as_i32 + horizontal_diff) as usize;

    return APosition {x: target_x, y: target_y};
}

struct APosition {
    x: usize,
    y: usize
}

impl PartialEq<APosition> for APosition {
    fn eq(&self, other: &APosition) -> bool {
        return other.x == self.x && other.y == self.y;
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
enum Direction {
    //North,
    NorthEast,
    //East,
    SouthEast,
    //South,
    SouthWest,
    //West,
    NorthWest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_line_reading_line_returns_line() {
        let line = "1234";

        let result = read_line(line);

        assert_eq!(result, ['1', '2', '3', '4']);
    }

    #[test]
    fn given_many_lines_reading_lines_returns_all_lines() {
        let lines = "123
456
789";

        let result = read_all_lines(lines);

        assert_eq!(result, [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]);
    }

    /*#[test]
    fn given_many_lines_when_reading_lines_two_is_north_of_five() {
        let lines_as_vecs = Vec::from([Vec::from(['1', '2', '3']), Vec::from(['4', '5', '6']), Vec::from(['7', '8', '9'])]);
        let pivot_x = 1;
        let pivot_y = 1;

        let result = get_element_in_direction(Direction::North, pivot_x, pivot_y, lines_as_vecs);

        assert_eq!(result, Some('2'));
    }

    #[test]
    fn given_many_lines_when_reading_lines_four_is_west_of_five() {
        let lines_as_vecs = Vec::from([Vec::from(['1', '2', '3']), Vec::from(['4', '5', '6']), Vec::from(['7', '8', '9'])]);
        let pivot_x = 1;
        let pivot_y = 1;

        let result = get_element_in_direction(Direction::West, pivot_x, pivot_y, lines_as_vecs);

        assert_eq!(result, Some('4'));
    }

    #[test]
    fn given_many_lines_when_reading_lines_west_of_four_is_out_of_bounds() {
        let lines_as_vecs = Vec::from([Vec::from(['1', '2', '3']), Vec::from(['4', '5', '6']), Vec::from(['7', '8', '9'])]);
        let pivot_x = 0;
        let pivot_y = 1;

        let result = get_element_in_direction(Direction::West, pivot_x, pivot_y, lines_as_vecs);
    }*/

    #[test]
    fn example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = find_patterns_xmas(input);

        assert_eq!(result, 9);
    }
}
