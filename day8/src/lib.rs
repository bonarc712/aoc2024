use std::collections::HashMap;
use std::thread::current;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Clone, Debug)]
struct Map {
    coordinates: Vec<Vec<char>>
}

impl Map {
    fn get_width(&self) -> usize {
        self.coordinates[0].len()
    }

    fn get_height(&self) -> usize {
        self.coordinates.len()
    }

    fn is_position_free(&self, position: Position) -> bool {
        return self.coordinates[position.y as usize][position.x as usize] == '.';
    }
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

pub fn read_map(lines: &str) -> Map {
    return Map {coordinates: read_all_lines(lines)}
}

pub fn calculate_antinodes_for(position1: Position, position2: Position, map: &Map) -> Vec<Position> {
    let x_diff = position2.x - position1.x;
    let y_diff = position2.y - position1.y;

    let mut map_edge_reached = false;
    let mut counter = 1;
    let mut antinodes = Vec::new();
    while !map_edge_reached {
        let first_position = Position{x: position2.x + x_diff*counter, y: position2.y + y_diff*counter};
        if is_position_in_map(&first_position, map) {
            antinodes.push(first_position);
            counter += 1;
        }
        else {
            map_edge_reached = true;
        }
    }
    map_edge_reached = false;
    counter = 1;
    while !map_edge_reached {
        let second_position = Position{x: position1.x - x_diff*counter, y: position1.y - y_diff*counter};
        if is_position_in_map(&second_position, map) {
            antinodes.push(second_position);
            counter += 1;
        }
        else {
            map_edge_reached = true;
        }
    }

    antinodes.push(position1);
    antinodes.push(position2);

    println!("Position 1: {:?}", position1);
    println!("Position 2: {:?}", position2);
    println!("Antinodes: {:?}", antinodes);
    return antinodes;
}

pub fn is_position_in_map(position: &Position, map: &Map) -> bool {
    return position.x >= 0
        && position.y >= 0
        && position.x < map.get_width() as i32
        && position.y < map.get_height() as i32;
}

pub fn calculate_antinode_amount(input: &str) -> u32 {
    let map = read_map(input);

    println!("Height: {}", map.get_height());
    println!("Width: {}", map.get_width());

    let mut antenna_hashmap : HashMap<char, Vec<Position>> = HashMap::new();
    let mut known_antenna_types = Vec::new();

    for i in 0..map.coordinates.len() {
        for j in 0..map.coordinates[i].len() {
            let current_element = map.coordinates[j][i];

            if current_element != '.' {
                let mut position_vector = Vec::new();
                if !known_antenna_types.contains(&current_element) {
                    known_antenna_types.push(current_element);
                }
                else {
                    position_vector = antenna_hashmap.get_mut(&current_element).unwrap().to_vec();
                }

                position_vector.push(Position{x: i as i32, y: j as i32});
                antenna_hashmap.insert(current_element, position_vector);
            }
        }
    }

    let mut antinodes = Vec::new();
    for antenna_type in known_antenna_types {
        println!("Testing for {}", antenna_type);
        let positions = antenna_hashmap.get(&antenna_type).unwrap().to_vec();

        let mut antinode_choices = Vec::new();
        for position_i in 0..positions.len() {
            for position_j in position_i+1..positions.len() {
                let current_antinodes = calculate_antinodes_for(positions[position_i], positions[position_j], &map);
                current_antinodes.iter().for_each(|antinode| antinode_choices.push(antinode.clone()));
            }
        }
        for position in antinode_choices {
            println!("Testing position {:?}", position);
            if !antinodes.contains(&position) {
                println!("Accepting position");
                antinodes.push(position);
            }
        }
    }

    return antinodes.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_positions_should_get_two_antinodes() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let map = read_map(input);
        let first_position = Position{ x: 4, y: 3 };
        let second_position = Position { x: 5, y: 5 };

        let result = calculate_antinodes_for(first_position, second_position, &map);

        assert_eq!(result, vec![Position { x: 6, y: 7 }, Position { x: 7, y: 9 }, Position { x: 8, y: 11 }, Position {x: 3, y: 1}, Position{ x: 4, y: 3 }, Position { x: 5, y: 5 }]);
    }

    #[test]
    fn given_two_positions_should_get_two_antinodes_2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let map = read_map(input);
        let first_position = Position{ x: 4, y: 3 };
        let second_position = Position { x: 8, y: 4 };

        let result = calculate_antinodes_for(first_position, second_position, &map);

        assert_eq!(result, vec![Position {x: 0, y: 2}, Position{ x: 4, y: 3 }, Position { x: 8, y: 4 }]);
    }

    #[test]
    fn given_position_not_in_map_then_should_not_be_in_map() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let map = read_map(input);

        let result = is_position_in_map(&Position {x: 13, y: 2}, &map);

        assert_eq!(result, false);
    }

    #[test]
    fn example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let result = calculate_antinode_amount(input);

        assert_eq!(result, 34)
    }

    #[test]
    fn la_patente() {
        let input = ".....wV....q.....................................n
.......w......q.h.....Vn.........................D
............w.S..G.....................DT.........
......S........h......e..T.....y......D...........
......m.......Ae.......T........o.................
....m....S........................................
...m..........................n........8..........
.........2...G......................n.............
..2........V.......h................Q.............
............................o.....................
.Z......I..U....e...u.....G....o..................
...N..G.........................................y.
.....I............q.......h...................s...
......U........qI....o.V..Rz........8........k....
......d.Z.........................R.......8y......
.........e..............T.....l...................
.......2.........................u...R............
.....d.............................Q..............
...................v.....................s.Q....M.
........2..........4.....................8..7.k...
...........x..N..................A..........k.....
...........ZN...........v...............K.........
...d.......N.....................Ky.6.............
...........................l6.....................
....L....g.................4.......k..K.......0...
..............L...........4R................s.....
U......r..............H.4.........................
.......U.............a.......H.............u......
......xY...............l..........................
...................................6..u...........
........Y......L......l............0..............
......9..L...........A.....v..HEa........K........
..................v........6.EX.............z.....
d..Y.............m......A.........................
......................a.i......M...........z......
...................g.......................0......
...............................H.........i........
..........3................W........E...i...0.....
.................t.a....g.................5.......
.r...t...........................7.....5..........
....................................7....5........
....................g.Y...wMz.....................
9..........O....3................W.7..E..XD...1...
t..............3.x.....9..........W.M.............
...9............W.................................
Z.............x................X.i......5.........
...........3.....................................1
...................O.......s....X.................
..............r...................................
..........................O.................1.....";

        let result = calculate_antinode_amount(input);

        assert_eq!(result, 14)

    }
}
