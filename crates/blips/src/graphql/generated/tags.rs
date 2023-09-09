#![allow(clippy::all, warnings)]
pub struct Tags;
pub mod tags {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Tags";
    pub const QUERY : & str = "query Tags {\n    tags {\n        ...Tag\n    }\n}\n\nfragment Tag on Tag {\n    __typename\n    id\n    name\n    slug\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct Tag {
        pub id: ID,
        pub name: String,
        pub slug: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub tags: Option<Vec<TagsTags>>,
    }
    pub type TagsTags = Tag;
}
impl graphql_client::GraphQLQuery for Tags {
    type Variables = tags::Variables;
    type ResponseData = tags::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: tags::QUERY,
            operation_name: tags::OPERATION_NAME,
        }
    }
}
