/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   22 Dec 2021, 17:20:49
 * Last edited:
 *   28 Dec 2021, 14:48:05
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The OpString library provides the OpString (Operational String) class,
 *   which can be generated from a normal string and works solely one
 *   graphene units; basically as string as you'd expect.
**/

use std::ops;
use std::fmt;
use unicode_segmentation::{UnicodeSegmentation};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        // Try to create the OpString around the empty string
        let ops = OpString::new("");
        // Make sure there is nothing in there
        assert_eq!(ops.len(), 0);
    }

    #[test]
    fn ascii_string() {
        // Try to create the OpString around a simple string of only ascii characters
        let ops = OpString::new("Hello, world!");

        // Check the length
        assert_eq!(ops.len(), 13);

        // Check the characters
        assert_eq!(ops[ 0], "H");
        assert_eq!(ops[ 1], "e");
        assert_eq!(ops[ 2], "l");
        assert_eq!(ops[ 3], "l");
        assert_eq!(ops[ 4], "o");
        assert_eq!(ops[ 5], ",");
        assert_eq!(ops[ 6], " ");
        assert_eq!(ops[ 7], "w");
        assert_eq!(ops[ 8], "o");
        assert_eq!(ops[ 9], "r");
        assert_eq!(ops[10], "l");
        assert_eq!(ops[11], "d");
        assert_eq!(ops[12], "!");
    }

    #[test]
    fn ascii_string_iter() {
        // Try to create the OpString around a simple string of only ascii characters
        let par = "Hello, world!";
        let ops = OpString::new(par);

        // Iterate over the pairs
        let mut j: usize = 0;
        for (i, c) in ops.iter() {
            assert_eq!(*i, j);
            assert_eq!(*c, String::from(par.chars().nth(*i).unwrap()));
            j += 1;
        }

        // Iterate over the characters
        let mut citer = ops.chars();
        assert_eq!(citer.next(), Some("H"));
        assert_eq!(citer.next(), Some("e"));
        assert_eq!(citer.next(), Some("l"));
        assert_eq!(citer.next(), Some("l"));
        assert_eq!(citer.next(), Some("o"));
        assert_eq!(citer.next(), Some(","));
        assert_eq!(citer.next(), Some(" "));
        assert_eq!(citer.next(), Some("w"));
        assert_eq!(citer.next(), Some("o"));
        assert_eq!(citer.next(), Some("r"));
        assert_eq!(citer.next(), Some("l"));
        assert_eq!(citer.next(), Some("d"));
        assert_eq!(citer.next(), Some("!"));
    }

    #[test]
    fn utf8_string() {
        // Try to create the OpString around a more complicated string of special characters
        let ops = OpString::new(":⁾");

        // Check the length
        assert_eq!(ops.len(), 2);

        // Check the characters
        assert_eq!(ops[ 0], ":");
        assert_eq!(ops[ 1], "⁾");
    }

    #[test]
    fn utf8_string_iter() {
        // Try to create the OpString around a simple string of only ascii characters
        let par = ":⁾";
        println!("par len: {}", par.len());
        let ops = OpString::new(par);

        // Iterate over the pairs
        let mut j: usize = 0;
        for (i, c) in ops.iter() {
            assert_eq!(*i, j);
            for k in 0..c.len() {
                assert_eq!((*c).bytes().nth(k).unwrap(), par.bytes().nth(*i + k).unwrap());
            }
            j += (*c).len();
        }

        // Iterate over the characters
        let mut citer = ops.chars();
        assert_eq!(citer.next(), Some(":"));
        assert_eq!(citer.next(), Some("⁾"));
    }
}





/***** OPSTRING STRUCT *****/
/// The OpString class can 'wrap' around a normal string to work with graphene units instead of the normal, character method.
/// 
/// Basically tries to mimic a string the way you'd expect.
#[derive(Debug)]
pub struct OpString<'a> {
    /// Reference to the parent string.
    parent : &'a str,
    /// The list of graphene items in this string. Each pair contains a 'char', with its position in the original string.
    data  : Vec<(usize, &'a str)>,
}

impl<'a> OpString<'a> {
    /// Constructor for the OpString class.
    /// 
    /// **Arguments**
    ///  * `s`: The normal string to wrap around. Note that only a reference is kept, so the lifetime of OpString is the same as the string.
    ///
    /// **Returns**
    /// A fully instantiated OpString instance.
    pub fn new(s: &'a str) -> OpString {
        // Create the list first
        let mut list: Vec<(usize, &'a str)> = Vec::new();
        list.reserve(s.len());
        let mut i: usize = 0;
        for g in UnicodeSegmentation::graphemes(s, true) {
            list.push((i, g));
            i += g.len();
        }

        // Return the opstring with this list of graphenes
        OpString {
            parent : s,
            data   : list
        }
    }



    /// Returns the parent string.
    /// 
    /// **Returns**  
    /// A reference to the parent string.
    pub fn parent(&self) -> &'a str {
        return self.parent;
    }



    /// Returns the first character in the OpString.
    /// 
    /// **Returns**  
    /// Returns the first character as a string slice, or an empty string if there is none.
    pub fn first(&self) -> &'a str {
        let elem = self.data.first();
        if elem == None { return ""; }
        return elem.unwrap().1;
    }

    /// Returns the last character in the OpString.
    /// 
    /// **Returns**  
    /// Returns the last character as a string slice, or an empty string if there is none.
    pub fn last(&self) -> &'a str {
        let elem = self.data.last();
        if elem == None { return ""; }
        return elem.unwrap().1;
    }



    /// Returns an iterator over the internal position-character pairs.
    /// 
    /// **Returns**  
    /// An OpStringPairIter() struct that iterates over the pairs.
    pub fn iter(&'a self) -> OpStringPairIter<'a> {
        return OpStringPairIter::new(&self.data);
    }

    /// Returns an iterator over the characters in the OpString.
    /// 
    /// **Returns**  
    /// An OpStringCharIter() struct that iterates over the characters.
    pub fn chars(&'a self) -> OpStringCharIter<'a> {
        return OpStringCharIter::new(&self.data);
    }



    /// Translates the given OpString index to the index of its parent string.
    /// 
    /// If the given index is out-of-bounds, returns the first position after the parent string instead.
    /// 
    /// **Arguments**
    ///  * `opstring_index`: The index to translate.
    /// 
    /// **Returns**  
    /// The index that can be used to slice or w/e in the original string.
    pub fn translate_opstr(&self, opstring_index: usize) -> usize {
        // Make sure the index fits
        if opstring_index >= self.data.len() {
            return self.parent.len();
        }

        // Return the correct pos
        return self.data[opstring_index].0;
    }

    /// Translates the given parent index to the index of the char in which it falls.
    /// 
    /// If the given index is out-of-bounds, returns the first position after the opstring instead.
    /// 
    /// **Arguments**
    ///  * `str_index`: The index to translate.
    /// 
    /// **Returns**  
    /// The index that can be used to access its character in the OpString.
    pub fn translate_str(&self, str_index: usize) -> usize {
        // Loop to find it
        for (i, g) in self.data.iter() {
            if str_index >= *i && str_index < i + g.len() { return str_index; }
        }

        // Nothing found; must be out-of-bounds
        return self.data.len();
    }

    /// The len() operator for the OpString.
    /// 
    /// **Returns**
    /// The size of the internal elements. Never given an index that exceeds this size, or the index function will panic.
    #[inline]
    pub fn len(&self) -> usize {
        return self.data.len();
    }
}

impl<'a> fmt::Display for OpString<'a> {
    /// Formats the string nicely in a normal format operation.
    /// 
    /// **Arguments**
    ///  * `f`: The format configuration to use for writing.
    /// 
    /// **Returns**  
    /// Whether the writing was successful or not, as a fmt::Result.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parent)
    }
}

impl<'a> ops::Index<usize> for OpString<'a> {
    /// The output type of the immutable index operation.
    type Output = &'a str;

    /// The immuteable index operation for he OpString class.
    /// 
    /// **Arguments**
    ///  * `index`: The index to return.
    /// 
    /// **Returns**
    /// The requested value. Will panic! if out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        // Make sure we're not out-of-bounds
        if index >= self.data.len() {
            panic!("Index {} is out of bounds for OpString of size {}.", index, self.data.len());
        }

        // Return the item
        return &self.data[index].1;
    }
}





/***** OPSTRINGPAIRITER STRUCT *****/
/// The OpStringPairIter class iterates over the position/char pairs in an OpString.
pub struct OpStringPairIter<'a> {
    /// Reference to the vector we iterate over.
    data : &'a Vec<(usize, &'a str)>,
    /// Position in the vector we iterate over.
    i    : usize,
}

impl<'a> OpStringPairIter<'a> {
    /// Private constructor for the OpStringPairIter class.
    /// 
    /// **Arguments**
    ///  * `data`: The data vector to iterate over.
    fn new(data: &'a Vec<(usize, &'a str)>) -> OpStringPairIter<'a> {
        return OpStringPairIter{
            data : data,
            i    : 0
        };
    }
}

impl<'a> Iterator for OpStringPairIter<'a> {
    /// The type of each item returned by the iterator.
    type Item = &'a (usize, &'a str);

    /// Returns the new item in the iterator.
    /// 
    /// **Returns**  
    /// The next item, or None if we reached the end.
    fn next(&mut self) -> std::option::Option<Self::Item> {
        // Check if not overflowing
        if self.i >= self.data.len() {
            return None;
        }

        // Otherwise, return the item after incrementing i
        let i = self.i;
        self.i += 1;
        return Some(&self.data[i]);
    }
}





/***** OPSTRINGCHARITER STRUCT *****/
/// The OpStringCharIter class iterates over the 'chars' (more like graphenes) in an OpString.
pub struct OpStringCharIter<'a> {
    /// Reference to the vector we iterate over.
    data : &'a Vec<(usize, &'a str)>,
    /// Position in the vector we iterate over.
    i    : usize,
}

impl<'a> OpStringCharIter<'a> {
    /// Private constructor for the OpStringCharIter class.
    /// 
    /// **Arguments**
    ///  * `data`: The data vector to iterate over.
    fn new(data: &'a Vec<(usize, &'a str)>) -> OpStringCharIter<'a> {
        return OpStringCharIter{
            data : data,
            i    : 0
        };
    }
}

impl<'a> Iterator for OpStringCharIter<'a> {
    /// The type of each item returned by the iterator.
    type Item = &'a str;

    /// Returns the new item in the iterator.
    /// 
    /// **Returns**  
    /// The next item, or None if we reached the end.
    fn next(&mut self) -> std::option::Option<Self::Item> {
        // Check if not overflowing
        if self.i >= self.data.len() {
            return None;
        }

        // Otherwise, return the item after incrementing i
        let i = self.i;
        self.i += 1;
        return Some(&self.data[i].1);
    }
}
