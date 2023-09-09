#![allow(clippy::all, warnings)]
pub struct UpdateDiary;
pub mod update_diary {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateDiary";
    pub const QUERY : & str = "mutation UpdateDiary($date: Date!, $note_body: String!) {\n    updateDiary(date: $date, noteBody: $note_body) {\n        ...Diary\n    }\n}\n\nfragment Diary on Diary {\n    __typename\n    collapseCompleted\n    date\n    id\n    noteBody\n    supportsNotes\n}" ;
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
        pub note_body: String,
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
        #[serde(rename = "updateDiary")]
        pub update_diary: UpdateDiaryUpdateDiary,
    }
    pub type UpdateDiaryUpdateDiary = Diary;
}
impl graphql_client::GraphQLQuery for UpdateDiary {
    type Variables = update_diary::Variables;
    type ResponseData = update_diary::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_diary::QUERY,
            operation_name: update_diary::OPERATION_NAME,
        }
    }
}
