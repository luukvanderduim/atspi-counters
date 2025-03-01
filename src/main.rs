use atspi::Event;
use std::{error::Error, sync::Arc};
use tokio_stream::StreamExt;
mod counters;
use clap::Parser;
use counters::{
    CacheCount, CounterStats, DocumentCount, InterfaceCount, MouseCount, ObjectCount,
    TerminalCount, WindowCount,
};

mod writer;
use writer::write_stats;
mod matchers;
use matchers::*;
mod setup;
use setup::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Aggregate the stats files and print the total stats
    #[arg(short, long)]
    aggregate: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let atspi = setup().await?;

    if args.aggregate {
        todo!();
    }

    let events = atspi.event_stream();
    tokio::pin!(events);

    let iface_count = Arc::new(InterfaceCount::new());
    let obj_count = Arc::new(ObjectCount::new());
    let win_count = Arc::new(WindowCount::new());
    let term_count = Arc::new(TerminalCount::new());
    let doc_count = Arc::new(DocumentCount::new());
    let mouse_count = Arc::new(MouseCount::new());
    let cache_count = Arc::new(CacheCount::new());

    let ctrlc_iface_count = iface_count.clone();
    let ctrlc_obj_count = obj_count.clone();
    let ctrlc_win_count = win_count.clone();
    let ctrlc_term_count = term_count.clone();
    let ctrlc_doc_count = doc_count.clone();
    let ctrlc_mouse_count = mouse_count.clone();
    let ctrlc_cache_count = cache_count.clone();

    ctrlc::set_handler(move || {
        let collections: &[Arc<dyn CounterStats>] = &[
            ctrlc_iface_count.clone(),
            ctrlc_obj_count.clone(),
            ctrlc_win_count.clone(),
            ctrlc_term_count.clone(),
            ctrlc_doc_count.clone(),
            ctrlc_mouse_count.clone(),
            ctrlc_cache_count.clone(),
        ];
        write_stats(collections);

        println!("\n\nStats:");
        println!("Interface stats:");
        ctrlc_iface_count.pretty_print_stats();

        println!("\nObject stats:");
        ctrlc_obj_count.pretty_print_stats();

        println!("\nWindow stats:");
        ctrlc_win_count.pretty_print_stats();

        println!("\nTerminal stats:");
        ctrlc_term_count.pretty_print_stats();

        println!("\nDocument stats:");
        ctrlc_doc_count.pretty_print_stats();

        println!("\nMouse stats:");
        ctrlc_mouse_count.pretty_print_stats();

        println!("\nCache stats:");
        ctrlc_cache_count.pretty_print_stats();

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Press Ctrl+C to stop the program and get stats");

    while let Some(Ok(ev)) = events.next().await {
        match ev {
            Event::Object(objev) => {
                iface_count.increment("object");
                match_object_events(objev, &obj_count);
            }
            Event::Window(wev) => {
                iface_count.increment("window");
                match_windowevents(wev, &win_count);
            }
            Event::Document(dev) => {
                iface_count.increment("document");
                match_documentevents(dev, &doc_count);
            }
            Event::Terminal(term_ev) => {
                iface_count.increment("terminal");
                match_terminal_events(term_ev, &term_count);
            }
            Event::Mouse(mev) => {
                iface_count.increment("mouse");
                match_mouse_events(mev, &mouse_count);
            }

            Event::Cache(cev) => {
                iface_count.increment("cache");
                match_cache_events(cev, &cache_count);
            }

            Event::Keyboard(_) => iface_count.increment("keyboard"),
            Event::Listener(_) => iface_count.increment("listener"),
            Event::Focus(_) => iface_count.increment("focus"),
            Event::Available(_) => iface_count.increment("available"),
            _ => {}
        }
    }
    Ok(())
}
