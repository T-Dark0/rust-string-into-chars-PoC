#![feature(assoc_char_funcs)]

mod libstd_stolen; //Shhh, I totally didn't shamelessly steal this code from libcore internals. Not at all. I have no idea what you're talking about

use libstd_stolen::{next_code_point, next_code_point_reverse, utf8_is_cont_byte};
use std::iter::{DoubleEndedIterator, FusedIterator};

struct StringIntoChars {
    iter: std::vec::IntoIter<u8>,
}

impl Iterator for StringIntoChars {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        next_code_point(&mut self.iter).map(|ch| {
            // SAFETY: `str` invariant says `ch` is a valid Unicode Scalar Value.
            unsafe { char::from_u32_unchecked(ch) }
        })
    }

    #[inline]
    fn count(self) -> usize {
        // length in `char` is equal to the number of non-continuation bytes
        self.iter.filter(|&byte| !utf8_is_cont_byte(byte)).count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        // `(len + 3)` can't overflow, because we know that the `slice::Iter`
        // belongs to a slice in memory which has a maximum length of
        // `isize::MAX` (that's well below `usize::MAX`).
        ((len + 3) / 4, Some(len))
    }

    #[inline]
    fn last(mut self) -> Option<char> {
        // No need to go through the entire string.
        self.next_back()
    }
}

impl DoubleEndedIterator for StringIntoChars {
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        next_code_point_reverse(&mut self.iter).map(|ch| {
            // SAFETY: `str` invariant says `ch` is a valid Unicode Scalar Value.
            unsafe { char::from_u32_unchecked(ch) }
        })
    }
}

impl FusedIterator for StringIntoChars {}

fn into_chars(string: String) -> StringIntoChars {
    StringIntoChars {
        iter: string.into_bytes().into_iter(),
    }
}
