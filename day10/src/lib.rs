pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct TrailheadScore {
    trailhead: Position,
    score: u32
}

#[derive(Clone, Debug)]
struct Map {
    coordinates: Vec<Vec<u32>>
}

impl Map {
    fn get_width(&self) -> usize {
        self.coordinates[0].len()
    }

    fn get_height(&self) -> usize {
        self.coordinates.len()
    }

    fn get_int_at_position(&self, pos: Position) -> u32 {
        return self.coordinates[pos.y as usize][pos.x as usize];
    }

    fn get_trailheads(&self) -> Vec<Position> {
        let mut trailheads: Vec<Position> = Vec::new();
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if self.coordinates[j][i] == 0 {
                    trailheads.push(Position { x: i as i32, y: j as i32 });
                }
            }
        }
        return trailheads;
    }
}

pub fn read_line(line: &str) -> Vec<u32> {
    return line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
}

pub fn read_all_lines(lines: &str) -> Vec<Vec<u32>> {
    let mut lines_vec = Vec::new();
    for line in lines.lines() {
        lines_vec.push(read_line(line));
    }
    return lines_vec;
}

pub fn read_map(lines: &str) -> Vec<Vec<u32>> {
    return read_all_lines(lines)
}

pub fn sum_trailhead_scores(trailhead_scores: Vec<TrailheadScore>) -> u32 {
    trailhead_scores.iter().map(|trailhead_score| trailhead_score.score).sum()
}

pub fn is_position_in_map(position: &Position, map: &Map) -> bool {
    return position.x >= 0
        && position.y >= 0
        && position.x < map.get_width() as i32
        && position.y < map.get_height() as i32;
}

pub fn get_ascending_coordinates_around(position: Position, map: &Map) -> Vec<Position> {
    let mut ascending_coordinates : Vec<Position> = Vec::new();
    let current_altitude = map.get_int_at_position(position);

    let north_position = Position { x: position.x, y: position.y - 1 };
    if is_position_in_map(&north_position, map) && map.get_int_at_position(north_position) == current_altitude + 1 {
        ascending_coordinates.push(north_position);
    }

    let east_position = Position { x: position.x + 1, y: position.y };
    if is_position_in_map(&east_position, map) && map.get_int_at_position(east_position) == current_altitude + 1 {
        ascending_coordinates.push(east_position);
    }

    let south_position = Position { x: position.x, y: position.y + 1 };
    if is_position_in_map(&south_position, map) && map.get_int_at_position(south_position) == current_altitude + 1 {
        ascending_coordinates.push(south_position);
    }

    let west_position = Position { x: position.x - 1, y: position.y };
    if is_position_in_map(&west_position, map) && map.get_int_at_position(west_position) == current_altitude + 1 {
        ascending_coordinates.push(west_position);
    }
    return ascending_coordinates;
}

#[derive(Clone, Debug)]
struct SummitsAndScore {
    summits: Vec<Position>,
    score: u32
}

pub fn explore_trail(current_trail_pos : Position, map: &Map, mut total: u32, mut found_summits: Vec<Position>) -> SummitsAndScore {
    let positions = get_ascending_coordinates_around(current_trail_pos, map);
    let mut summits_and_score = SummitsAndScore { summits: found_summits, score: total };
    for position in positions {
        if map.get_int_at_position(position) == 9 {
            //if !summits_and_score.summits.contains(&position) { --> uncomment for part 1
                println!("Summit not in summits found {:?}", position);
                summits_and_score.summits.push(position);
                summits_and_score.score += 1;
            //}
            println!("Found summit {:?}", position);
        }
        else {
            summits_and_score = explore_trail(position, map, summits_and_score.score.clone(), summits_and_score.summits.clone());
        }
    }
    return summits_and_score;
}

pub fn find_score_sum_for_little_reindeer(map: Map) -> u32 {
    let mut trailhead_scores = Vec::new();
    let trailheads = map.get_trailheads();

    for trailhead in trailheads {
        println!("Exploring position {:?}", trailhead);
        let summits_and_score = explore_trail(trailhead, &map, 0, Vec::new());
        trailhead_scores.push(TrailheadScore { trailhead, score : summits_and_score.score });
    }
    println!("Trailheads: {:?}", trailhead_scores);

    //return trails;
    return sum_trailhead_scores(trailhead_scores);
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
    fn reading_map_works() {
        let input = "0123
1234
8765
9876";

        let result = Map { coordinates: read_map(input)};

        assert_eq!(result.coordinates[2][2], 6);
    }

    #[test]
    fn getting_trailheads_returns_them() {
        let input = "1023
1234
8765
9876";
        let map = Map { coordinates: read_map(input)};

        let result = map.get_trailheads();

        assert_eq!(result, vec![Position { x: 1, y: 0 }]);
    }

    #[test]
    fn getting_coordinates_around_point_returns_only_one_above() {
        let input = "0123
1234
8765
9876";
        let map = Map { coordinates: read_map(input)};

        let result = get_ascending_coordinates_around(Position { x: 2, y: 0}, &map);

        assert_eq!(result, vec![Position { x: 3, y: 0 }, Position { x: 2, y: 1 }]);
    }

    #[test]
    fn summing_trailheads_scores_works() {
        let trailhead_score_1 = TrailheadScore { trailhead: Position { x: 1, y: 0 }, score: 3 };
        let trailhead_score_2 = TrailheadScore { trailhead: Position { x: 5, y: 0 }, score: 5 };
        let trailhead_scores = vec![trailhead_score_1, trailhead_score_2];

        let result = sum_trailhead_scores(trailhead_scores);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let map = Map { coordinates: read_map(input)};

        let result = find_score_sum_for_little_reindeer(map);

        assert_eq!(result, 81);
    }

    #[test]
    fn test_example_2() {
        let input = "9990999
9991999
9992999
6543456
7000007
8000008
9000009";

        let map = Map { coordinates: read_map(input)};

        let result = find_score_sum_for_little_reindeer(map);

        assert_eq!(result, 2);
    }
}
