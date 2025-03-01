#![allow(unused)]
use atspi::events::{
    AvailableEvent, CacheEvents, DocumentEvents, Event, EventListenerEvents, FocusEvents,
    KeyboardEvents, MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};
use std::sync::atomic::AtomicU32;

pub trait Getters {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)];
    fn get_total(&self) -> &AtomicU32;
}

pub trait CounterStats
where
    Self: Getters,
{
    fn increment(&self, category: &'static str) {
        self.get_total()
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        for (cat, count) in self.get_categories() {
            if *cat == category {
                count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    fn total(&self) -> u32 {
        self.get_total().load(std::sync::atomic::Ordering::Relaxed)
    }

    fn pretty_print_stats(&self) {
        println!("Total events: {}", self.total());
        let mut stats: Vec<(&'static str, u32)> = self
            .get_categories()
            .iter()
            .map(|(cat, count)| (*cat, count.load(std::sync::atomic::Ordering::Relaxed)))
            .collect();
        stats.sort_by(|a, b| b.1.cmp(&a.1));

        for (cat, count) in stats {
            let percentage = if self.total() > 0 {
                (count as f32 / self.total() as f32) * 100.0
            } else {
                0.0
            };
            // Position these in fixed width columns
            print!("{:<30}", cat);
            print!("{:<10}", count);
            println!("{:.2}%", percentage);
        }
    }
}

pub struct InterfaceCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 10],
}

impl InterfaceCount {
    pub fn new() -> Self {
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
}

impl Getters for InterfaceCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for InterfaceCount {}

pub struct ObjectCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 22],
}

impl ObjectCount {
    pub fn new() -> Self {
        ObjectCount {
            total: AtomicU32::new(0),
            categories: [
                ("property-change", AtomicU32::new(0)),
                ("bounds-changed", AtomicU32::new(0)),
                ("link-selected", AtomicU32::new(0)),
                ("state-changed", AtomicU32::new(0)),
                ("children-changed", AtomicU32::new(0)),
                ("visible-data-changed", AtomicU32::new(0)),
                ("selection-changed", AtomicU32::new(0)),
                ("model-changed", AtomicU32::new(0)),
                ("active-descendant-changed", AtomicU32::new(0)),
                ("announcement", AtomicU32::new(0)),
                ("attributes-changed", AtomicU32::new(0)),
                ("row-inserted", AtomicU32::new(0)),
                ("row-reordered", AtomicU32::new(0)),
                ("row-deleted", AtomicU32::new(0)),
                ("column-inserted", AtomicU32::new(0)),
                ("column-reordered", AtomicU32::new(0)),
                ("column-deleted", AtomicU32::new(0)),
                ("text-bounds-changed", AtomicU32::new(0)),
                ("text-selection-changed", AtomicU32::new(0)),
                ("text-changed", AtomicU32::new(0)),
                ("text-attributes-changed", AtomicU32::new(0)),
                ("text-caret-moved", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for ObjectCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for ObjectCount {}

pub struct WindowCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 19],
}

impl WindowCount {
    pub fn new() -> Self {
        WindowCount {
            total: AtomicU32::new(0),
            categories: [
                ("property-change", AtomicU32::new(0)),
                ("minimize", AtomicU32::new(0)),
                ("maximize", AtomicU32::new(0)),
                ("restore", AtomicU32::new(0)),
                ("close", AtomicU32::new(0)),
                ("create", AtomicU32::new(0)),
                ("reparent", AtomicU32::new(0)),
                ("desktop-create", AtomicU32::new(0)),
                ("desktop-destroy", AtomicU32::new(0)),
                ("destroy", AtomicU32::new(0)),
                ("activate", AtomicU32::new(0)),
                ("deactivate", AtomicU32::new(0)),
                ("raise", AtomicU32::new(0)),
                ("lower", AtomicU32::new(0)),
                ("move", AtomicU32::new(0)),
                ("resize", AtomicU32::new(0)),
                ("shade", AtomicU32::new(0)),
                ("uushade", AtomicU32::new(0)),
                ("restyle", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for WindowCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for WindowCount {}

pub struct TerminalCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 5],
}

impl TerminalCount {
    pub fn new() -> Self {
        TerminalCount {
            total: AtomicU32::new(0),
            categories: [
                ("line-changed", AtomicU32::new(0)),
                ("column-count-changed", AtomicU32::new(0)),
                ("line-count-changed", AtomicU32::new(0)),
                ("application-changed", AtomicU32::new(0)),
                ("char-width-changed", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for TerminalCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for TerminalCount {}

pub struct MouseCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 3],
}

impl MouseCount {
    pub fn new() -> Self {
        MouseCount {
            total: AtomicU32::new(0),
            categories: [
                ("abs", AtomicU32::new(0)),
                ("rel", AtomicU32::new(0)),
                ("button", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for MouseCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for MouseCount {}

pub struct DocumentCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 6],
}

impl DocumentCount {
    pub fn new() -> Self {
        DocumentCount {
            total: AtomicU32::new(0),
            categories: [
                ("load-complete", AtomicU32::new(0)),
                ("reload", AtomicU32::new(0)),
                ("load-stopped", AtomicU32::new(0)),
                ("content-changed", AtomicU32::new(0)),
                ("attributes-changed", AtomicU32::new(0)),
                ("page-changed", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for DocumentCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for DocumentCount {}

pub struct CacheCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 3],
}

impl CacheCount {
    pub fn new() -> Self {
        CacheCount {
            total: AtomicU32::new(0),
            categories: [
                ("add", AtomicU32::new(0)),
                ("legacy-add", AtomicU32::new(0)),
                ("remove", AtomicU32::new(0)),
            ],
        }
    }
}

impl Getters for CacheCount {
    fn get_categories(&self) -> &[(&'static str, AtomicU32)] {
        &self.categories
    }

    fn get_total(&self) -> &AtomicU32 {
        &self.total
    }
}

impl CounterStats for CacheCount {}
