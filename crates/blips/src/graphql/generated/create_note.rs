#![allow(clippy::all, warnings)]
pub struct CreateNote;
pub mod create_note {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateNote";
    pub const QUERY : & str = "mutation CreateNote($date: Date, $name: String, $project_id: ID) {\n    createNote(date: $date, name: $name, projectId: $project_id) {\n        ...Note\n    }\n}\n\nfragment Note on Note {\n    __typename\n    body\n    date\n    endDate\n    hidePreview\n    id\n    name\n    updatedAt\n}" ;
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
        pub name: Option<String>,
        pub project_id: Option<ID>,
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
        #[serde(rename = "createNote")]
        pub create_note: CreateNoteCreateNote,
    }
    pub type CreateNoteCreateNote = Note;
}
impl graphql_client::GraphQLQuery for CreateNote {
    type Variables = create_note::Variables;
    type ResponseData = create_note::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_note::QUERY,
            operation_name: create_note::OPERATION_NAME,
        }
    }
}
