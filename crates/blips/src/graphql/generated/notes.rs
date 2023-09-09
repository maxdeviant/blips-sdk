#![allow(clippy::all, warnings)]
pub struct Notes;
pub mod notes {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Notes";
    pub const QUERY : & str = "query Notes($date: Date, $project_id: ID, $query: String) {\n    notes(date: $date, projectId: $project_id, query: $query) {\n        ...Note\n    }\n}\n\nfragment Note on Note {\n    __typename\n    body\n    date\n    endDate\n    hidePreview\n    id\n    name\n    updatedAt\n}" ;
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
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub date: Option<Date>,
        pub project_id: Option<ID>,
        pub query: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Note {
        pub body: Option<String>,
        pub date: Option<Date>,
        #[serde(rename = "endDate")]
        pub end_date: Option<Date>,
        #[serde(rename = "hidePreview")]
        pub hide_preview: Boolean,
        pub id: ID,
        pub name: String,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub notes: Option<Vec<NotesNotes>>,
    }
    pub type NotesNotes = Note;
}
impl graphql_client::GraphQLQuery for Notes {
    type Variables = notes::Variables;
    type ResponseData = notes::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: notes::QUERY,
            operation_name: notes::OPERATION_NAME,
        }
    }
}
