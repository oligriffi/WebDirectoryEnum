use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::env;

use reqwest::blocking::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];
    let url = &args[2];
    let start_line = args.get(3).and_then(|x| x.parse::<usize>().ok()).unwrap_or(1);

    let wordlist = read_wordlist(wordlist_file);
    let client = Client::new();

    let start_time = Instant::now();
    let total_lines = wordlist.len();
    for (i, word) in wordlist.iter().enumerate().skip(start_line - 1) {
        let target = format!("{}/{}", url, word);
        let res = client.get(&target).send();

        if let Ok(response) = res {
            if !response.status().is_client_error() {
                println!("{} - Line {}: {}", response.status(), i + 1, target);
                continue;
            }
            if response.status().as_u16() == 404 {
                let res_without_replace = client.get(&format!("{}/{}", url, word)).send();
                if let Ok(response_without_replace) = res_without_replace {
                    if !response_without_replace.status().is_client_error() {
                        println!("{} - Line {}: {}", response_without_replace.status(), i + 1, target);
                        continue;
                    }
                }
                if word.contains("-") {
                    let new_word = word.replace("-", "_");
                    let new_target = format!("{}/{}", url, new_word);
                    let new_res = client.get(&new_target).send();

                    if let Ok(new_response) = new_res {
                        if !new_response.status().is_client_error() {
                            println!("{} - Line {}: {} (retry with _)", new_response.status(), i + 1, new_target);
                            continue;
                        }
                    }
                } else if word.contains("_") {
                    let new_word = word.replace("_", "-");
                    let new_target = format!("{}/{}", url, new_word);
                    let new_res = client.get(&new_target).send();

                    if let Ok(new_response) = new_res {
                        if !new_response.status().is_client_error() {
                            println!("{} - Line {}: {} (retry with -)", new_response.status(), i + 1, new_target);
                            continue;
                        }
                    }
                }
            }
        }

        // Print the progress in a single line
        print!("\rChecked {} of {} lines ", i + 1, total_lines);
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("\n\nFinished in {} seconds", elapsed);
}

fn read_wordlist(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .collect()
}

