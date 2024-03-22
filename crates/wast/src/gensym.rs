use crate::token::{Id, Span};
use std::cell::Cell;
use std::sync::atomic::{AtomicU32, Ordering};

// thread_local!(static NEXT: Cell<u32> = Cell::new(0));

static SYM: AtomicU32 = AtomicU32::new(0);

pub fn reset() {
    SYM.store(0, Ordering::Relaxed);
}

pub fn gen(span: Span) -> Id<'static> {
    Id::gensym(span, SYM.fetch_add(1, Ordering::Relaxed))
}

pub fn fill<'a>(span: Span, slot: &mut Option<Id<'a>>) -> Id<'a> {
    *slot.get_or_insert_with(|| gen(span))
}
