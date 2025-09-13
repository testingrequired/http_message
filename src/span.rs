use std::ops::Range;

pub type Span = Range<usize>;

/// Get all line spans in the given string
pub fn get_line_spans(input: &str) -> Vec<Range<usize>> {
    let mut spans = Vec::new();
    let mut start = 0;

    for (idx, ch) in input.char_indices() {
        if ch == '\n' {
            spans.push(start..idx + ch.len_utf8());
            start = idx + ch.len_utf8();
        }
    }

    if start < input.len() {
        spans.push(start..input.len());
    }

    spans
}

#[cfg(test)]
mod get_line_spans_tests {
    use super::*;

    #[test]
    fn test_line_spans() {
        let text = "hello\nworld\nlast";
        let spans = get_line_spans(text);
        assert_eq!(spans, vec![0..6, 6..12, 12..16]);
        assert_eq!(&text[spans[0].clone()], "hello\n");
        assert_eq!(&text[spans[1].clone()], "world\n");
        assert_eq!(&text[spans[2].clone()], "last");
    }

    #[test]
    fn test_with_trailing_newline() {
        let text = "one\ntwo\n";
        let spans = get_line_spans(text);
        assert_eq!(spans, vec![0..4, 4..8]);
        assert_eq!(&text[spans[0].clone()], "one\n");
        assert_eq!(&text[spans[1].clone()], "two\n");
    }
}
