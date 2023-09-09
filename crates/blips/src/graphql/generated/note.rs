#![allow(clippy::all, warnings)]
pub struct Note;
pub mod note {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Note";
    pub const QUERY : & str = "query Note($note_id: ID!) {\n    note(noteId: $note_id) {\n        ...Note\n    }\n}\n\nfragment Note on Note {\n    __typename\n    body\n    date\n    endDate\n    hidePreview\n    id\n    name\n    updatedAt\n}" ;
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
        pub note_id: ID,
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
        pub note: Option<NoteNote>,
    }
    pub type NoteNote = Note;
}
impl graphql_client::GraphQLQuery for Note {
    type Variables = note::Variables;
    type ResponseData = note::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: note::QUERY,
            operation_name: note::OPERATION_NAME,
        }
    }
}
