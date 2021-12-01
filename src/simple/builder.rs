use crate::simple::{AutocompleteType, SearchIndex, SearchType};
use std::clone::Clone;
use std::cmp::Ord;
use std::collections::{BTreeMap, BTreeSet};

// -----------------------------------------------------------------------------
//
/// The [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)
/// can be used to customize your search index. Use
/// `SearchIndexBuilder::default()` to start the builder chain, and `.build()`
/// to finish it.
///
/// If you're in a hurry, you can instantiate your search index with
/// `SearchIndex::default()` instead.

pub struct SearchIndexBuilder<K> {
    b_tree_map: BTreeMap<String, BTreeSet<K>>,
    search_type: SearchType,
    autocomplete_type: AutocompleteType,
    split_pattern: Option<Vec<char>>,
    case_sensitive: bool,
    minimum_keyword_length: usize,
    maximum_keyword_length: usize,
    maximum_string_length: Option<usize>,
    exclude_keywords: Option<Vec<String>>,
    maximum_autocomplete_options: usize,
    maximum_search_results: usize,
    maximum_keys_per_keyword: usize,
    dump_keyword: Option<String>,
} // SearchIndexBuilder

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> From<SearchIndex<K>> for SearchIndexBuilder<K> {
    /// Convert to `SearchIndexBuilder<K>` struct from `SearchIndex<K>` struct.
    fn from(search_index: SearchIndex<K>) -> Self {
        SearchIndexBuilder {
            b_tree_map: search_index.b_tree_map,
            search_type: search_index.search_type,
            autocomplete_type: search_index.autocomplete_type,
            split_pattern: search_index.split_pattern,
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            exclude_keywords: search_index.exclude_keywords,
            maximum_autocomplete_options: search_index.maximum_autocomplete_options,
            maximum_search_results: search_index.maximum_search_results,
            maximum_keys_per_keyword: search_index.maximum_keys_per_keyword,
            dump_keyword: search_index.dump_keyword,
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> From<&SearchIndexBuilder<K>> for SearchIndex<K> {
    /// Convert to `SearchIndex<K>` struct from `SearchIndexBuilder<K>` struct.
    fn from(search_index: &SearchIndexBuilder<K>) -> Self {
        SearchIndex {
            b_tree_map: search_index.b_tree_map.clone(),
            search_type: search_index.search_type.to_owned(),
            autocomplete_type: search_index.autocomplete_type.to_owned(),
            split_pattern: search_index.split_pattern.to_owned(),
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            exclude_keywords: search_index.exclude_keywords.clone(),
            maximum_autocomplete_options: search_index.maximum_autocomplete_options,
            maximum_search_results: search_index.maximum_search_results,
            maximum_keys_per_keyword: search_index.maximum_keys_per_keyword,
            dump_keyword: search_index.dump_keyword.to_owned(),
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndexBuilder<K> {

    /// Initialize `SearchIndexBuilder` with default settings.
    pub fn default() -> Self {
        SearchIndexBuilder::from(SearchIndex::default())
    } // fn

    /// Search type (or logical conjuction). Used to determine how to connect
    /// search results for each keyword. See [`SearchType`] for more
    /// information.
    ///
    /// [`SearchType`]: enum.SearchType.html
    pub fn search_type(&mut self, search_type: &SearchType) -> &mut Self {
        self.search_type = search_type.to_owned();
        self
    } // fn

    /// Autocomplete type (or keyword scope). Used to determine if or how to
    /// filtering keyword results for autocompletion. See [`AutocompleteType`]
    /// for more information.
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    pub fn autocomplete_type(&mut self, autocomplete_type: &AutocompleteType) -> &mut Self {
        self.autocomplete_type = autocomplete_type.to_owned();
        self
    } // fn

    /// Characters used to split strings into keywords.
    pub fn split_pattern(&mut self, split_pattern: &Option<Vec<char>>) -> &mut Self {
        self.split_pattern = split_pattern.to_owned();
        self
    } // fn

    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be normalized to lower case.
    pub fn case_sensitive(&mut self, case_sensitive: &bool) -> &mut Self {
        self.case_sensitive = *case_sensitive;
        self
    } // fn

    /// Minimum keyword length (in chars or codepoints) to be indexed.
    pub fn min_keyword_len(&mut self, minimum_keyword_length: &usize) -> &mut Self {
        self.minimum_keyword_length = *minimum_keyword_length;
        self
    } // fn

    /// Maximum keyword length (in chars or codepoints) to be indexed.
    pub fn max_keyword_len(&mut self, maximum_keyword_length: &usize) -> &mut Self {
        self.maximum_keyword_length = *maximum_keyword_length;
        self
    } // fn

    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will index the record's _full field text_ & _whole strings_ as
    /// a single keyword for autocompletion purposes.
    pub fn max_string_len(&mut self, maximum_string_length: &Option<usize>) -> &mut Self {
        self.maximum_string_length = *maximum_string_length;
        self
    } // fn

    /// List of keywords that should not be indexed. It might be a good idea to
    /// exclude minor words - short conjunctions, articles, and short
    /// prepositions from your search index. For example, words such as `and`,
    /// `as`, `a`, `as`, `at`, etc. See also: the [`profile`] utility method.
    ///
    /// [`profile`]: struct.SearchIndex.html#method.profile
    pub fn exclude_keywords(&mut self, exclude_keywords: &Option<Vec<String>>) -> &mut Self {
        self.exclude_keywords = exclude_keywords.to_owned();
        self
    } // fn

    /// Maximum number of auto-complete options to return.
    pub fn max_autocomplete_options(&mut self, maximum_autocomplete_options: &usize) -> &mut Self {
        self.maximum_autocomplete_options = *maximum_autocomplete_options;
        self
    } // fn

    /// Maximum number of search results to return.
    pub fn max_search_results(&mut self, maximum_search_results: &usize) -> &mut Self {
        self.maximum_search_results = *maximum_search_results;
        self
    } // fn

    /// Maximum number of keys per keyword. If there are too many records
    /// attached to a single keyword, performance can begin to degrade. This
    /// setting limits the number of keys that may be attached to a keyword. See
    /// also: the `exclude_keywords` list and the `profile` method.
    pub fn max_keys_per_keyword(&mut self, maximum_keys_per_keyword: &usize) -> &mut Self {
        self.maximum_keys_per_keyword = *maximum_keys_per_keyword;
        self
    } // fn

    /// A special keyword that will return or "dump" all keys (or records) in
    /// the search index. This is helpful for the `Select2` module, where it
    /// should be returning all records if the search string is empty.
    pub fn dump_keyword(&mut self, dump_keyword: &Option<String>) -> &mut Self {
        self.dump_keyword = dump_keyword.to_owned();
        self
    } // fn

    /// Build `SearchIndex` from the settings given to the `SearchIndexBuilder`.
    pub fn build(&self) -> SearchIndex<K> {
        SearchIndex::from(self)
    } // fn

} // impl