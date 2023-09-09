#![allow(clippy::all, warnings)]
pub struct UpdateNote;
pub mod update_note {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateNote";
    pub const QUERY : & str = "mutation UpdateNote($body: String, $date: Date, $end_date: Date, $hide_preview: Boolean, $last_updated_at: DateTime, $name: String, $note_id: ID!, $project_id: ID) {\n    updateNote(body: $body, date: $date, endDate: $end_date, hidePreview: $hide_preview, lastUpdatedAt: $last_updated_at, name: $name, noteId: $note_id, projectId: $project_id) {\n        ...UpdateNoteResult\n    }\n}\n\nfragment UpdateNoteResult on UpdateNoteResult {\n    __typename\n    \n}" ;
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
        pub body: Option<String>,
        pub date: Option<Date>,
        pub end_date: Option<Date>,
        pub hide_preview: Option<Boolean>,
        pub last_updated_at: Option<DateTime>,
        pub name: Option<String>,
        pub note_id: ID,
        pub project_id: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum UpdateNoteResult {
        NoteUpdateOutdated,
        NoteUpdated,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "updateNote")]
        pub update_note: UpdateNoteUpdateNote,
    }
    pub type UpdateNoteUpdateNote = UpdateNoteResult;
}
impl graphql_client::GraphQLQuery for UpdateNote {
    type Variables = update_note::Variables;
    type ResponseData = update_note::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_note::QUERY,
            operation_name: update_note::OPERATION_NAME,
        }
    }
}
