use crate::simple::internal::TopScores;
use kstring::KString;
use std::{clone::Clone, collections::BTreeSet, cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: Clone + PartialOrd> TopScores<'a, K, S> {

    // -----------------------------------------------------------------------------
    //
    /// Attempts to insert the provided _keyword_, _keys_, & _score_ into the
    /// top scores.
    ///
    /// If the caller provided score is higher than the current lowest top
    /// score, the caller's score will be inserted into the collection. If it
    /// provided score doesn't beat the lowest top score, it will be ignored.

    pub(crate) fn insert(
        &mut self,
        keyword: &'a KString,
        keys: &'a BTreeSet<K>,
        score: S,
    ) {

        // Check if the `TopScores` struct has reached its maximum capacity:
        if self.top.len() >= self.capacity {

            // If the `TopScores` is at capacity and the lowest top score (the
            // bottom) is currently unknown, find it:
            if self.bottom.is_none() { self.find_bottom() }

            // The lowest top score should be known at this point:
            if let Some(bottom) = &self.bottom {
                // If the caller's provided score is higher than the lowest
                // top score, we have a new score:
                if score > bottom.1 {
                    // Remove the old lowest top score (or bottom) from the
                    // collection:
                    self.remove_bottom();
                    // Insert the new score into the collection:
                    self.top.insert(keyword, (keys, score));
                } // if
            } // if

        } else {

            // The `TopScores` struct has not reached its capacity, we may
            // blindly add the _keyword_, _keys_, & _score_ without checking the
            // lowest score:
            self.top.insert(keyword, (keys, score));

        } // if

    } // fn insert

} // impl TopScores