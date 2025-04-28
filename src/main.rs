use std::env;
use std::time::{Duration, Instant};

fn main() {
    // Defaults
    let mut iters = 10;
    let mut problems = Vec::new();

    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.peek().cloned() {
        if arg == "--iters" {
            args.next();
            iters = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                eprintln!("Invalid iteration count after --iters");
                std::process::exit(1);
            });
            break;
        }
        let num = args
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(|| {
                eprintln!("Invalid problem number: {}", arg);
                std::process::exit(1);
            });
        problems.push(num);
    }

    if problems.is_empty() {
        eprintln!("Usage: bench <problem> [<problem> ...] [--iters N]");
        std::process::exit(1);
    }

    for &problem in &problems {
        let solver = solve(problem).unwrap_or_else(|| {
            eprintln!("No solver for problem {}", problem);
            std::process::exit(1);
        });
        run_stats(problem, solver, iters);
    }
}

fn run_stats(problem: usize, solver: fn() -> usize, iters: usize) {
    let mut durations = Vec::with_capacity(iters);
    for _ in 0..iters {
        let start = Instant::now();
        let _ = solver();
        durations.push(start.elapsed());
    }

    let mut times: Vec<f64> = durations.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = times.first().copied().unwrap_or(0.0);
    let max = times.last().copied().unwrap_or(0.0);
    let sum: f64 = times.iter().sum();
    let mean = sum / (iters as f64);

    let median = if iters % 2 == 0 {
        let mid = iters / 2;
        (times[mid - 1] + times[mid]) / 2.0
    } else {
        times[iters / 2]
    };

    let var: f64 = times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / (iters as f64);
    let stddev = var.sqrt();

    println!(
        "Problem {:>2}: runs = {} | min = {:.3}ms | mean = {:.3}ms | median = {:.3}ms | max = {:.3}ms | stddev = {:.3}ms",
        problem, iters, min, mean, median, max, stddev
    );
}

fn solve(problem: usize) -> Option<fn() -> usize> {
    match problem {
        10 => Some(euler_10),
        11 => Some(euler_11),
        51 => Some(euler_51),
        _ => None,
    }
}

// 22 seconds
fn euler_10() -> usize {
    let mut primes: Vec<usize> = vec![];
    let mut checked_integers: Vec<usize> = (3..2000000).collect();
    let mut prime = true;
    while checked_integers.len() > 0 {
        prime = true;
        let u = checked_integers[0];
        let lim = u.isqrt();
        for &y in primes.iter().take_while(|a| a < &&lim) {
            if u % y == 0 && u != y {
                prime = false;
                break;
            }
        }
        if prime {
            checked_integers.retain(|a| a % u != 0);
            primes.push(u);
        }
    }
    primes.into_iter().sum()
}
// 5ms, created by chatgpt, not MINE but here for reference
fn euler_10_ai() -> usize {
    const LIM: usize = 2000000;
    let mut is_prime = vec![true; LIM];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut sum: u64 = 0;
    let sqrt_lim = (LIM as f64).sqrt() as usize;

    for i in 2..=sqrt_lim {
        if is_prime[i] {
            let mut j = i * i;
            while j < LIM {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    for (i, &prime) in is_prime.iter().enumerate() {
        if prime {
            sum += i as u64;
        }
    }

    sum as usize
}

fn euler_11() -> usize {
    let input_str_lines: Vec<&str> = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"
        .lines()
        .collect();
    let mut arr: [[usize; 20]; 20] = [[0; 20]; 20];
    for i in 0..19 {
        let split_lines: Vec<usize> = input_str_lines[i]
            .split_whitespace()
            .map(|b| b.parse::<usize>().unwrap())
            .collect();
        for y in 0..19 {
            arr[i][y] = split_lines[y];
        }
    }
    let (mut max, mut x, mut y, mut d, mut dl) = (0, 0, 0, 0, 0);
    // checks if y overflow or x overflow or left diagonal overflow, as right diagonal overflow is y_block | x_block
    let (mut y_block, mut x_block, mut d_block) = (false, false, true);
    let l = arr.len();
    for (i, row) in arr.clone().iter_mut().enumerate() {
        if !y_block && i > l - 4 {
            y_block = true;
        }
        for s in 0..row.len() {
            if s > row.len() - 4 {
                x_block = true;
            }
            if s > 4 {
                d_block = false;
            }
            if !(y_block | x_block) {
                d = row[s] * &arr[i + 1][s + 1] * &arr[i + 2][s + 2] * &arr[i + 3][s + 3];
            }
            if !(x_block) {
                x = row[s] * row[s + 1] * row[s + 2] * row[s + 3];
            }
            if !y_block {
                y = row[s] * &arr[i + 1][s] * &arr[i + 2][s] * &arr[i + 3][s];
            }
            if !(y_block | d_block) {
                dl = row[s] * &arr[i + 1][s - 1] * &arr[i + 2][s - 2] * &arr[i + 3][s - 3];
            }
            max = max.max(x).max(y).max(d).max(dl);
        }
        x_block = false;
        d_block = true;
    }
    max
}

// to get to correct row, we get val = y*20 + x;
pub fn euler_11_flat() -> usize {
    let input_str_lines: Vec<&str> = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"
        .lines()
        .collect();

    let mut arr: [usize; 400] = [0; 400];
    for i in 0..19 {
        let split_lines: Vec<usize> = input_str_lines[i]
            .split_whitespace()
            .map(|b| b.parse::<usize>().unwrap())
            .collect();
        for y in 0..19 {
            arr[20 * i + y] = split_lines[y];
        }
    }

    0
}

// i didn't read
pub fn euler_51() -> usize {
    let mut mult = 100;
    let mut primes = vec![true; mult];
    primes[0] = false;
    primes[1] = false;
    let mut digits;
    let mut index_val: usize = 2;
    let mut cp;
    let mut count = 0;

    loop {
        count = 0;
        for i in 2..primes.len() {
            if primes[i] {
                let mut j = i * i;
                while j < primes.len() {
                    primes[j] = false;
                    j += i;
                }
            }
        }
        while index_val < mult {
            if primes[index_val] {
                digits = vec![];
                cp = index_val;
                while cp > 0 {
                    digits.push(cp % 10);
                    cp /= 10;
                }
                digits.reverse();
                // n is how many digits to set as "wild";
                let l = digits.len().clone();
                for n in 2..l {
                    let mut idx: Vec<Vec<usize>> = vec![];
                    let mut data = vec![];
                    combination(0, n, l, &mut data, &mut idx);
                    for c in idx {
                        let mut res = vec![];
                        count = 0;
                        for i in 0..=9 {
                            data = digits.clone();
                            for v in c.clone().into_iter() {
                                data[v] = i;
                            }
                            cp = 0;
                            if data[0] == 0 {
                                continue;
                            }
                            for digit in data.into_iter() {
                                cp = cp * 10 + digit;
                            }

                            if primes[cp] {
                                res.push(cp);
                                count += 1;
                            }
                        }
                        if count == 8 {
                            res.sort();
                            return res[0];
                        }
                    }
                }
            }
            index_val += 1;
        }
        mult *= 100;
        if primes.len() < mult {
            primes.resize_with(mult, || true);
        }
    }
}

fn combination(i: usize, r: usize, d: usize, data: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
    if data.len() == r {
        res.push(data.clone());
        return;
    }

    for x in i..d {
        data.push(x);
        combination(x + 1, r, d, data, res);
        data.pop();
    }
}
