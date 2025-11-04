#!/bin/bash

# Number of days to generate
DAYS=12

# Template for `src/bin/dayXX.rs`
BIN_TEMPLATE='
use my_aoc2025::days::DAY_MOD::{part1, part2};

fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer {}", part2());
}
'

# Template for `src/days/dayXX.rs`
DAY_TEMPLATE='
#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/inputDAY_NUM.txt");

pub fn part1() -> u32 {
    42
}

pub fn part2() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 0);
    }
    
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
    }
}
'

# Prepare the lib.rs content with the header for the days module
LIB_CONTENT='
pub mod days {
'

# Loop through each day and generate the files
for i in $(seq -w 1 $DAYS); do
    # Format the day number (e.g., 01, 02, etc.)
    DAY_NUM=$i
    DAY_MOD="day$DAY_NUM"

    # Generate `src/bin/dayXX.rs`
    BIN_FILENAME="src/bin/$DAY_MOD.rs"
    BIN_CONTENT="${BIN_TEMPLATE//DAY_MOD/$DAY_MOD}"
    mkdir -p "$(dirname "$BIN_FILENAME")"
    echo "$BIN_CONTENT" > "$BIN_FILENAME"
    echo "Generated $BIN_FILENAME"

    # Generate `src/days/dayXX.rs`
    DAY_FILENAME="src/days/$DAY_MOD.rs"
    DAY_CONTENT="${DAY_TEMPLATE//DAY_NUM/$DAY_NUM}"
    mkdir -p "$(dirname "$DAY_FILENAME")"
    echo "$DAY_CONTENT" > "$DAY_FILENAME"
    echo "Generated $DAY_FILENAME"

    # Generate `inputs/inputXX.txt` with placeholder content
    INPUT_FILENAME="inputs/input$DAY_NUM.txt"
    echo "replace me with input from day$DAY_NUM" > "$INPUT_FILENAME"
    echo "Generated $INPUT_FILENAME"

    # Append this day to the LIB_CONTENT
    LIB_CONTENT+="    pub mod $DAY_MOD;\n"
done

# Finalize and write `src/lib.rs`
LIB_CONTENT+='}'

LIB_FILENAME="src/lib.rs"
echo -e "$LIB_CONTENT" > "$LIB_FILENAME"
echo "Generated $LIB_FILENAME"
