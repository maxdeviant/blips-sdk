#![allow(clippy::all, warnings)]
pub struct DeleteNote;
pub mod delete_note {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteNote";
    pub const QUERY : & str = "mutation DeleteNote($note_id: ID!) {\n    deleteNote(noteId: $note_id) {\n        ...Note\n    }\n}\n\nfragment Note on Note {\n    __typename\n    body\n    date\n    endDate\n    hidePreview\n    id\n    name\n    updatedAt\n}" ;
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
        #[serde(rename = "deleteNote")]
        pub delete_note: DeleteNoteDeleteNote,
    }
    pub type DeleteNoteDeleteNote = Note;
}
impl graphql_client::GraphQLQuery for DeleteNote {
    type Variables = delete_note::Variables;
    type ResponseData = delete_note::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_note::QUERY,
            operation_name: delete_note::OPERATION_NAME,
        }
    }
}
