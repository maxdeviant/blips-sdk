#![allow(clippy::all, warnings)]
pub struct ArchiveBoard;
pub mod archive_board {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ArchiveBoard";
    pub const QUERY : & str = "mutation ArchiveBoard($board_id: ID!) {\n    archiveBoard(boardId: $board_id) {\n        ...Board\n    }\n}\n\nfragment Board on Board {\n    __typename\n    archivedAt\n    emoji\n    id\n    lastViewedAt\n    name\n    projectCompletedProjectColumnId\n    taskCompletedProjectColumnId\n}" ;
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
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub board_id: ID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Board {
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub emoji: Option<String>,
        pub id: ID,
        #[serde(rename = "lastViewedAt")]
        pub last_viewed_at: Option<DateTime>,
        pub name: String,
        #[serde(rename = "projectCompletedProjectColumnId")]
        pub project_completed_project_column_id: Option<ID>,
        #[serde(rename = "taskCompletedProjectColumnId")]
        pub task_completed_project_column_id: Option<ID>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "archiveBoard")]
        pub archive_board: Vec<ArchiveBoardArchiveBoard>,
    }
    pub type ArchiveBoardArchiveBoard = Board;
}
impl graphql_client::GraphQLQuery for ArchiveBoard {
    type Variables = archive_board::Variables;
    type ResponseData = archive_board::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: archive_board::QUERY,
            operation_name: archive_board::OPERATION_NAME,
        }
    }
}
