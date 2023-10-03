extern crate rayon;
extern crate colored;
extern crate walkdir;
extern crate clap;
extern crate indicatif;
extern crate serde_json;

use rayon::prelude::*;
use std::sync::{Mutex, Arc};
use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead, Write};
use colored::*;
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_derive::Serialize;

#[derive(Serialize)]
struct ReportItem {
    str1: String,
    str2: String,
    file1: String,
    file2: String,
    distance: i32,
}

fn levenshtein(str1: &str, str2: &str) -> i32 {
    let m = str1.len();
    let n = str2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if &str1[i - 1..i] == &str2[j - 1..j] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[m][n]
}

fn main() -> io::Result<()> {
    let matches = App::new("Levenshtein Distance Calculator")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("DIRECTORY")
            .help("Sets the directory to process")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("distance")
            .short("d")
            .long("distance")
            .value_name("MAX_DISTANCE")
            .help("Sets the maximum Levenshtein distance")
            .required(false)
            .default_value("5")
            .takes_value(true))
        .arg(Arg::with_name("json")
            .short("j")
            .long("json")
            .help("Output the report as a JSON file")
            .takes_value(false))
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let max_distance: i32 = matches.value_of("distance").unwrap().parse().expect("Invalid distance value");
    let output_json = matches.is_present("json");

    let mut strings = Vec::new();
    let distances: Arc<Mutex<Vec<(String, String, String, String, i32)>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Processing files in directory: {}", path);

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_path = path.to_str().unwrap_or("Unknown").to_string();
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(valid_line) = line {
                    strings.push((valid_line, file_path.clone()));
                }
            }
        }
    }

    println!("Calculating Levenshtein distances...");
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} [{elapsed_precise}] {msg}"));
    progress_bar.set_message("Calculating Levenshtein distances...");
    progress_bar.enable_steady_tick(100);

    strings.par_iter().enumerate().for_each(|(i, (str1, file1))| {
        let mut cache = distances.lock().unwrap();
        for j in i + 1..strings.len() {
            let (str2, file2) = &strings[j];
            let distance = levenshtein(str1, str2);
            if distance <= max_distance {
                cache.push((str1.clone(), str2.clone(), file1.clone(), file2.clone(), distance));
            }
            progress_bar.tick();
        }
    });

    progress_bar.finish_with_message("Done calculating Levenshtein distances.");
    println!("Sorting and printing results...");

    let mut sorted_distances = distances.lock().unwrap().clone();
    sorted_distances.sort_by(|a, b| a.4.cmp(&b.4));

    if output_json {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();
        let json_filename = format!("levenshtein_report_{}.json", timestamp);

        let report: Vec<ReportItem> = sorted_distances.iter().map(|(str1, str2, file1, file2, distance)| {
            ReportItem {
                str1: str1.clone(),
                str2: str2.clone(),
                file1: file1.clone(),
                file2: file2.clone(),
                distance: *distance,
            }
        }).collect();

        let json_string = serde_json::to_string(&report).expect("Failed to convert to JSON");
        let mut json_file = File::create(&json_filename).expect("Failed to create JSON file");
        write!(json_file, "{}", json_string).expect("Failed to write to JSON file");
        println!("Report saved as {}", json_filename);
    } else {
        println!("{}", "Levenshtein Distance Report".bold().underline());
        for (str1, str2, file1, file2, distance) in sorted_distances.iter() {
            println!(
                "Similar: {} (from {}) <-> {} (from {}) : {}",
                str1.red().bold(),
                file1,
                str2.blue().bold(),
                file2,
                distance.to_string().green().bold()
            );
        }
        println!("{}", "End of Report".bold().underline());
    }

    Ok(())
}
