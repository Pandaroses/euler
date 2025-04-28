use std::{
    fs,
    io::{Read, Write},
    time::Instant,
};

pub fn run_stats(problem: usize, solver: fn() -> usize, iters: usize, write: bool) {
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
    if write {
        write_row_to_readme(problem, iters, min, mean, median, max, stddev);
    }
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
    let marker_start = "<!-- BENCHMARK_TABLE_START -->";
    let marker_end = "<!-- BENCHMARK_TABLE_END -->";

    // Load README
    let mut buf = String::new();
    fs::File::open("README.md")
        .and_then(|mut f| f.read_to_string(&mut buf))
        .expect("README.md not found");

    // Find the two markers
    let start = buf.find(marker_start).expect("start marker");
    let end = buf.find(marker_end).expect("end marker");

    // Split into three parts
    let head = &buf[..start + marker_start.len()];
    let tail = &buf[end..];
    let block = &buf[start + marker_start.len()..end];

    // 1) Capture header & separator as-is
    let mut lines = block.lines();
    let header_line = lines.next().unwrap_or("");
    let separator_line = lines.next().unwrap_or("");

    // 2) Parse existing data rows
    let mut entries: Vec<(usize, String)> = lines
        .filter(|l| l.trim_start().starts_with('|'))
        .filter_map(|l| {
            let cols: Vec<&str> = l.trim().split('|').collect();
            cols.get(1)
                .and_then(|s| s.trim().parse::<usize>().ok())
                .map(|num| (num, l.trim().to_string()))
        })
        .collect();

    // 3) Create your new row
    let new_row = format!(
        "| {:>2} | {:>4} | {:>7.3} | {:>8.3} | {:>9.3} | {:>7.3} | {:>11.3} |",
        problem, iters, min, mean, median, max, stddev
    );
    // Remove any old entry & insert the new one
    entries.retain(|&(n, _)| n != problem);
    entries.push((problem, new_row));
    entries.sort_by_key(|&(n, _)| n);

    // 4) Rebuild the block
    let mut new_block = String::new();
    new_block.push_str("\n"); // ensure a clean linebreak
    new_block.push_str(header_line);
    new_block.push('\n');
    new_block.push_str(separator_line);
    new_block.push('\n');
    for &(_, ref row) in &entries {
        new_block.push_str(row);
        new_block.push('\n');
    }
    new_block.push_str("\n"); // pad before end marker

    // 5) Write it all back
    let new_readme = format!("{}{}{}", head, new_block, tail);
    fs::write("README.md", new_readme).expect("failed to write README.md");
}
