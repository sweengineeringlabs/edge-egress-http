//! SSE event value object (egress).

/// A single Server-Sent Event frame received from a remote SSE feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    /// Optional `event:` field — names the event type.
    pub event: Option<String>,
    /// The `data:` field — the payload carried by the event.
    pub data: String,
    /// Optional `id:` field — the last-event-ID for reconnect resumption.
    pub id: Option<String>,
}

impl SseEvent {
    /// Construct a data-only event with no type or ID.
    pub fn data(data: impl Into<String>) -> Self {
        Self {
            event: None,
            data: data.into(),
            id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: data
    #[test]
    fn test_sse_event_data_sets_payload_and_leaves_optional_fields_none() {
        let ev = SseEvent::data("hello");
        assert_eq!(ev.data, "hello");
        assert!(ev.event.is_none());
        assert!(ev.id.is_none());
    }

    #[test]
    fn test_sse_event_full_fields_are_preserved() {
        let ev = SseEvent {
            event: Some("update".into()),
            data: "{}".into(),
            id: Some("42".into()),
        };
        assert_eq!(ev.event.as_deref(), Some("update"));
        assert_eq!(ev.id.as_deref(), Some("42"));
    }
}
