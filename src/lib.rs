/// Each slice in this struct's list is a word in some
/// in-memory text document.
#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

/// Make a new empty target words list.
impl<'a> KWIndex<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        KWIndex(Vec::new())
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this `KWIndex`
    /// index.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up an index.
    ///
    /// Words are separated by whitespace. Words may
    /// Words may have leading or trailing punctuation,
    /// which is removed during indexing. Words
    /// consist of a span of one or more consecutive
    /// letters (any UTF-8 character in the "letter" class)
    /// with no internal punctuation; words with internal
    /// punctuation are ignored during indexing.  The index
    /// is case sensitive.
    ///
    /// For example, the text
    ///
    /// ```text
    /// "It ain't over untïl it ain't, over."
    /// ```
    ///
    /// contains the sequence of words `"It"`, `"over"`,
    /// `"untïl"`, `"it"`, `"over"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let index = kwindex::KWIndex::new()
    ///     .extend_from_text("Hello world.");
    /// assert_eq!(2, index.len());
    /// assert_eq!(1, index.count_matches("world"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        for word in target.split_whitespace() {
            let mut push = true;
            let wordcheck = word.trim_matches(|c: char| !c.is_alphabetic());
            for chars in wordcheck.chars() {
                if !chars.is_alphabetic() {
                    push = false;
                }
            }
            if push {
                self.0.push(wordcheck);
            }
        }
        self
    }

    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut count = 0;
        for word in &self.0 {
            if &keyword == word {
                count += 1;
            }
        }
        count
    }
    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    pub fn len(&self) -> usize {
        let size = &self.0.len();
        *size
    }
    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        let mut ret = false;
        if self.0.is_empty() {
            ret = true
        }
        ret
    }
}
