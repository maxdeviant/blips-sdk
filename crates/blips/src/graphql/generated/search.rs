#![allow(clippy::all, warnings)]
pub struct Search;
pub mod search {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Search";
    pub const QUERY : & str = "query Search($query: String!) {\n    search(query: $query) {\n        ...Search\n    }\n}\n\nfragment Search on Search {\n    __typename\n    \n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub query: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum Search {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub search: SearchSearch,
    }
    pub type SearchSearch = Search;
}
impl graphql_client::GraphQLQuery for Search {
    type Variables = search::Variables;
    type ResponseData = search::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search::QUERY,
            operation_name: search::OPERATION_NAME,
        }
    }
}
