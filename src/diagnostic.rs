//! Positions and errors.
//!
//! Every stage reports through one [`Diagnostic`] type so that rendering lives in
//! exactly one place. A failure without a [`Span`] is a bug report the reader
//! cannot act on, so the span is not optional.

use std::fmt;
use std::ops::Range;

/// A half-open byte range into the source that produced it.
///
/// Byte offsets rather than line and column because that is what slicing wants;
/// turning one into a human position is the renderer's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end, "a span cannot end before it starts");
        Self { start, end }
    }

    /// An empty span, for pointing at a position rather than at text.
    pub fn at(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// The smallest span covering both.
    pub fn to(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// The text this span covers, or `None` if it does not land on character
    /// boundaries of `src`. Returning an `Option` keeps a wrong span from
    /// becoming a panic in the renderer.
    pub fn text<'a>(&self, src: &'a str) -> Option<&'a str> {
        src.get(self.range())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Something wrong with the user's program, with somewhere to point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
    /// One concrete thing to try. Absent when there is nothing honest to suggest;
    /// a guessed hint is worse than none.
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, " ({hint})")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_span_slices_the_text_it_covers() {
        let src = "oath sort";
        assert_eq!(Span::new(5, 9).text(src), Some("sort"));
        assert_eq!(Span::new(0, 4).text(src), Some("oath"));
    }

    #[test]
    fn a_span_off_a_character_boundary_is_none_rather_than_a_panic() {
        let src = "π";
        assert_eq!(Span::new(0, 1).text(src), None);
        assert_eq!(Span::new(0, 2).text(src), Some("π"));
    }

    #[test]
    fn spans_join_to_cover_both() {
        assert_eq!(Span::new(2, 4).to(Span::new(8, 9)), Span::new(2, 9));
        assert_eq!(Span::new(8, 9).to(Span::new(2, 4)), Span::new(2, 9));
    }

    #[test]
    fn an_empty_span_points_at_a_position() {
        let s = Span::at(4);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.text("abcdef"), Some(""));
    }

    #[test]
    fn a_hint_shows_in_the_display_form() {
        let d = Diagnostic::new(Span::at(0), "unexpected character").with_hint("try removing it");
        assert_eq!(d.to_string(), "unexpected character (try removing it)");
        assert_eq!(Diagnostic::new(Span::at(0), "bare").to_string(), "bare");
    }
}
