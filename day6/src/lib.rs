#![allow(warnings)]

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Clone, Debug, PartialEq)]
struct Move {
    dir: Direction,
    pos: Position
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

pub fn read_map(lines: &str) -> Vec<Vec<char>> {
    return read_all_lines(lines)
}

pub fn is_clear_way(cell: char) -> bool {
    return cell == '.' || cell == 'X'
}

pub fn is_obstacle(cell: char) -> bool {
    return cell == '#'
}

pub fn is_guard(cell: char) -> bool {
    return cell == '^' || cell == 'v' || cell == '<' || cell == '>'
}

pub fn get_facing_direction_of_guard(cell: char) -> Direction {
    return match cell {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("This is not a guard!")
    };
}

pub fn get_guard_symbol_by_direction(dir: Direction) -> char {
    return match dir {
        Direction::North => '^',
        Direction::East => '>',
        Direction::South => 'v',
        Direction::West => '<'
    }
}

pub fn find_guard(map: Vec<Vec<char>>) -> Position {
    for i in 0..map.len() {
        let current_sub_vec: Vec<char> = map.get(i).unwrap().clone();
        for j in 0..current_sub_vec.len() {
            let current_element = map[j][i];

            if is_guard(current_element) {
                return Position { x: i as i32, y: j as i32 }
            }
        }
    }
    panic!("No guard found");
}

// rotate 90° clockwise
pub fn get_direction_to_rotate_to(source_dir: Direction) -> Direction {
    return match source_dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

// rotate 90° counterclockwise (what do I see on my left)
pub fn get_direction_rotating_from(source_dir: Direction) -> Direction {
    return match source_dir {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

pub fn is_position_inside_of_map(dest_position: Position, map: Vec<Vec<char>>) -> bool {
    return dest_position.x >= 0
        && dest_position.y >= 0
        && dest_position.x < map.get(0).unwrap().len() as i32
        && dest_position.y < map.get(0).unwrap().len() as i32;
}

pub fn get_destination_position(source_position: Position, orientation: Direction) -> Position {
    return match orientation {
        Direction::North => Position { x: source_position.x, y: source_position.y - 1 },
        Direction::East => Position { x: source_position.x + 1, y: source_position.y },
        Direction::South => Position { x: source_position.x, y: source_position.y + 1 },
        Direction::West => Position { x: source_position.x - 1, y: source_position.y }
    }
}

pub fn get_amount_of_x(map: Vec<Vec<char>>) -> u32 {
    let mut positions_visited: u32 = 0;

    for i in 0..map.len() {
        let current_sub_vec: Vec<char> = map.get(i).unwrap().clone();
        for j in 0..current_sub_vec.len() {
            let current_element = map[j][i];

            if current_element == 'X' {
                positions_visited += 1;
            }
        }
    }
    return positions_visited;
}

pub fn analyze_guards_patrol_pattern(lines: &str) -> u32 {
    let mut map = read_map(lines);
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    //println!("Guard pos : {:?}", guard_position);
    //println!("{:?}", guard_orientation);

    loop { // action loop
        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            break;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
            //println!("Advancing");
            //println!("New position: {:?}", source_position);
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            //println!("Rotating");
            //println!("New orientation: {:?}", guard_orientation);
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };

    return get_amount_of_x(map);
}

pub fn find_all_possible_obstructions_old(lines: &str) -> u32 {
    let mut valid_obstructions: Vec<Position> = Vec::new();
    let mut visited_obstacles: Vec<Position> = Vec::new();
    let mut map = read_map(lines);
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    /*println!("Blablablublu");
    println!("Guard pos : {:?}", guard_position);
    println!("{:?}", guard_orientation);*/

    loop { // action loop

        let position_on_the_left = get_destination_position(source_position.clone(),
                                                            get_direction_rotating_from(guard_orientation.clone()));

        println!("New pos: {:?}", position_on_the_left);


        if is_position_inside_of_map(position_on_the_left.clone(), map.clone()) &&
            is_obstacle(map[position_on_the_left.y as usize][position_on_the_left.x as usize]) //&& 
        //visited_obstacles.get(visited_obstacles.len() - 1).unwrap() != &position_on_the_left
        {
            visited_obstacles.push(position_on_the_left);
        }

        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            break;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            if visited_obstacles.len() >= 3 {
                // if I have an obstacle on my left, it's as though I would have visited this obstacle.
                if test_possible_obstruction(map.clone(), visited_obstacles.clone(), dest_position.clone()) {
                    valid_obstructions.push(dest_position.clone());
                }
            }
            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
            //println!("Advancing");
            //println!("New position: {:?}", source_position);
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            visited_obstacles.push(dest_position);
            //println!("Rotating");
            //println!("New orientation: {:?}", guard_orientation);
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };
    //print_map(&map);
    print_map_with_valid_obstructions(&map, &valid_obstructions);

    return valid_obstructions.len() as u32;
}

pub fn find_all_possible_obstructions_old_2(lines: &str) -> u32 {
    let mut valid_obstructions: Vec<Position> = Vec::new();
    let mut visited_obstacles: Vec<Position> = Vec::new();
    let mut map = read_map(lines);
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    /*println!("Blablablublu");
    println!("Guard pos : {:?}", guard_position);
    println!("{:?}", guard_orientation);*/

    loop { // action loop

        //let position_on_the_left = get_destination_position(source_position.clone(), 
        //    get_direction_rotating_from(guard_orientation.clone()));

        //println!("New pos: {:?}", position_on_the_left);


        //if is_position_inside_of_map(position_on_the_left.clone(), map.clone()) &&
        //is_obstacle(map[position_on_the_left.y as usize][position_on_the_left.x as usize]) //&&
        //visited_obstacles.get(visited_obstacles.len() - 1).unwrap() != &position_on_the_left
        //{
        //visited_obstacles.push(position_on_the_left);
        //}

        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            break;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            if test_if_goes_forever(map.clone(), dest_position.clone()) {
                valid_obstructions.push(dest_position.clone());
            }

            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
            //println!("Advancing");
            //println!("New position: {:?}", source_position);
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            //visited_obstacles.push(dest_position);
            //println!("Rotating");
            //println!("New orientation: {:?}", guard_orientation);
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };
    //print_map(&map);
    print_map_with_valid_obstructions(&map, &valid_obstructions);

    return valid_obstructions.len() as u32;
}

pub fn find_all_possible_obstructions(lines: &str) -> u32 {
    let mut valid_obstructions: Vec<Position> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut map = read_map(lines);
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    println!("First position: {:?}", guard_position.clone());

    loop { // action loop
        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            break;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {

            if blocking_dest_redoes_a_move(map.clone(), moves.clone(), dest_position.clone()) {
                if !valid_obstructions.contains(&dest_position) {
                    valid_obstructions.push(dest_position.clone());
                }
            }
            moves.push(Move { dir: guard_orientation, pos: dest_position.clone()});
            /*if test_if_goes_forever(map.clone(), dest_position.clone()) {
                valid_obstructions.push(dest_position.clone());
            }*/

            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };
    print_map_with_valid_obstructions(&map, &valid_obstructions);

    return valid_obstructions.len() as u32;
}

pub fn blocking_dest_redoes_a_move(source_map: Vec<Vec<char>>, moves: Vec<Move>, obstruction_to_test: Position) -> bool {
    let mut steps = 30000;
    let mut moves_for_this = moves.clone();
    let mut map = source_map.clone();
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);
    let original_guard_orientation = guard_orientation.clone();

    loop { // action loop
        steps -= 1;
        if steps == 0 {
            println!("Went to end of steps");
            println!("Position to try: {:?}", obstruction_to_test);
            return true;
        }

        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if dest_position == obstruction_to_test {
            map[dest_position.y as usize][dest_position.x as usize] = '#';
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            continue;
        }

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            //println!("Came out of map");
            return false;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            let a_move = Move { dir: guard_orientation, pos: dest_position.clone() };
            if moves_for_this.contains(&a_move) {
                //println!("Went to end of moves");
                println!("Obstructing: {:?} for position {:?}", original_guard_orientation, obstruction_to_test);
                return true;
            }
            moves_for_this.push(a_move);
            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };
}

pub fn test_if_goes_forever(source_map: Vec<Vec<char>>, obstruction_to_test: Position) -> bool {
    let mut steps = 20000;

    let mut map = source_map.clone();
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    /*println!("Blablablublu");
    println!("Guard pos : {:?}", guard_position);
    println!("{:?}", guard_orientation);*/

    loop { // action loop
        steps -= 1;
        if steps == 0 {
            return true;
        }

        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if dest_position == obstruction_to_test {
            map[dest_position.y as usize][dest_position.x as usize] = '#';
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            //println!("Rotating");
            //println!("New orientation: {:?}", guard_orientation);
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            continue;
        }

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            return false;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
        }
        else if is_obstacle(destination_cell) {
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
        }
    };

}

pub fn get_all_obstacles(source_map: &Vec<Vec<char>>) -> Vec<Position> {
    let mut obstacles: Vec<Position> = Vec::new();

    for i in 0..source_map.len() {
        let mut map_line = String::from("");
        let current_sub_vec: Vec<char> = source_map.get(i).unwrap().clone();
        for j in 0..current_sub_vec.len() {
            let current_element = source_map[i][j];
            if (current_element == '#')
            {
                obstacles.push(Position {x: j as i32, y: i as i32})
            }
        }
    }
    return obstacles;
}

pub fn print_map(source_map: &Vec<Vec<char>>) -> () {
    for i in 0..source_map.len() {
        let mut map_line = String::from("");
        let current_sub_vec: Vec<char> = source_map.get(i).unwrap().clone();
        for j in 0..current_sub_vec.len() {
            let current_element = source_map[i][j];
            map_line.push(current_element);
        }
        println!("{}", map_line);
    }
}

pub fn print_map_with_valid_obstructions(source_map: &Vec<Vec<char>>, obstruction_list: &Vec<Position>) -> () {
    for i in 0..source_map.len() {
        let mut map_line = String::from("");
        let current_sub_vec: Vec<char> = source_map.get(i).unwrap().clone();
        for j in 0..current_sub_vec.len() {
            let mut current_element = source_map[i][j];
            for position in 0..obstruction_list.len() {
                if i == obstruction_list[position].y as usize && j == obstruction_list[position].x as usize {
                    current_element = 'O';
                }
            }
            map_line.push(current_element);
        }
        println!("{}", map_line);
    }
}

pub fn test_possible_obstruction(source_map: Vec<Vec<char>>, known_obstacles: Vec<Position>, obstruction_to_test: Position) -> bool {
    let mut map = source_map.clone();
    let guard_position = find_guard(map.clone());
    let mut source_position = guard_position.clone();
    let mut guard_orientation = get_facing_direction_of_guard(map[guard_position.y as usize][guard_position.x as usize]);

    /*println!("Blablablublu");
    println!("Guard pos : {:?}", guard_position);
    println!("{:?}", guard_orientation);*/

    loop { // action loop

        let dest_position = get_destination_position(source_position.clone(), guard_orientation.clone());

        if dest_position == obstruction_to_test {
            map[dest_position.y as usize][dest_position.x as usize] = '#';
            guard_orientation = get_direction_to_rotate_to(guard_orientation.clone());
            //println!("Rotating");
            //println!("New orientation: {:?}", guard_orientation);
            map[source_position.y as usize][source_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            continue;
        }

        if !is_position_inside_of_map(dest_position.clone(), map.clone()) {
            return false;
        }

        let destination_cell = map[dest_position.y as usize][dest_position.x as usize];
        if is_clear_way(destination_cell) {
            map[source_position.y as usize][source_position.x as usize] = 'X';
            map[dest_position.y as usize][dest_position.x as usize] = get_guard_symbol_by_direction(guard_orientation);
            source_position = dest_position;
            //println!("Advancing");
            //println!("New position: {:?}", source_position);
        }
        else if is_obstacle(destination_cell) {
            for i in 0..known_obstacles.len() {
                if *known_obstacles.get(i).unwrap() == dest_position {
                    return true;
                }
            }
            return false;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_possible_obstructions_does_find_them_all() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = find_all_possible_obstructions(input);

        assert_eq!(6, result);
    }

    #[test]
    fn given_a_map_find_all_obstacles() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let map = read_map(input);
        let mut obstacles = Vec::new();
        obstacles.push(Position { x: 4, y: 0});
        obstacles.push(Position { x: 9, y: 1});
        obstacles.push(Position { x: 2, y: 3});
        obstacles.push(Position { x: 7, y: 4});
        obstacles.push(Position { x: 1, y: 6});
        obstacles.push(Position { x: 8, y: 7});
        obstacles.push(Position { x: 0, y: 8});
        obstacles.push(Position { x: 6, y: 9});

        let result = get_all_obstacles(&map);

        assert_eq!(obstacles, result);
    }

    #[test]
    fn given_a_map_find_all_working_obstacles() {

    }

    #[test]
    fn given_known_obstruction_possibility_it_is_recognized() {
        let input = "....#.....
....XXXXX#
....X...X.
..#.X...X.
....X..#X.
....X...X.
.#..<XXXX.
........#.
#.........
......#...";
        let map = read_map(input);
        let mut known_obstacles = Vec::new();
        known_obstacles.push(Position { x: 4, y: 0});
        known_obstacles.push(Position { x: 9, y: 1});
        known_obstacles.push(Position { x: 8, y: 7});
        let potential_obstruction = Position { x: 3, y: 6};

        let result = test_possible_obstruction(map, known_obstacles, potential_obstruction);

        assert_eq!(true, result);
    }

    #[test]
    fn given_obstruction_in_a_square_it_is_recognized() {
        let input = ".#.
.v#
.#.";
        let map = read_map(input);
        let mut known_obstacles = Vec::new();
        known_obstacles.push(Position { x: 1, y: 0});
        known_obstacles.push(Position { x: 2, y: 1});
        known_obstacles.push(Position { x: 1, y: 2});
        let potential_obstruction = Position { x: 0, y: 1};

        let result = test_possible_obstruction(map, known_obstacles, potential_obstruction);

        assert_eq!(true, result);
    }

    #[test]
    fn given_obstruction_that_might_fail_should_be_recognized() {
        let input = "..#..
..XX#
#^XX#
.#.#.
.....";
        let map = read_map(input);
        let mut known_obstacles = Vec::new();
        known_obstacles.push(Position { x: 2, y: 0});
        known_obstacles.push(Position { x: 4, y: 1});
        known_obstacles.push(Position { x: 4, y: 2});
        known_obstacles.push(Position { x: 3, y: 3});
        known_obstacles.push(Position { x: 0, y: 2});
        let potential_obstruction = Position { x: 1, y: 1};

        let result = test_possible_obstruction(map, known_obstacles, potential_obstruction);

        assert_eq!(true, result);
    }

    #[test]
    fn given_obstruction_with_longer_known_list_return_true() {
        let input = "....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXvXX.
........#.
#.........
......#...";
        let map = read_map(input);
        let mut known_obstacles = Vec::new();
        known_obstacles.push(Position { x: 4, y: 0});
        known_obstacles.push(Position { x: 9, y: 1});
        known_obstacles.push(Position { x: 8, y: 7});
        known_obstacles.push(Position { x: 1, y: 6});
        known_obstacles.push(Position { x: 2, y: 3});
        known_obstacles.push(Position { x: 7, y: 4});
        let potential_obstruction = Position { x: 6, y: 7};

        let result = test_possible_obstruction(map, known_obstacles, potential_obstruction);

        assert_eq!(true, result);
    }

    #[test]
    fn given_obstruction_that_pushes_outside_boundary_return_false() {
        let input = "....#.....
....XXXXX#
....X...X.
..#.X...X.
....X..#X.
..^.X...X.
.#XXXXXXX.
........#.
#.........
......#...";
        let map = read_map(input);
        let mut known_obstacles = Vec::new();
        known_obstacles.push(Position { x: 4, y: 0});
        known_obstacles.push(Position { x: 9, y: 1});
        known_obstacles.push(Position { x: 8, y: 7});
        known_obstacles.push(Position { x: 1, y: 6});
        let potential_obstruction = Position { x: 2, y: 4};

        let result = test_possible_obstruction(map, known_obstacles, potential_obstruction);

        assert_eq!(false, result);
    }

    #[test]
    fn given_a_src_pos_going_north_is_one_less_y() {
        let src_pos = Position { x: 4, y: 15};
        let direction = Direction::North;

        let result = get_destination_position(src_pos, direction);

        assert_eq!(Position { x: 4, y: 14}, result);
    }

    #[test]
    fn given_map_with_3_pos_visited_then_should_return_3() {
        let input = ".XX
..X
..#";

        let result = get_amount_of_x(read_map(input));

        assert_eq!(3, result);
    }

    #[test]
    fn given_map_a_position_out_of_it_is_invalid() {
        let input = ".^
..";
        let position = Position { x: -1, y: 0 };

        let map = read_map(input);
        let result = is_position_inside_of_map(position, map);

        assert_eq!(false, result);
    }

    #[test]
    fn given_map_a_position_out_of_it_is_invalid_2() {
        let input = ".^
..";
        let position = Position { x: 2, y: 0 };

        let map = read_map(input);
        let result = is_position_inside_of_map(position, map);

        assert_eq!(false, result);
    }

    #[test]
    fn given_north_should_rotate_to_east() {
        let input = Direction::North;

        let result = get_direction_to_rotate_to(input);

        assert_eq!(Direction::East, result);
    }

    #[test]
    fn given_a_map_it_can_be_read_with_obstacles_and_clear_ways() {
        let input = ".#
..";

        let result = read_map(input);

        assert_eq!(true, is_clear_way(result[1][0]));
        assert_eq!(true, is_obstacle(result[0][1]));
    }

    #[test]
    fn given_a_map_with_a_guard_then_guard_found() {
        let input = ".^
..";

        let result = read_map(input);

        assert_eq!(true, is_guard(result[0][1]));
    }

    #[test]
    fn given_a_map_with_a_guard_then_guard_position_found() {
        let input = ".^
..";

        let map = read_map(input);
        let guard_position = find_guard(map);

        assert_eq!(Position {x: 1, y: 0}, guard_position);
    }

    #[test]
    fn given_a_map_with_a_guard_then_guard_found_2() {
        let input = ".v
..";

        let result = read_map(input);

        assert_eq!(true, is_guard(result[0][1]));
    }

    macro_rules! guard_facing_tests {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
        
                let result = get_facing_direction_of_guard(input);
                
                assert_eq!(expected, result);
            }
        )*
        }
    }

    guard_facing_tests! {
        given_a_guard_get_direction_guard_is_facing_north: ('^', Direction::North)
        given_a_guard_get_direction_guard_is_facing_east: ('>', Direction::East)
        given_a_guard_get_direction_guard_is_facing_south: ('v', Direction::South)
        given_a_guard_get_direction_guard_is_facing_west: ('<', Direction::West)
    }

    #[test]
    #[should_panic]
    fn given_no_guard_direction_should_panic() {
        let input = '.';

        let result = get_facing_direction_of_guard(input);
    }

    #[test]
    fn expecting_la_patente_to_fail() {
        let input = "[input]";
        // real result is 1933, thanks to https://github.com/nick42d/aoc-2024 for the solution shared on Reddit. I wonder what the flaw in my algo is but it worked thanks to him.

        let result = find_all_possible_obstructions(input);

        assert_eq!(6, result);
    }
}
