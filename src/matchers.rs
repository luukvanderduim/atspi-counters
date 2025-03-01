use crate::counters::{
    CacheCount, CounterStats, DocumentCount, MouseCount, ObjectCount, TerminalCount, WindowCount,
};
use atspi::events::{
    CacheEvents, DocumentEvents, MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};
use std::sync::Arc;

pub fn match_object_events(oev: ObjectEvents, obj_count: &Arc<ObjectCount>) {
    match oev {
        ObjectEvents::PropertyChange(_) => obj_count.increment("property-change"),
        ObjectEvents::BoundsChanged(_) => obj_count.increment("bounds-changed"),
        ObjectEvents::LinkSelected(_) => obj_count.increment("link-selected"),
        ObjectEvents::StateChanged(_) => obj_count.increment("state-changed"),
        ObjectEvents::ChildrenChanged(_) => obj_count.increment("children-changed"),
        ObjectEvents::VisibleDataChanged(_) => obj_count.increment("visible-data-changed"),
        ObjectEvents::SelectionChanged(_) => obj_count.increment("selection-changed"),
        ObjectEvents::ModelChanged(_) => obj_count.increment("model-changed"),
        ObjectEvents::ActiveDescendantChanged(_) => {
            obj_count.increment("active-descendant-changed")
        }
        ObjectEvents::Announcement(_) => obj_count.increment("announcement"),
        ObjectEvents::AttributesChanged(_) => obj_count.increment("attributes-changed"),
        ObjectEvents::RowInserted(_) => obj_count.increment("row-inserted"),
        ObjectEvents::RowReordered(_) => obj_count.increment("row-reordered"),
        ObjectEvents::RowDeleted(_) => obj_count.increment("row-deleted"),
        ObjectEvents::ColumnInserted(_) => obj_count.increment("column-inserted"),
        ObjectEvents::ColumnReordered(_) => obj_count.increment("column-reordered"),
        ObjectEvents::ColumnDeleted(_) => obj_count.increment("column-deleted"),
        ObjectEvents::TextBoundsChanged(_) => obj_count.increment("text-bounds-changed"),
        ObjectEvents::TextSelectionChanged(_) => obj_count.increment("text-selection-changed"),
        ObjectEvents::TextChanged(_) => obj_count.increment("text-changed"),
        ObjectEvents::TextAttributesChanged(_) => obj_count.increment("text-attributes-changed"),
        ObjectEvents::TextCaretMoved(_) => obj_count.increment("text-caret-moved"),
    };
}

pub fn match_windowevents(wev: WindowEvents, win_count: &Arc<WindowCount>) {
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

pub fn match_documentevents(dev: DocumentEvents, doc_count: &Arc<DocumentCount>) {
    match dev {
        DocumentEvents::LoadComplete(_) => doc_count.increment("load-complete"),
        DocumentEvents::Reload(_) => doc_count.increment("reload"),
        DocumentEvents::LoadStopped(_) => doc_count.increment("load-stopped"),
        DocumentEvents::ContentChanged(_) => doc_count.increment("content-changed"),
        DocumentEvents::AttributesChanged(_) => doc_count.increment("attributes-changed"),
        DocumentEvents::PageChanged(_) => doc_count.increment("page-changed"),
    };
}

pub fn match_terminal_events(tev: TerminalEvents, term_count: &Arc<TerminalCount>) {
    match tev {
        TerminalEvents::LineChanged(_) => term_count.increment("line-changed"),
        TerminalEvents::ColumnCountChanged(_) => term_count.increment("column-count-changed"),
        TerminalEvents::LineCountChanged(_) => term_count.increment("line-count-changed"),
        TerminalEvents::ApplicationChanged(_) => term_count.increment("application-changed"),
        TerminalEvents::CharWidthChanged(_) => term_count.increment("char-width-changed"),
    };
}

pub fn match_mouse_events(mev: MouseEvents, mouse_count: &Arc<MouseCount>) {
    match mev {
        MouseEvents::Abs(_) => mouse_count.increment("abs"),
        MouseEvents::Rel(_) => mouse_count.increment("rel"),
        MouseEvents::Button(_) => mouse_count.increment("button"),
    };
}

pub fn match_cache_events(cev: CacheEvents, cache_count: &Arc<CacheCount>) {
    match cev {
        CacheEvents::Add(_) => cache_count.increment("add"),
        CacheEvents::LegacyAdd(_) => cache_count.increment("legacy-add"),
        CacheEvents::Remove(_) => cache_count.increment("remove"),
    };
}
