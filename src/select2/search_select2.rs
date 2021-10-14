//! `Select2` interfaces to the `simple::SearchIndex`.

use crate::select2::Request;
use crate::simple::{SearchIndex, SearchType};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord> SearchIndex<K> {

    /// Once the client's `Select2` query-string has been parsed into a
    /// `Request` struct, the struct may be passed to this search method. This
    /// method will return all search results for the client's query.

    #[tracing::instrument(level = "trace", name = "Select2 Search", skip(self))]
    pub fn search_select2(
        &'a self,
        request: &'a Request,
    ) -> Vec<&'a K> {

        // Get query (or "search term"), if any:
        let query_term: Option<&String> = request.query_term(&self.dump_keyword);

        println!("Query: {:#?}", &query_term);

        if let Some(query_term) = query_term {

            // If valid query provided, perform search of index:
            self.search_with(
                &SearchType::Live,
                &self.max_keys_per_keyword(),
                query_term,
            ) // search_with

        } else {

            // If no query (or "search term"), then return empty results:
            Vec::new()

        } // if

    } // fn

} // impl