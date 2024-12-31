use std::collections::HashMap;
use std::thread::current;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Clone, Debug)]
struct Filesystem {
    memory_entries: Vec<Option<u64>>,
    group_sizes: HashMap<u64, u32>,
    space_sizes: HashMap<u64, u32>
}

impl Filesystem {
    fn print(&self) -> String {
        let mut string = String::new();
        for i in 0..self.memory_entries.len() {
            match self.memory_entries[i] {
                None => {
                    string.push_str(".");
                }
                Some(index) => {
                    string.push_str(&index.to_string());
                }
            }
        }
        return string;
    }

    fn fragment(&mut self) -> () {
        for i in (0..self.memory_entries.len()).rev() {
            if self.memory_entries[i].is_none() {
                continue;
            }

            let current_index = self.memory_entries[i].unwrap();
            let mut group_size_of_index = *self.group_sizes.get(&current_index).unwrap() as usize;

            let mut next_j_index_to_test = 0;
            for j in 0..i {
                if j < next_j_index_to_test {
                    continue;
                }

                next_j_index_to_test += 1;
                if !self.memory_entries[j].is_none() {
                    continue;
                }

                let current_space_index = j;
                let current_spaces_count = self.get_spaces_count_after_index(j);

                if current_spaces_count >= group_size_of_index {
                    self.swap_elements_at(current_space_index, i - group_size_of_index + 1, group_size_of_index);
                    next_j_index_to_test += group_size_of_index;
                }
            }
        }
    }

    fn swap_elements_at(&mut self, index1: usize, index2: usize, length: usize) {
        for i in 0..length {
            self.memory_entries.swap(index1 as usize + i, index2 as usize + i);
        }
    }

    fn get_spaces_count_after_index(&mut self, index: usize) -> usize {
        let mut count: usize = 0;
        for i in index..self.memory_entries.len() {
            if self.memory_entries[i].is_none() {
                count += 1;
            }
            else {
                break;
            }
        }
        return count;
    }

    /*fn fragment(&mut self) -> () {
        for i in self.memory_entries.len()-1..0 {
            if self.memory_entries[i].is_none() {
                continue;
            }

            let current_index = self.memory_entries[i].unwrap();
            let mut group_size_of_index = *self.group_sizes.get(&current_index).unwrap();

            for index_of_space in self.space_sizes.keys() {
                let length_of_space = *self.space_sizes.get(index_of_space).unwrap();

                if length_of_space >= group_size_of_index {
                    for i in 0..group_size_of_index as usize {
                        self.memory_entries.swap(*index_of_space as usize + i, (current_index - group_size_of_index as u64) as usize + i);
                    }
                }

                self.space_sizes.insert(*index_of_space + group_size_of_index as u64, length_of_space - group_size_of_index);
            }
        }
    }*/

    /*fn fragment(&mut self) -> () { // partie 1
        let mut left_most_empty_space = 0;
        let mut right_most_file_space = self.memory_entries.len() - 1;

        while left_most_empty_space < right_most_file_space {
            if self.memory_entries[left_most_empty_space].is_none()
                && !self.memory_entries[right_most_file_space].is_none() {
                self.memory_entries.swap(left_most_empty_space as usize, right_most_file_space as usize);
            }

            if !self.memory_entries[left_most_empty_space].is_none() {
                left_most_empty_space += 1;
            }

            if self.memory_entries[right_most_file_space].is_none() {
                right_most_file_space -= 1;
            }
        }
    }*/
}

/*pub fn fragment_file_system(mut filesystem: Filesystem) -> Filesystem {

    for i in filesystem.memory_entries.len()-1..0 {
        if filesystem.memory_entries[i].is_none() {
            continue;
        }

        let current_index = filesystem.memory_entries[i].unwrap();
        let mut group_size_of_index = *filesystem.group_sizes.get(&current_index).unwrap();

        for index_of_space in filesystem.space_sizes.keys() {
            let length_of_space = *filesystem.space_sizes.get(index_of_space).unwrap();

            if length_of_space >= group_size_of_index {
                filesystem.swap_elements_at(*index_of_space, current_index - group_size_of_index as u64 + 1, group_size_of_index as usize);
            }
        }
    }

    return filesystem;
}*/

pub fn read_file_system(line: &str) -> Filesystem {
    let mut file_system = Filesystem { memory_entries: Vec::new(), group_sizes: HashMap::new(), space_sizes: HashMap::new() };
    let mut file_mode = true;
    let mut file_index = 0;
    line.chars().for_each(|ch| {
        let current_digit = ch.to_digit(10).unwrap();
        if file_mode {
            for i in 0..current_digit {
                file_system.memory_entries.push(Some(file_index));
            }
            file_system.group_sizes.insert(file_index, current_digit);
            file_index = file_index + 1;
        }
        else {
            for i in 0..current_digit {
                file_system.memory_entries.push(None);
            }
            file_system.space_sizes.insert((file_system.memory_entries.len() as u64) - current_digit as u64, current_digit);
        }
        file_mode = !file_mode;
    });
    return file_system;
}

pub fn calculate_checksum(file_system: &Filesystem) -> u64 {
    let mut checksum : u64 = 0;

    for i in 0..file_system.memory_entries.len() {
        match file_system.memory_entries[i] {
            None => continue,
            Some(index) => checksum += index * (i as u64)
        }
    }

    return checksum;
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
    fn when_printing_filesystem_prints_correctly() {
        let input = "12345";

        let file_system = read_file_system(input);

        let result = file_system.print();

        assert_eq!(result, "0..111....22222");
    }

    #[test]
    fn when_swapping_filesystem_prints_correctly() {
        let input = "12345";

        let mut file_system = read_file_system(input);

        file_system.swap_elements_at(3, 10, 3);

        let result = file_system.print();

        assert_eq!(result, "0..222....11122");
    }

    #[test]
    fn when_counting_spaces_is_ok() {
        let input = "12345";

        let mut file_system = read_file_system(input);

        let result = file_system.get_spaces_count_after_index(6);

        assert_eq!(result, 4);
    }

    #[test]
    fn when_reading_filesystem_space_sizes_are_correct() {
        let input = "12345";

        let file_system = read_file_system(input);

        let result = file_system.space_sizes;

        println!("{:#?}", result);
        assert_eq!(result.get(&1), Some(&2));
        assert_eq!(result.get(&6), Some(&4));
    }

    #[test]
    fn when_reading_filesystem_group_size_is_good() {
        let input = "12345";

        let file_system = read_file_system(input);

        let result = *file_system.group_sizes.get(&2).unwrap();

        assert_eq!(result, 5 as u32);
    }

    //#[test]
    fn when_fragmenting_filesystem_prints_correctly() {
        let input = "12345";

        let mut file_system = read_file_system(input);

        //file_system.fragment();

        let result = file_system.print();

        assert_eq!(result, "022111222......");
    }

    #[test]
    fn when_printing_example_prints_correctly() {
        let input = "2333133121414131402";

        let file_system = read_file_system(input);

        let result = file_system.print();

        assert_eq!(result, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn when_fragmenting_example_prints_correctly() {
        let input = "2333133121414131402";

        let mut file_system = read_file_system(input);

        file_system.fragment();

        let result = file_system.print();

        assert_eq!(result, "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn given_disk_can_calculate_checksum() {
        let input = "2333133121414131402";

        let mut file_system = read_file_system(input);

        file_system.fragment();

        let result = calculate_checksum(&file_system);

        assert_eq!(result, 2858);
    }

    #[test]
    fn la_patente_checksum() {
        let mut input = "";

        let mut file_system = read_file_system(input);

        file_system.fragment();

        let result = calculate_checksum(&file_system);

        assert_eq!(result, 1928);
    }
}
