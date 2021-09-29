#[test]
fn simple() {

    use crate::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};

    #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MyStruct {
        title: String,
        year: u16,
        body: String,
    }

    impl Indexable for MyStruct {
        fn strings(&self) -> Vec<String> {
            vec![
                self.title.clone(),
                self.year.to_string(),
                self.body.clone(),
            ]
        }
    }

    let my_vec = vec![
        MyStruct {
            title: "Harold Godwinson".to_string(),
            year: 1066,
            body: "Last crowned Anglo-Saxon king of England.".to_string(),
        },
        MyStruct {
            title: "Edgar Ætheling".to_string(),
            year: 1066,
            body: "Last male member of the royal house of Cerdic of Wessex.".to_string(),
        },
        MyStruct {
            title: "William the Conqueror".to_string(),
            year: 1066,
            body: "First Norman monarch of England.".to_string(),
        },
        MyStruct {
            title: "William Rufus".to_string(),
            year: 1087,
            body: "Third son of William the Conqueror.".to_string(),
        },
        MyStruct {
            title: "Henry Beauclerc".to_string(),
            year: 1100,
            body: "Fourth son of William the Conqueror.".to_string(),
        },
    ];

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)|
            search_index.insert(&index, element)
        );

    let search_results = search_index.search("third fourth");
    assert_eq!(search_results, vec![&3, &4]);

    // Search for `last` or `wessex`. `Edgar Ætheling` contains both keywords,
    // so he should be returned first. `Harold Godwinson` only contains `last`
    // so he should be returned last:
    let search_results = search_index.search("last Wessex");
    assert_eq!(search_results, vec![&1, &0]);

    let search_results = search_index.search_type(&SearchType::Keyword, "Wessex");
    assert_eq!(search_results, vec![&1]);

    let search_results = search_index.search_type(&SearchType::Or, "last England");
    assert_eq!(search_results, vec![&0, &1, &2]);

    let search_results = search_index.search_type(&SearchType::And, "Conqueror third");
    assert_eq!(search_results, vec![&3]);

    let autocomplete_options = search_index.autocomplete("Edgar last c");
    assert_eq!(autocomplete_options, vec!["edgar last cerdic".to_string()]);

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Keyword, "E");
    assert_eq!(autocomplete_options, vec!["edgar".to_string(), "edgar ætheling".to_string(), "england".to_string()]);

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "1100 e");
    assert_eq!(autocomplete_options, vec!["1100 edgar".to_string(), "1100 edgar ætheling".to_string(), "1100 england".to_string()]);

    // The only `w` keywords that `1087` should contain are `William` and
    // `William Rufus`. `Wessex` exists in the index but it is not related to
    // `1087`:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "1087 w");
    assert_eq!(autocomplete_options, vec!["1087 william".to_string(), "1087 william rufus".to_string()]);

} // fn