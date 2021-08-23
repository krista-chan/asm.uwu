use std::{fmt::Debug, ops::Range};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span {
            start: range.start,
            end: range.end
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T>
where
    T: Clone + Debug,
{
    pub data: T,
    pub span: Span,
}
