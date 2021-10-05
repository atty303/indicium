use crate::simple::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //

    ///
    /// Basic usage:
    ///
    /// ```ignore
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
    /// #
    /// # struct MyStruct {
    /// #   title: String,
    /// #   year: u16,
    /// #   body: String,
    /// # }
    /// #
    /// # impl Indexable for MyStruct {
    /// #   fn strings(&self) -> Vec<String> {
    /// #       vec![
    /// #           self.title.clone(),
    /// #           self.year.to_string(),
    /// #           self.body.clone(),
    /// #       ]
    /// #   }
    /// # }
    /// #
    /// # let my_vec = vec![
    /// #   MyStruct {
    /// #       title: "Harold Godwinson".to_string(),
    /// #       year: 1066,
    /// #       body: "Last crowned Anglo-Saxon king of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Edgar Ætheling".to_string(),
    /// #       year: 1066,
    /// #       body: "Last male member of the royal house of Cerdic of Wessex.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William the Conqueror".to_string(),
    /// #       year: 1066,
    /// #       body: "First Norman monarch of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William Rufus".to_string(),
    /// #       year: 1087,
    /// #       body: "Third son of William the Conqueror.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Henry Beauclerc".to_string(),
    /// #       year: 1100,
    /// #       body: "Fourth son of William the Conqueror.".to_string(),
    /// #   },
    /// # ];
    /// #
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// #
    /// # my_vec
    /// #   .iter()
    /// #   .enumerate()
    /// #   .for_each(|(index, element)|
    /// #       search_index.insert(&index, element)
    /// #   );
    /// #
    /// let search_results = search_index
    ///     .search_live("Norman C")
    ///     .iter()
    ///     .cloned()
    ///     .collect::<Vec<&usize>>();
    ///
    /// assert_eq!(search_results, vec![&2]);
    /// ```

    #[tracing::instrument(level = "trace", name = "Live Search", skip(self))]
    pub(crate) fn search_live(&self, string: &str) -> BTreeSet<&K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Perform `And` search for entire string without the last keyword.
            // We also must convert the return `HashSet` into a `BTreeSet`. It
            // should be investigated if `internal_search_and` should returning
            // a `BTreeSet` instead.
            let search_results: BTreeSet<&K> =
                self.internal_search_and(keywords.as_slice())
                    // Iterate over each key:
                    .iter()
                    // Copy each `&K` key reference from the iterator or we'll
                    // get a doubly-referenced `&&K` key:
                    .cloned()
                    // Collect serach results into our `BTreeSet`:
                    .collect();

            // Get all autocomplete options for the last keyword and its keys:
            let autocomplete_options: BTreeSet<&BTreeSet<K>> =
                self.internal_autocomplete_keyword(&last_keyword)
                    // Iterate over each search result:
                    .iter()
                    // We're not interested in the `keyword` since we're
                    // returning `&K` keys. Return only `&K` from the tuple:
                    .map(|(_keyword, keys)| *keys)
                    // Collect search results from each autocomplete option:
                    .collect();

            // How we combine `search_results` and `autocomplete_options`
            // together depends on how many keywords there are in the search
            // string. Strings that have only a single keyword, and a strings
            // that have multiple keywords must be handled differently:

            match keywords.len() {

                // Consider this example search string: `t`.
                //
                // Depending on the data-set, autocomplete options `trouble` and
                // `tribble` may be given.
                //
                // There are no previous keywords to intersect with, just the
                // letter `t`. We will return the keys for these autocomplete
                // options without further processing:

                0 => autocomplete_options
                    // Iterate over each autocomplete option:
                    .iter()
                    // Copy the `&BTreeSet` reference from the iterator or we'll
                    // be working with a doubly-referenced `&&BTreeSet`:
                    .cloned()
                    // Flatten the `key` results from each autocomplete option
                    // into our collection:
                    .flatten()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    // And collect each key into a `BTreeSet` that will be the
                    // search results.
                    .collect(),

                // Consider this example search string: `Shatner t`.
                //
                // Depending on the data-set, autocomplete options for `t` might
                // be `trouble` and `tribble`. However, in this example there is
                // a previous keyword: `Shatner`.
                //
                // This match arm will intersect the results from each
                // autocomplete option with `Shatner`. For both `trouble` and
                // `tribble` autocomplete options, only keys that also exist for
                // `Shatner` will be returned. All resulting keys for both
                // autocomplete options will be flattened together:

                _ => autocomplete_options
                    // Iterate over each autocomplete option:
                    .iter()
                    // For each autocomplete option, we will intersect its
                    // search results with the search results of the preceding
                    // keywords:
                    .map(|autocompletion_keys| autocompletion_keys
                        // Iterate over each key returned for this autocomplete
                        // option:
                        .iter()
                        // Only keep the `&K` key for this autocomplete option
                        // if it is contained in the search results for the
                        // preceding keywords:
                        .filter(|autocompletion_key|
                            search_results.contains(autocompletion_key)
                        ) // filter
                        // Collect all resulting keys into a `Vec`:
                        .collect::<Vec<&K>>()
                    ) // map
                    // Flatten the `key` results for each autocomplete option
                    // into our collection:
                    .flatten()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    // And collect each key into a `BTreeSet` that will be the
                    // search results.
                    .collect(),

            } // match

        } else {

            // The search string did not have a last keyword to autocomplete (or
            // any keywords to search for.) Return an empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl