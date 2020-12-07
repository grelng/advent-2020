use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    q3();
    /*
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
     */

    /*
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} success", display),
    }
    for num in s.split('\n') {
        println!("{}", num);
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed 
    */
}


fn q1() {
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers.push(ip.parse().unwrap());
                // println!("{}", numbers[numbers.len()-1]);
            }
        }
    }
    for first_num in numbers.iter() {
        for second_num in numbers.iter() {
            let sum = first_num + second_num;
            if sum == 2020 {
                println!("{} + {} = 2020, product is {}", first_num, second_num, first_num * second_num);
            }
        }
    }

    for first_num in numbers.iter() {
        for second_num in numbers.iter() {
            for third_num in numbers.iter() {
                let sum = first_num + second_num + third_num;
                if sum == 2020 {
                    println!("{} + {} + {} = 2020, product is {}", first_num, second_num, third_num, first_num * second_num * third_num);
                }
            }
        }
    }
}

fn q2() {
    if let Ok(lines) = read_lines("2.txt") {
        let mut num_valid_a = 0;
        let mut num_valid_b = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut v: Vec<&str> = ip.split(|c| c == ' ' || c == ':' || c == '-').collect();
                v.retain(|&x| x != "");
                println!("{:#?}", v);
                let min: usize = v[0].parse().unwrap();
                let max: usize = v[1].parse().unwrap();
                let mychar = v[2].chars().next().unwrap();
                let pw = v[3].chars().collect::<Vec<char>>();
                let mut count: usize = 0;
                for c in &pw {
                    if *c == mychar {
                        count += 1;
                    }
                }
                if count >= min && count <= max {
                    num_valid_a += 1;
                }
                let mut num_valid = 0;
                if pw[min-1] == mychar {
                    num_valid += 1;
                }
                if pw[max-1] == mychar {
                    num_valid += 1;
                }
                if num_valid == 1 {
                    num_valid_b += 1;
                }
            }
        }
        println!("num valid a {}", num_valid_a);
        println!("num valid b {}", num_valid_b);
    }
}

fn q3() {
    const HEIGHT: usize = 323;
    const WIDTH: usize = 31;
    let mut v = vec![vec![0; WIDTH]; HEIGHT];
    if let Ok(lines) = read_lines("3.txt") {
        let mut line_ctr: usize = 0;
        for line in lines {
            if let Ok(ip) = line {
                let full_line = ip;
                let mut char_ctr: usize = 0;
                for c in full_line.chars() {
                    v[line_ctr][char_ctr] = if c == '.' {
                        0
                    } else {
                        1
                    };
                    char_ctr += 1;
                }
                line_ctr += 1;
            }
        }
    }
   
    let mut tree_total: usize = 1;
    let steps = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    for step in steps {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut num_trees: usize = 0;
        while y < HEIGHT {
                num_trees += v[y][x];
                x = (x + step.0) % 31;
                y += step.1;
        }
        tree_total = tree_total * num_trees;
    }

    println!("tree_total: {}", tree_total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
