use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// This function will use the `AutocompleteType` setting stored in the
    /// `SearchIndex`. Partial keywords must be an exact match.
    ///
    /// The search string may contain multiple keywords and the last (partial)
    /// keyword will be autocompleted. The last keyword in the search string
    /// will be autocompleted from all available keywords in the search index.
    /// If your data-set is very large or has repetitive keywords, this is the
    /// recommended autocomplete type. Results are returned in lexographic
    /// order.

    pub fn autocomplete_global(&self, string: &str) -> Vec<String> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list. It's the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Autocomplete the last keyword:
            let autocompletions = self.autocomplete_keyword(&last_keyword);

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