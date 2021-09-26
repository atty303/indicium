use crate::simple::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.
    ///
    /// For `And` autocompletion, the autocompletions are contextual. A search
    /// of `this that` will only return autocompletions that are related to
    /// records containing keywords both `this` and `that`. This conjuction uses
    /// more CPU resources than `Or` because the results must be filtered
    /// according to the previous keywords in the string.

    pub fn and_autocomplete(&self, string: &str) -> Vec<String> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list. It's the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Perform search for entire string without the last keyword:
            let search_results: BTreeSet<&K> =
                self.internal_and_search(keywords.as_slice());

            // Get all autocompletions for the last keyword.
            let autocompletions: BTreeSet<(&String, &BTreeSet<K>)> =
                self.internal_autocomplete_keyword(&last_keyword);

            // Intersect the autocompletions for the last keyword with the
            // search results. This way, only relevant autocompletions are
            // returned:

            let autocompletions: Vec<&String> = if search_results.is_empty() {

                autocompletions
                    .iter()
                    .take(self.maximum_autocomplete_results)
                    // `internal_autocomplete_keyword` returns a key-value pair.
                    // We're autocompleting the key, so discard the value:
                    .map(|(keyword, _keys)| keyword)
                    // Copy each keyword from the iterator or we'll get a
                    // doubly-referenced `&&String` keyword:
                    .cloned()
                    // Collect all keyword autocompletions into a `Vec`:
                    .collect()

            } else {

                autocompletions
                    .iter()
                    // Only keep this autocompletion if it contains a key that the
                    // search results contain:
                    .filter(|(_keyword, keys)|
                       keys.iter().any(|key| search_results.contains(key))
                    ) // filter
                    // Only return `maximum_autocomplete_results` number of keywords:
                    .take(self.maximum_autocomplete_results)
                    // `internal_autocomplete_keyword` returns a key-value pair.
                    // We're autocompleting the key, so discard the value:
                    .map(|(keyword, _keys)| keyword)
                    // Copy each keyword from the iterator or we'll get a
                    // doubly-referenced `&&String` keyword:
                    .cloned()
                    // Collect all keyword autocompletions into a `Vec`:
                    .collect()

            }; // if

            // Push a blank placeholder onto the end of the keyword list. We
            // will be putting our autocompletions for the last keyword into
            // this spot:
            keywords.push(String::from(""));

            // Build autocompleted search strings from the autocompletions
            // derived from the last keyword:
            autocompletions
                // Iterate over each autocompleted last keyword:
                .iter()
                // Use the prepended `keywords` and autocompleted last keyword
                // to build an autocompleted search string:
                .map(|last_keyword| {
                    // Remove previous autocompleted last keyword from list:
                    keywords.pop();
                    // Add current autocompleted last keyword to end of list:
                    keywords.push(String::from(*last_keyword));
                    // Join all keywords together into a single `String` using a
                    // space delimiter:
                    keywords.join(" ")
                })
                // Collect all string autocompletions into a `Vec`:
                .collect()

        } else {

            // The search string did not have a last keyword to autocomplete.
            // Return an empty `Vec`:
            Vec::new()

        } // if

    } // fn

} // impl