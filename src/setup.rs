use atspi::events::{
    AvailableEvent, CacheEvents, DocumentEvents, EventListenerEvents, FocusEvents, KeyboardEvents,
    MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};

pub async fn setup() -> Result<atspi::AccessibilityConnection, atspi::AtspiError> {
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

    Ok(atspi)
}
