use atspi::events::{
    AvailableEvent, CacheEvents, DocumentEvents, Event, EventListenerEvents, FocusEvents,
    KeyboardEvents, MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};
use std::{
    error::Error,
    sync::{Arc, atomic::AtomicU32},
};
use tokio_stream::StreamExt;

struct InterfaceCount {
    total: AtomicU32,

    categories: [(&'static str, AtomicU32); 10],
}

impl InterfaceCount {
    fn new() -> Self {
        InterfaceCount {
            total: AtomicU32::new(0),
            categories: [
                ("object", AtomicU32::new(0)),
                ("window", AtomicU32::new(0)),
                ("document", AtomicU32::new(0)),
                ("terminal", AtomicU32::new(0)),
                ("mouse", AtomicU32::new(0)),
                ("keyboard", AtomicU32::new(0)),
                ("listener", AtomicU32::new(0)),
                ("cache", AtomicU32::new(0)),
                ("focus", AtomicU32::new(0)),
                ("available", AtomicU32::new(0)),
            ],
        }
    }

    /// Increment the count for the given category
    fn increment(&self, category: &'static str) {
        self.total
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        for (cat, count) in &self.categories {
            if *cat == category {
                count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return;
            }
        }
    }

    /// Get the total count
    fn total(&self) -> u32 {
        self.total.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Pretty print the stats
    /// Order by percentage of total    
    fn pretty_print_stats(&self) {
        println!("Total events: {}", self.total());
        let mut stats: Vec<(&'static str, u32)> = self
            .categories
            .iter()
            .map(|(cat, count)| (*cat, count.load(std::sync::atomic::Ordering::Relaxed)))
            .collect();
        stats.sort_by(|a, b| b.1.cmp(&a.1));

        for (cat, count) in stats {
            let percentage = (count as f32 / self.total() as f32) * 100.0;
            println!("{}: {} ({}%)", cat, count, percentage);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let atspi = atspi::AccessibilityConnection::new().await?;
    atspi.register_event::<ObjectEvents>().await?;
    atspi.register_event::<WindowEvents>().await?;
    atspi.register_event::<DocumentEvents>().await?;
    atspi.register_event::<TerminalEvents>().await?;
    atspi.register_event::<MouseEvents>().await?;
    atspi.register_event::<KeyboardEvents>().await?;
    atspi.register_event::<EventListenerEvents>().await?;
    atspi.register_event::<CacheEvents>().await?;
    atspi.register_event::<FocusEvents>().await?;
    atspi.register_event::<AvailableEvent>().await?;

    let events = atspi.event_stream();
    tokio::pin!(events);

    let count = Arc::new(InterfaceCount::new());
    let ctrlc_count = count.clone();

    ctrlc::set_handler(move || {
        println!("\n\nStats:");
        ctrlc_count.pretty_print_stats();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Press Ctrl+C to stop the program and get stats");

    while let Some(Ok(ev)) = events.next().await {
        match ev {
            Event::Object(_) => count.increment("object"),
            Event::Window(_) => count.increment("window"),
            Event::Document(_) => count.increment("document"),
            Event::Terminal(_) => count.increment("terminal"),
            Event::Mouse(_) => count.increment("mouse"),
            Event::Keyboard(_) => count.increment("keyboard"),
            Event::Listener(_) => count.increment("listener"),
            Event::Cache(_) => count.increment("cache"),
            Event::Focus(_) => count.increment("focus"),
            Event::Available(_) => count.increment("available"),
            _ => {}
        }
    }
    Ok(())
}
