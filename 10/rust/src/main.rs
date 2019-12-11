use std::str::EncodeUtf16;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::cmp::{max, Ordering};
use num::integer::gcd;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn print_space(inp: &Vec<Vec<i32>>) {
    let mut meteors = 0;
    let mut max = 0;
    print!("    ");
    for j in 0..inp[0].len() {
        print!("{:3} ", j);
    }
    print!("\n");
    for i in 0..inp.len() {
        print!("{:3} ", i);
        for j in 0..inp[0].len() {
            print!("{:3} ", inp[j][i]);
            if inp[j][i] >= 0 {
                meteors += 1;
                if inp[j][i] > max {
                    max = inp[j][i];
                }
            }
        }
        print!("\n");
    }
    println!("{} {} {} {}", inp.len(), inp[0].len(), meteors, max);
}

fn main() {
    // let mut a = [1, 2, 5, 3, 0];
    // a.sort_by(|x, y| { x.cmp(y) });
    // println!("{:?}", a);

    main2();
//    let mut map = HashMap::<(usize, usize), i32>::new();
//    map.insert((1, 1), 3);
//    let a = map.get(&(1 as usize, 1 as usize));
//    println!("{:?}", a);
}

fn main2() {
     let filename = "input.txt";
     let contents = fs::read_to_string(filename)
         .expect("Something went wrong reading the file");

    let mut inp: Vec<Vec<i32>> = vec![];
    for line in contents.split_whitespace() {
        let mut linp: Vec<i32> = vec![];
        for c in line.chars() {
            if c == '#' {
                linp.push(0);
            } else {
                linp.push(-1);
            }
        }
        inp.push(linp);
    }

    // print_space(&mut inp);

    let mut dirs: Vec<(i32, i32)> = Vec::new();
    dirs.push((1, 0));
    dirs.push((0, 1));

    for x in 1..inp.len() {
        for y in 1..inp[0].len() {
            let g = gcd(x, y);
            if g == 1 {
                dirs.push((x as i32, y as i32));
                dirs.push((x as i32, -(y as i32)));
            }
        }
    }

    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            if inp[i][j] < 0 {
                continue;
            }

            for dir in dirs.iter() {
                let mut t = (i as i32 + dir.0, j as i32 + dir.1);
                loop {
                    if t.0 >= inp.len() as i32 || t.1 >= inp[0].len() as i32 || t.0 < 0 || t.1 < 0 {
                        break;
                    }
                    if inp[t.0 as usize][t.1 as usize] >= 0 {
                        inp[i][j] += 1;
                        inp[t.0 as usize][t.1 as usize] += 1;
                        break;
                    }
                    t = (t.0 + dir.0, t.1 + dir.1);
                }
            }
        }
    }

    // print_space(&mut inp);
    
    let mut max = 0;
    let mut imax = 0;
    let mut jmax = 0;
    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            if inp[i][j] > max {
                max = inp[i][j];
                imax = i;
                jmax = j;
            }
        }
    }
    inp[imax][jmax] = -99;

    dirs.sort_by(|a, b| {
        if a.1 == 0 { Ordering::Greater }
        else if b.1 == 0 { Ordering::Less }
        else if (a.0 as f32 / a.1 as f32) > (b.0 as f32 / b.1 as f32) { Ordering::Less }
        else { Ordering::Greater }
    });
    
    for i in dirs.len()..0 {
        let (x, y) = dirs[i];
        dirs.push((-x, y));
    }
    
    println!("{:?}", dirs);
    
    let now = Instant::now();
    let mut shots = 0;
    loop {
        for dir in dirs.iter() {
            let mut t = (imax as i32 + dir.0, jmax as i32 + dir.1);
            loop {
                if t.0 >= inp.len() as i32 || t.1 >= inp[0].len() as i32 || t.0 < 0 || t.1 < 0 {
                    break;
                }
                if inp[t.0 as usize][t.1 as usize] > 0 {
                    shots += 1;
                    println!("{}: {} {} -> {} {}", shots, t.0, t.1, dir.0, dir.1);
                    if shots == 52 {
                        print_space(&inp);
                    }
                    if shots == 200 {
                        println!("200 = {} {}", t.0, t.1);
                        print_space(&inp);
                        return;
                    }
                    
                    inp[t.0 as usize][t.1 as usize] = 0;
                    break;
                }
                t = (t.0 + dir.0, t.1 + dir.1);
            }
        } 
    }

    


}
