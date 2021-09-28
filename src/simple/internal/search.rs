use crate::simple::internal::MAXIMUM_INTERNAL_SEARCH_RESULTS;
use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method only accepts a single keyword as the
    /// search string._ Search keywords must be an exact match.
    ///
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest search type. It is good for compact interfaces,
    /// where records are very simple, or data-sets are quite small.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Note: This function is lower-level and for internal use only. It does
    /// not observe any settings such as _case-sensitivity_ or _maximum
    /// results_. These constraints should be observed at higher levels.

    pub(crate) fn internal_keyword_search(&self, keyword: &str) -> BTreeSet<&K> {

        // Attempt to get matching keys for the search keyword from BTreeMap:
        if let Some(keys) = self.b_tree_map.get(keyword) {

            // Attempt to get matching keys for search keyword:
            keys
                // Iterate over all matching keys and only return
                // `maximum_search_results` number of keys:
                .iter()
                // Only return `maximum_search_results` number of keys:
                .take(MAXIMUM_INTERNAL_SEARCH_RESULTS)
                // Insert each resulting key into the hash set:
                .collect()

            // -> If fuzzy matching were to be implemented for
            // `indicium::simple` it would probably be put here. <-

        } else {

            // The search keyword did not result in any matches. Return an
            // empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl