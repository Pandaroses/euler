use std::{
    fs,
    io::{Read, Write},
    time::Instant,
};

pub fn run_stats(problem: usize, solver: fn() -> usize, iters: usize) {
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
    write_row_to_readme(problem, iters, min, mean, median, max, stddev);
}

fn write_row_to_readme(
    problem: usize,
    iters: usize,
    min: f64,
    mean: f64,
    median: f64,
    max: f64,
    stddev: f64,
) {
    // Format a markdown row
    let new_row = format!(
        "| {:>2} | {:>4} | {:>7.3} | {:>8.3} | {:>9.3} | {:>7.3} | {:>11.3} |",
        problem, iters, min, mean, median, max, stddev
    );

    // Load README.md
    let mut contents = String::new();
    fs::File::open("README.md")
        .and_then(|mut f| f.read_to_string(&mut contents))
        .expect("README.md not found");

    let start_marker = "<!-- BENCHMARK_TABLE_START -->";
    let end_marker = "<!-- BENCHMARK_TABLE_END -->";

    if let (Some(start_idx), Some(end_idx)) =
        (contents.find(start_marker), contents.find(end_marker))
    {
        let before = &contents[..start_idx + start_marker.len()];
        let table_section = &contents[start_idx + start_marker.len()..end_idx];
        let after = &contents[end_idx..];

        // Parse existing rows into (num, line)
        let mut entries: Vec<(usize, String)> = table_section
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with('|') {
                    let cols: Vec<&str> = trimmed.split('|').collect();
                    if cols.len() > 1 {
                        if let Ok(num) = cols[1].trim().parse::<usize>() {
                            return Some((num, trimmed.to_string()));
                        }
                    }
                }
                None
            })
            .collect();

        // Insert or replace this problem's row
        entries.retain(|&(num, _)| num != problem);
        entries.push((problem, new_row));

        // Sort by problem number
        entries.sort_by_key(|&(num, _)| num);

        // Rebuild section
        let new_section: String = entries
            .into_iter()
            .map(|(_, line)| format!("{}\n", line))
            .collect();

        // Write back
        let new_contents = format!("{}\n{}{}", before, new_section.trim_end(), after);
        fs::File::create("README.md")
            .and_then(|mut f| f.write_all(new_contents.as_bytes()))
            .expect("Unable to write README.md");
    } else {
        eprintln!("README.md markers not found—cannot update table");
    }
}
