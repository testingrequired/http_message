use std::ops::Range;

pub type Span = Range<usize>;

pub type LineSpans = Vec<Option<Span>>;
