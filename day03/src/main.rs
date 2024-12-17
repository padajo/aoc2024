pub mod aoc2024 {

    use std::fs::File;
    use std::io::{self, BufRead, Error};
    // returns the path based on the test flag
    pub fn get_data_path(test: bool, part: u32) -> String {
        if test {
            if part == 1 {
                return "./data/test-data".to_string();
            } else {
                return "./data/test-data2".to_string();
            }
        } else {
            return "./data/real-data".to_string();
        }
    }

    // returns a vec of lines as Strings
    pub fn get_data(test: bool, part: u32) -> Result<Vec<String>, Error> {
        let path = get_data_path(test, part);
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);

        let mut data = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            data.push(line);
        }

        Ok(data)
    }

    pub fn line_to_number_vec(line: &str) -> Vec<i32> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let nums = parts.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        nums
    }

    pub mod day03 {
        
        use std::io::{self};

        pub fn part1(test: bool) -> Result<i32, io::Error> {

            let lines = super::get_data(test, 1);

            let mut total = 0;
            
            for line in lines.unwrap() {
                // println!("{:?}", line);
                let multiplications = get_multiplications(&line);

                let muls = multiplications;

                // get the total of all multiplications
                if muls.is_ok() {
                    let muls = muls.unwrap();
                    // println!("{:?}", muls);
                    for m in muls {
                        total += m[0] * m[1];
                    }
                }

            }

            Ok(total)
        }
    
        pub fn part2(test: bool) -> Result<i32, io::Error> {

            let data = super::get_data(test, 2);

            if data.is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, "Error getting data"));
            }

            let lines = data.unwrap();

            let mut total = 0;

            // put all lines into one line?
            let line = lines.join("");

            // check for do() and don't() instructions to know which mul() to use
            // for line in lines {
                // println!("{}", line);
                let do_parts = get_do_parts(&line);
                if do_parts.is_err() {
                    return Err(io::Error::new(io::ErrorKind::Other, "Error getting do_parts"));
                }
                // now for each do_part, get the multiplications
                for do_part in do_parts.unwrap() {
                    // println!("{}", do_part);
                    let multiplications = get_multiplications(&do_part);

                    let muls = multiplications;

                    // get the total of all multiplications
                    if muls.is_ok() {
                        let muls = muls.unwrap();
                        // println!("{:?}", muls);
                        for m in muls {
                            total += m[0] * m[1];
                        }
                    }
                }
            // }

            Ok(total)
        }

        pub fn get_do_parts(memory: &String) -> Result<Vec<String>, String> {
            // println!("{:?}", memory);
            let mut do_parts = Vec::new();

            let mut do_enabled = true;
            let mut instruction_data: String = "".to_string();
            
            let find_do = "do()";
            let find_dont = "don't()";

            // add characters of don't() length to the instruction data
            for i in 0..find_dont.len() {
                instruction_data.push(memory.chars().nth(i).unwrap());
            }

            // looking for do and don't
            for i in find_dont.len()..memory.len() {
                if do_enabled {
                    // println!("{:?}", instruction_data);
                }

                if do_enabled {
                    // check for previous chars to see if they are "don't()"
                    let prev = &memory[i - find_dont.len()..i];
                    if prev == find_dont {
                        // println!("DON'T: {:?}", prev);
                        do_enabled = false;
                        // add the instruction data to the do_parts
                        // remove the last 6 chars
                        instruction_data = instruction_data[..instruction_data.len() - find_dont.len()].to_string();
                        // println!("SAVING: {:?}", instruction_data);
                        do_parts.push(instruction_data.clone());
                        instruction_data = "".to_string();
                    }
                } else {
                    // check for previous chars to see if they are "do()"
                    let prev = &memory[i - find_do.len()..i];
                    if prev == find_do {
                        // println!("DO: {:?}", prev);
                        do_enabled = true;
                        instruction_data = "".to_string();
                    }
                }

                if do_enabled {
                    instruction_data.push(memory.chars().nth(i).unwrap());
                }

            }

            // add the last instruction data
            if do_enabled {
                do_parts.push(instruction_data.clone());
            }

            // println!("{:?}", do_parts);
            
            Ok(do_parts)
            
        }
        
        fn get_mul(mem_part: &String) -> Result<Vec<i32>, String> {

            // println!("{:?}", mem_part);

            // find the first occurrence of ")"
            let end_response = mem_part.find(")");
            if end_response.is_none() {
                return Ok(vec![]);
            } 

            let end = end_response.unwrap();

            // get the substring
            let sub = &mem_part[..end];
            // now it should be "a,b"
            
            // now split on ","
            let parts: Vec<&str> = sub.split(",").collect();

            // there should be 2 parts
            if parts.len() != 2 {
                return Ok(vec![]);
            }

            // length of each should be 1 2 or 3 chars
            let alen = parts[0].len();
            let blen = parts[1].len();
            
            if alen < 1 || alen > 3 || blen < 1 || blen > 3 {
                return Ok(vec![]);
            }
            
            // convert to i32
            let a = parts[0].parse::<i32>().unwrap();
            let b = parts[1].parse::<i32>().unwrap();
            

            let nums = vec![a, b];
                
            Ok(nums)
        }

        fn get_multiplications(memory: &String) -> Result<Vec<Vec<i32>>, String> {
            let mut multiplications = Vec::new();

            // split memory on "mul("
            let parts: Vec<&str> = memory.split("mul(").collect();
            // println!("{:?}", parts);
            
            for part in parts {
                
                // get multiplications
                let mul = get_mul(&(part.to_string()));
                if mul.is_ok() {
                    
                    let m = mul.unwrap();
                    if m.len() == 2 {
                        // println!("M: {:?}", m);
                        multiplications.push(m);
                    }
                }

            }
            
            Ok(multiplications)
            
        }
    }
}


fn main() {

    // https://adventofcode.com/2024/day/2

    println!("Part 1");
    // let num1 = aoc2024::day03::part1(false).unwrap();
    let num1 = aoc2024::day03::part1(false).unwrap();
    println!("Answer: {}", num1);

    println!("Part 2");
    let num2 = aoc2024::day03::part2(false);
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
        let num = aoc2024::day03::part1(true).unwrap();
        assert_eq!(num, 161);
    }
    
    #[test]
    fn test_get_do_parts() {
        let line = "adon't()bcdo(dont()edo()fdont't()as";
        let do_parts = aoc2024::day03::get_do_parts(&line.to_string()).unwrap();
        assert_eq!(do_parts, ["a","fdont't()as"]);
    }

    #[test]
    fn test_part2() {
        let num = aoc2024::day03::part2(true).unwrap();
        assert_eq!(num, 48);
    }
}