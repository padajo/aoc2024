use aoc2024::day02::{part1, part2};

pub mod aoc2024 {

    use std::fs::File;
    use std::io::{self, BufRead, Error};
    // returns the path based on the test flag
    pub fn get_data_path(test: bool) -> String {
        if test {
            return "./data/test-data".to_string();
        } else {
            return "./data/real-data".to_string();
        }
    }

    // returns a vec of lines as Strings
    pub fn get_data(test: bool) -> Result<Vec<String>, Error> {
        let path = get_data_path(test);
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);

        let mut data = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            data.push(line);
        }

        Ok(data)
    }

    // mod for day01
    pub mod day01 {

        use std::collections::HashMap;
        use std::io::{self};

        pub fn get_and_transpose_data(test:bool) -> Vec<Vec<i32>> {
            // let path = aoc2024::get_data_path(test);
            // let file = File::open(path).unwrap();
            // let reader = io::BufReader::new(file);
            // let lines = get_data(test);
            let lines = super::get_data(test);
        
            if lines.is_err() {
                println!("Error: {}", lines.err().unwrap());
                return vec![];
            }
        
            let data_lines = lines.unwrap();
        
            let mut data = Vec::new();
            // for line in reader.lines() {
            for line in data_lines {
                // let line = line.unwrap();
                let parts: Vec<&str> = line.split_whitespace().collect();
                // change each vec item to i32
                let nums = parts.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                
                // let num = line.parse::<i32>().unwrap();
                data.push(nums);
            }
        
            let transposed = transpose(data);
        
            transposed
        
        }
        
        pub fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
        where
            T: Clone,
        {
            if matrix.is_empty() {
                return vec![];
            }
        
            let num_cols = matrix[0].len();
            let num_rows = matrix.len();
        
            // Create a new matrix with transposed dimensions
            let mut transposed = vec![vec![]; num_cols];
        
            for col in 0..num_cols {
                for row in 0..num_rows {
                    transposed[col].push(matrix[row][col].clone());
                }
            }
        
            transposed
        }
        
        pub fn get_diffs(data: Vec<Vec<i32>>) -> Vec<u32> {
            // sort each vec by size
        
            let mut sorted = vec![];
            for mut row in data {
                row.sort();
                sorted.push(row);
            }
        
            // transpose (rows to columns)
            let transposed = transpose(sorted);
        
            let diffs = transposed.iter().map(|x| i32::abs_diff(x[0], x[1]) ).collect::<Vec<u32>>();
            
            diffs
        }
        
        pub fn get_occurrence_count(data: Vec<i32>) -> Result<HashMap<i32,u32>, String> {
            let mut counts: HashMap<i32,u32> = HashMap::new();
        
            for num in data {
                let count = counts.entry(num).or_insert(0);
                *count += 1;
            }
        
            Ok(counts)
        }

        pub fn part1(test: bool) -> Result<u32, io::Error> {
            let data = get_and_transpose_data(test);
            let diffs = get_diffs(data);
        
            let sum: u32 = diffs.iter().sum();
        
            Ok(sum)
        }
        
        pub fn part2(test: bool) -> Result<u32, io::Error> {
        
            let data = get_and_transpose_data(test);
        
            // for each vec in data, get the occurrence count
            let mut counts = Vec::new();
            let col1 = data[0].clone();
            for row in data {
                let count = get_occurrence_count(row);
                match count {
                    Ok(c) => counts.push(c),
                    Err(e) => println!("Error: {}", e)
                }
            }
        
            let mut similarities: Vec<u32> = vec![];
        
            let mut similarity = 0;
            for num in col1 {
                if counts[1].contains_key(&num) {
                    let n = counts[1].get(&num).unwrap();
                    let product = num as u32 * n;
                    similarity += product.clone();
                    similarities.push(product);
                }
            }
        
            Ok(similarity)
        }
    }
    
    pub mod day02 {
        
        use std::io::{self};

        pub fn part1(test: bool) -> Result<u32, io::Error> {
            Ok(0)
        }
    
        pub fn part2(test: bool) -> Result<u32, io::Error> {
            Ok(0)
        }
    }
}




fn main() {


    println!("Part 1");
    // let num1 = aoc2024::day01::part1(false).unwrap();
    let num1 = aoc2024::day02::part1(false).unwrap();
    println!("Answer: {}", num1);

    println!("Part 2");
    // let num2 = aoc2024::day01::part2(false);
    let num2 = aoc2024::day02::part2(false);
    match num2 {
        Ok(n) => println!("Answer: {}", n),
        Err(e) => println!("Error: {}", e)
    }



}

// add part1 and part2 tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // check if part1 returns 11
        let num = aoc2024::day02::part1(true).unwrap();
        assert_eq!(num, 0);
    }

    #[test]
    fn test_part2() {
        let num = aoc2024::day02::part2(true).unwrap();
        assert_eq!(num, 0);
    }
}