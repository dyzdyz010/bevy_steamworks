//! Small helpers for bounded state caches.

/// Drops the oldest entries until `items.len() <= limit`.
pub(crate) fn trim_oldest<T>(items: &mut Vec<T>, limit: usize) {
    if items.len() > limit {
        let overflow = items.len() - limit;
        items.drain(0..overflow);
    }
}
