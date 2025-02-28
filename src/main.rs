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

struct ObjectCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 22],
}

impl ObjectCount {
    fn new() -> Self {
        ObjectCount {
            total: AtomicU32::new(0),
            categories: [
                ("object:property-change", AtomicU32::new(0)),
                ("object:bounds-changed", AtomicU32::new(0)),
                ("object:link-selected", AtomicU32::new(0)),
                ("object:state-changed", AtomicU32::new(0)),
                ("object:children-changed", AtomicU32::new(0)),
                ("object:visible-data-changed", AtomicU32::new(0)),
                ("object:selection-changed", AtomicU32::new(0)),
                ("object:model-changed", AtomicU32::new(0)),
                ("object:active-descendant-changed", AtomicU32::new(0)),
                ("object:announcement", AtomicU32::new(0)),
                ("object:attributes-changed", AtomicU32::new(0)),
                ("object:row-inserted", AtomicU32::new(0)),
                ("object:row-reordered", AtomicU32::new(0)),
                ("object:row-deleted", AtomicU32::new(0)),
                ("object:column-inserted", AtomicU32::new(0)),
                ("object:column-reordered", AtomicU32::new(0)),
                ("object:column-deleted", AtomicU32::new(0)),
                ("object:text-bounds-changed", AtomicU32::new(0)),
                ("object:text-selection-changed", AtomicU32::new(0)),
                ("object:text-changed", AtomicU32::new(0)),
                ("object:text-attributes-changed", AtomicU32::new(0)),
                ("object:text-caret-moved", AtomicU32::new(0)),
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
        println!("\n\nTotal object events: {}", self.total());
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

struct WindowCount {
    total: AtomicU32,
    categories: [(&'static str, AtomicU32); 19],
}

impl WindowCount {
    fn new() -> Self {
        WindowCount {
            total: AtomicU32::new(0),
            categories: [
                ("window:property-change", AtomicU32::new(0)),
                ("window:minimize", AtomicU32::new(0)),
                ("window:maximize", AtomicU32::new(0)),
                ("window:restore", AtomicU32::new(0)),
                ("window:close", AtomicU32::new(0)),
                ("window:create", AtomicU32::new(0)),
                ("window:reparent", AtomicU32::new(0)),
                ("window:desktop-create", AtomicU32::new(0)),
                ("window:desktop-destroy", AtomicU32::new(0)),
                ("window:destroy", AtomicU32::new(0)),
                ("window:activate", AtomicU32::new(0)),
                ("window:deactivate", AtomicU32::new(0)),
                ("window:raise", AtomicU32::new(0)),
                ("window:lower", AtomicU32::new(0)),
                ("window:move", AtomicU32::new(0)),
                ("window:resize", AtomicU32::new(0)),
                ("window:shade", AtomicU32::new(0)),
                ("window:uushade", AtomicU32::new(0)),
                ("window:restyle", AtomicU32::new(0)),
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
        println!("\n\nTotal window events: {}", self.total());
        let mut stats: Vec<(&'static str, u32)> = self
            .categories
            .iter()
            .map(|(cat, count)| (*cat, count.load(std::sync::atomic::Ordering::Relaxed)))
            .collect();
        stats.sort_by(|a, b| b.1.cmp(&a.1));
        stats.sort_by(|a, b| b.1.cmp(&a.1));

        for (cat, count) in stats {
            let percentage = (count as f32 / self.total() as f32) * 100.0;
            println!("{}: {} ({}%)", cat, count, percentage);
        }
    }
}

// PropertyChange(WindowPropertyChangeEvent),
// Minimize(MinimizeEvent),
// Maximize(MaximizeEvent),
// Restore(RestoreEvent),
// Close(CloseEvent),
// Create(CreateEvent),
// Reparent(ReparentEvent),
// DesktopCreate(DesktopCreateEvent),
// DesktopDestroy(DesktopDestroyEvent),
// Destroy(DestroyEvent),
// Activate(ActivateEvent),
// Deactivate(DeactivateEvent),
// Raise(RaiseEvent),
// Lower(LowerEvent),
// Move(MoveEvent),
// Resize(ResizeEvent),
// Shade(ShadeEvent),
// UUshade(UUshadeEvent),
// Restyle(RestyleEvent),

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

    let iface_count = Arc::new(InterfaceCount::new());
    let obj_count = Arc::new(ObjectCount::new());
    let win_count = Arc::new(WindowCount::new());

    let ctrlc_iface_count = iface_count.clone();
    let ctrl_c_obj_count = obj_count.clone();
    let ctrl_c_win_count = win_count.clone();

    ctrlc::set_handler(move || {
        println!("\n\nStats:");
        ctrlc_iface_count.pretty_print_stats();
        ctrl_c_obj_count.pretty_print_stats();
        ctrl_c_win_count.pretty_print_stats();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Press Ctrl+C to stop the program and get stats");

    while let Some(Ok(ev)) = events.next().await {
        match ev {
            Event::Object(objev) => {
                iface_count.increment("object");
                match objev {
                    ObjectEvents::PropertyChange(_) => {
                        obj_count.increment("object:property-change")
                    }
                    ObjectEvents::BoundsChanged(_) => obj_count.increment("object:bounds-changed"),
                    ObjectEvents::LinkSelected(_) => obj_count.increment("object:link-selected"),
                    ObjectEvents::StateChanged(_) => obj_count.increment("object:state-changed"),
                    ObjectEvents::ChildrenChanged(_) => {
                        obj_count.increment("object:children-changed")
                    }
                    ObjectEvents::VisibleDataChanged(_) => {
                        obj_count.increment("object:visible-data-changed")
                    }
                    ObjectEvents::SelectionChanged(_) => {
                        obj_count.increment("object:selection-changed")
                    }
                    ObjectEvents::ModelChanged(_) => obj_count.increment("object:model-changed"),
                    ObjectEvents::ActiveDescendantChanged(_) => {
                        obj_count.increment("object:active-descendant-changed")
                    }
                    ObjectEvents::Announcement(_) => obj_count.increment("object:announcement"),
                    ObjectEvents::AttributesChanged(_) => {
                        obj_count.increment("object:attributes-changed")
                    }
                    ObjectEvents::RowInserted(_) => obj_count.increment("object:row-inserted"),
                    ObjectEvents::RowReordered(_) => obj_count.increment("object:row-reordered"),
                    ObjectEvents::RowDeleted(_) => obj_count.increment("object:row-deleted"),
                    ObjectEvents::ColumnInserted(_) => {
                        obj_count.increment("object:column-inserted")
                    }
                    ObjectEvents::ColumnReordered(_) => {
                        obj_count.increment("object:column-reordered")
                    }
                    ObjectEvents::ColumnDeleted(_) => obj_count.increment("object:column-deleted"),
                    ObjectEvents::TextBoundsChanged(_) => {
                        obj_count.increment("object:text-bounds-changed")
                    }
                    ObjectEvents::TextSelectionChanged(_) => {
                        obj_count.increment("object:text-selection-changed")
                    }
                    ObjectEvents::TextChanged(_) => obj_count.increment("object:text-changed"),
                    ObjectEvents::TextAttributesChanged(_) => {
                        obj_count.increment("object:text-attributes-changed")
                    }
                    ObjectEvents::TextCaretMoved(_) => {
                        obj_count.increment("object:text-caret-moved")
                    }
                }
            }

            Event::Window(wev) => {
                iface_count.increment("window");
                match wev {
                    WindowEvents::PropertyChange(_) => {
                        win_count.increment("window:property-change")
                    }
                    WindowEvents::Minimize(_) => win_count.increment("window:minimize"),
                    WindowEvents::Maximize(_) => win_count.increment("window:maximize"),
                    WindowEvents::Restore(_) => win_count.increment("window:restore"),
                    WindowEvents::Close(_) => win_count.increment("window:close"),
                    WindowEvents::Create(_) => win_count.increment("window:create"),
                    WindowEvents::Reparent(_) => win_count.increment("window:reparent"),
                    WindowEvents::DesktopCreate(_) => win_count.increment("window:desktop-create"),
                    WindowEvents::DesktopDestroy(_) => {
                        win_count.increment("window:desktop-destroy")
                    }
                    WindowEvents::Destroy(_) => win_count.increment("window:destroy"),
                    WindowEvents::Activate(_) => win_count.increment("window:activate"),
                    WindowEvents::Deactivate(_) => win_count.increment("window:deactivate"),
                    WindowEvents::Raise(_) => win_count.increment("window:raise"),
                    WindowEvents::Lower(_) => win_count.increment("window:lower"),
                    WindowEvents::Move(_) => win_count.increment("window:move"),
                    WindowEvents::Resize(_) => win_count.increment("window:resize"),
                    WindowEvents::Shade(_) => win_count.increment("window:shade"),
                    WindowEvents::UUshade(_) => win_count.increment("window:uushade"),
                    WindowEvents::Restyle(_) => win_count.increment("window:restyle"),
                };
            }
            Event::Document(_) => iface_count.increment("document"),
            Event::Terminal(_) => iface_count.increment("terminal"),
            Event::Mouse(_) => iface_count.increment("mouse"),
            Event::Keyboard(_) => iface_count.increment("keyboard"),
            Event::Listener(_) => iface_count.increment("listener"),
            Event::Cache(_) => iface_count.increment("cache"),
            Event::Focus(_) => iface_count.increment("focus"),
            Event::Available(_) => iface_count.increment("available"),
            _ => {}
        }
    }
    Ok(())
}
