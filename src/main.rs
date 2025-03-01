use atspi::events::{
    AvailableEvent, CacheEvents, DocumentEvents, Event, EventListenerEvents, FocusEvents,
    KeyboardEvents, MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};
use std::{error::Error, sync::Arc};
use tokio_stream::StreamExt;
mod counters;
use counters::{CounterStats, InterfaceCount, ObjectCount, WindowCount};
mod writer;
use writer::write_stats;

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
    let term_count = Arc::new(InterfaceCount::new());
    let doc_count = Arc::new(InterfaceCount::new());
    let mouse_count = Arc::new(InterfaceCount::new());
    let cache_count = Arc::new(InterfaceCount::new());

    let ctrlc_iface_count = iface_count.clone();
    let ctrlc_obj_count = obj_count.clone();
    let ctrlc_win_count = win_count.clone();
    let ctrlc_term_count = term_count.clone();
    let ctrlc_doc_count = doc_count.clone();
    let ctrlc_mouse_count = mouse_count.clone();
    let ctrlc_cache_count = cache_count.clone();

    ctrlc::set_handler(move || {
        // let collections: &[Arc<dyn CounterStats>] = &[
        //     ctrlc_iface_count.clone(),
        //     ctrlc_obj_count.clone(),
        //     ctrlc_win_count.clone(),
        //     ctrlc_term_count.clone(),
        //     ctrlc_doc_count.clone(),
        //     ctrlc_mouse_count.clone(),
        //     ctrlc_cache_count.clone(),
        // ];
        // write_stats(collections);

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
                    WindowEvents::PropertyChange(_) => win_count.increment("property-change"),
                    WindowEvents::Minimize(_) => win_count.increment("minimize"),
                    WindowEvents::Maximize(_) => win_count.increment("maximize"),
                    WindowEvents::Restore(_) => win_count.increment("restore"),
                    WindowEvents::Close(_) => win_count.increment("close"),
                    WindowEvents::Create(_) => win_count.increment("create"),
                    WindowEvents::Reparent(_) => win_count.increment("reparent"),
                    WindowEvents::DesktopCreate(_) => win_count.increment("desktop-create"),
                    WindowEvents::DesktopDestroy(_) => win_count.increment("desktop-destroy"),
                    WindowEvents::Destroy(_) => win_count.increment("destroy"),
                    WindowEvents::Activate(_) => win_count.increment("activate"),
                    WindowEvents::Deactivate(_) => win_count.increment("deactivate"),
                    WindowEvents::Raise(_) => win_count.increment("raise"),
                    WindowEvents::Lower(_) => win_count.increment("lower"),
                    WindowEvents::Move(_) => win_count.increment("move"),
                    WindowEvents::Resize(_) => win_count.increment("resize"),
                    WindowEvents::Shade(_) => win_count.increment("shade"),
                    WindowEvents::UUshade(_) => win_count.increment("uushade"),
                    WindowEvents::Restyle(_) => win_count.increment("restyle"),
                };
            }
            Event::Document(doc_ev) => {
                iface_count.increment("document");
                match doc_ev {
                    DocumentEvents::LoadComplete(_) => doc_count.increment("load-complete"),
                    DocumentEvents::Reload(_) => doc_count.increment("reload"),
                    DocumentEvents::LoadStopped(_) => doc_count.increment("load-stopped"),
                    DocumentEvents::ContentChanged(_) => doc_count.increment("content-changed"),
                    DocumentEvents::AttributesChanged(_) => {
                        doc_count.increment("attributes-changed")
                    }
                    DocumentEvents::PageChanged(_) => doc_count.increment("page-changed"),
                };
            }
            Event::Terminal(term_ev) => {
                iface_count.increment("terminal");
                match term_ev {
                    TerminalEvents::LineChanged(_) => term_count.increment("line-changed"),
                    TerminalEvents::ColumnCountChanged(_) => {
                        term_count.increment("column-count-changed")
                    }
                    TerminalEvents::LineCountChanged(_) => {
                        term_count.increment("line-count-changed")
                    }
                    TerminalEvents::ApplicationChanged(_) => {
                        term_count.increment("application-changed")
                    }
                    TerminalEvents::CharWidthChanged(_) => {
                        term_count.increment("char-width-changed")
                    }
                };
            }

            Event::Mouse(mouse_ev) => {
                iface_count.increment("mouse");
                match mouse_ev {
                    MouseEvents::Abs(_) => mouse_count.increment("abs"),
                    MouseEvents::Rel(_) => mouse_count.increment("rel"),
                    MouseEvents::Button(_) => mouse_count.increment("button"),
                };
            }

            Event::Cache(cache_ev) => {
                iface_count.increment("cache");
                match cache_ev {
                    CacheEvents::Add(_) => cache_count.increment("add"),
                    CacheEvents::LegacyAdd(_) => cache_count.increment("legacy-add"),
                    CacheEvents::Remove(_) => cache_count.increment("remove"),
                };
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
