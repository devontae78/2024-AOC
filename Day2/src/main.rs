use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
};

fn main() {
    load_from_file("./src/PuzzleInput.txt").expect("woops")
}

fn load_from_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path).expect("It should be here");
    let mut temps = 0;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if ascension(&numbers) {
            println!("{:?} is ascending.", numbers);
        } else if descention(&numbers) {
            println!("{:?} is desending", numbers);
        } else {
            println!("Not either {:?}", numbers);
        }

        match correct_sequence(&numbers) {
            Some(corrected) => temps += 1,
            None => println!("{:?} cannot be made valid by removing one number.", numbers),
        }
    }

    println!("{:?}", temps);

    Ok(())
}

fn correct_sequence(numbers: &[i32]) -> Option<Vec<i32>> {
    if is_valid(numbers) {
        return Some(numbers.to_vec());
    }

    for i in 0..numbers.len() {
        let mut modii = numbers.to_vec();
        modii.remove(i);
        if is_valid(&modii) {
            return Some(modii);
        }
    }

    None
}

fn is_valid(numbers: &[i32]) -> bool {
    ascension(numbers) || descention(numbers)
}

fn descention(lines: &[i32]) -> bool {
    lines.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        diff > 0 && diff <= 3
    })
}

fn ascension(lines: &[i32]) -> bool {
    lines.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        diff > 0 && diff <= 3
    })
}
