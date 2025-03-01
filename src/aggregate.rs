// use std::io::{BufRead, BufReader};
// use std::io::{BufWriter, Write};

// // Read the stats from all files.
// // Report how many files were read.

// pub fn read_stats() -> Result<(), Box<dyn std::error::Error>> {
//     let mut files_read = 0;
//     let mut total_events = 0;
//     let mut total_categories = 0;
//     let mut total_lines = 0;

//     let mut stats: Vec<(String, Vec<(String, u32)>)> = Vec::new();

//     for entry in std::fs::read_dir(".")? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             let file = std::fs::File::open(&path)?;
//             let reader = BufReader::new(file);
//             let mut lines = reader.lines();
//             let mut file_stats: Vec<(String, u32)> = Vec::new();
//             let mut total = 0;
//             let mut categories = 0;
//             let mut file_lines = 0;

//             while let Some(line) = lines.next() {
//                 let line = line?;
//                 file_lines += 1;
//                 if file_lines == 1 {
//                     total = line.split_whitespace().last().unwrap().parse()?;
//                 } else {
//                     let mut parts = line.split(": ");
//                     let category = parts.next().unwrap().to_string();
//                     let count = parts.next().unwrap().parse()?;
//                     file_stats.push((category, count));
//                     categories += 1;
//                 }
//             }

//             stats.push((path.to_string_lossy().to_string(), file_stats));
//             files_read += 1;
//             total_events += total;
//             total_categories += categories;
//             total_lines += file_lines;
//         }
//     }

//     println!("Files read: {}", files_read);
//     println!("Total events: {}", total_events);
//     println!("Total categories: {}", total_categories);
//     println!("Total lines: {}", total_lines);

//     for (file, file_stats) in stats {
//         println!("File: {}", file);
//         for (category, count) in file_stats {
//             println!("{}: {}", category, count);
//         }
//     }

//     Ok(())
// }
