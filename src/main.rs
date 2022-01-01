use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines};
use sliding_windows::IterExt;
use sliding_windows::Storage;

fn file_to_lines(file_path: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(file_path)?;
    let b = BufReader::new(f);
    let lines = b.lines();
    Ok(lines)
}

fn lines_to_nums(lines: Lines<BufReader<File>>) -> Vec<u32> {
    lines.map(|num_str| num_str.unwrap().parse().unwrap()).collect()
}

// This helper function lets us calculate the increments step by step by zipping the 
// input numbers with their "tail" and counting them.
fn count_increments(nums: &Vec<u32>) -> u32 {
    itertools::zip(nums, &nums[1..]).fold(0, {
        |acc, (prev,next)|
            if next > prev {
                acc + 1
            } else {
                acc
            }
    })
}

// For part 1 we just call count increments on the input
fn solve1(input_file: &str) -> u32 {
    let lines = file_to_lines(input_file).unwrap(); 
    let nums = lines_to_nums(lines);
    count_increments(&nums)
}

// For part 2 I wanted to try out a crate to handle the sliding window of triples. It 
// seems a bit overkill here, the ergonomics are not great.
fn solve2(input_file: &str) -> u32 {
    let lines = file_to_lines(input_file).unwrap(); 
    let nums = lines_to_nums(lines);
    let elements = vec![0u32,3];
    let mut storage = Storage::from_vec(elements, 3);
    let it = nums.into_iter().sliding_windows(&mut storage); 
    let sums: Vec<u32> = it.map(|window| window.iter().sum()).collect();
    count_increments(&sums)
}

// solve 1 example 7
// solve 1 input 1529
// solve 2 example 5
// solve 2 input 1567

fn main() {
    println!("solve 1 example {:?}", solve1("data/example.txt"));
    println!("solve 1 input {:?}", solve1("data/input.txt"));

    println!("solve 2 example {:?}", solve2("data/example.txt"));
    println!("solve 2 input {:?}", solve2("data/input.txt"));
}
