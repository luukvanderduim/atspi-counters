use once_cell::sync::Lazy;
use std::{
    io::{BufWriter, Write},
    sync::Arc,
};

use crate::counters::CounterStats;

// Create a static filename atspi-stats-<timestamp>.txts
// Create a file with the filename
pub static FILENAME: Lazy<String> = Lazy::new(|| {
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    format!("atspi-stats-{}.txt", timestamp)
});

pub fn write_stats(stats: &[Arc<dyn CounterStats>]) {
    // Open the file
    let file = std::fs::File::create(&*FILENAME).unwrap();
    let mut writer = BufWriter::new(file);

    // Write the stats to the file
    for stat in stats {
        writeln!(writer, "Total events: {}", stat.total()).unwrap();
        for (cat, count) in stat.get_categories() {
            write!(
                writer,
                "{}: {},",
                cat,
                count.load(std::sync::atomic::Ordering::Relaxed)
            )
            .unwrap();
        }
        writeln!(writer).unwrap();
    }
}
