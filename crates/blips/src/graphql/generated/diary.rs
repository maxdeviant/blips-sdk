#![allow(clippy::all, warnings)]
pub struct Diary;
pub mod diary {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Diary";
    pub const QUERY : & str = "query Diary($date: Date!) {\n    diary(date: $date) {\n        ...Diary\n    }\n}\n\nfragment Diary on Diary {\n    __typename\n    collapseCompleted\n    date\n    id\n    noteBody\n    supportsNotes\n}" ;
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
    type Date = crate::graphql::custom_scalars::Date;
    #[derive(Serialize)]
    pub struct Variables {
        pub date: Date,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Diary {
        #[serde(rename = "collapseCompleted")]
        pub collapse_completed: Boolean,
        pub date: Date,
        pub id: ID,
        #[serde(rename = "noteBody")]
        pub note_body: Option<String>,
        #[serde(rename = "supportsNotes")]
        pub supports_notes: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub diary: DiaryDiary,
    }
    pub type DiaryDiary = Diary;
}
impl graphql_client::GraphQLQuery for Diary {
    type Variables = diary::Variables;
    type ResponseData = diary::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: diary::QUERY,
            operation_name: diary::OPERATION_NAME,
        }
    }
}
