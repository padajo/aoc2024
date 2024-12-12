use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Error};

fn get_data_path(test: bool) -> String {
    if test {
        return "./data/test-data".to_string();
    } else {
        return "./data/real-data".to_string();
    }
}

fn get_and_transpose_data(test:bool) -> Vec<Vec<i32>> {
    let path = get_data_path(test);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        // change each vec item to i32
        let nums = parts.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        // let num = line.parse::<i32>().unwrap();
        data.push(nums);
    }

    let transposed = transpose(data);

    transposed

}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
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

fn get_diffs(data: Vec<Vec<i32>>) -> Vec<u32> {
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

fn get_occurrence_count(data: Vec<i32>) -> Result<HashMap<i32,u32>, String> {
    let mut counts: HashMap<i32,u32> = HashMap::new();

    for num in data {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }

    Ok(counts)
}

fn part1(test: bool, expected_answer: u32) -> Result<(), io::Error> {
    let data = get_and_transpose_data(test);
    let diffs = get_diffs(data);

    let sum: u32 = diffs.iter().sum();

    println!("Sum of diffs: {}", sum);

    if(test) {
        if(sum == expected_answer) {
            println!("Test Passed!");
        } else {
            println!("Test Failed!");
        }
    }

    Ok(())
}

fn part2(test: bool, expected_answer: u32) -> Result<(), io::Error> {

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

    // println!("{:?}", counts);

    // let keys1: HashSet<i32> = counts[0].keys().copied().collect();
    // let keys2: HashSet<i32> = counts[1].keys().copied().collect();

    // println!("{:?}", counts[0].keys());
    // println!("{:?}", counts[1].keys());

    let mut similarities: Vec<u32> = vec![];

    let mut similarity = 0;
    for num in col1 {
        if counts[1].contains_key(&num) {
            let n = counts[1].get(&num).unwrap();
            let product = num as u32 * n;
            similarity += product.clone();
            // println!("N: {}\t{}\t{}\t={}", num, n, product, similarity);
            similarities.push(product);
            // println!("Found: {}", num);
            // println!("Count: {}", counts[1].get(&num).unwrap());
            //similarities.push(num * counts[1].get(num as u32).unwrap());
        }
    }



    println!("Similarity Score: {}", similarity);

    if(test == true) {
        if(similarity == expected_answer) {
            println!("Test Passed!");
        } else {
            println!("Test Failed!");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {


    println!("Part 1");
    part1(true, 11);
    part1(false, 0);

    println!("Part 2");
    part2(true, 31);
    part2(false, 0);



    // let data = get_data("./data/test-data".to_string());
    // let data = get_data("./data/real-data".to_string());

    // println!("{:?}", data);

    // let diffs = get_diffs(data);

    // println!("{:?}", diffs);

    // // sum the diffs
    // let sum: u32 = diffs.iter().sum();

    // println!("Sum of diffs: {}", sum);

    // // Open the file
    // let file = File::open("./data/test-data")?;
    // let reader = io::BufReader::new(file);

    // // Read lines into a Vec<String>
    // let lines: Vec<String> = reader
    //     .lines()
    //     .map(|line| line.expect("Failed to read line")) // Handle any potential errors
    //     .collect();

    // // Print the lines
    // for (index, line) in lines.iter().enumerate() {
    //     // Split on whitespace (automatically handles multiple spaces)
    //     let parts: Vec<&str> = line.split_whitespace().collect();

    //     // Print the result
    //     println!("{:?}", parts);

    //     let a = parts[0].parse::<i32>().unwrap();
    //     let b = parts[1].parse::<i32>().unwrap();

    //     println!("Line {}: {} {}", index, a, b);
    // }

    Ok(())
}
