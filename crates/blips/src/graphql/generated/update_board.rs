#![allow(clippy::all, warnings)]
pub struct UpdateBoard;
pub mod update_board {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateBoard";
    pub const QUERY : & str = "mutation UpdateBoard($emoji: String, $name: String, $board_id: ID!, $project_completed_project_column_id: ID, $task_completed_project_column_id: ID) {\n    updateBoard(emoji: $emoji, name: $name, boardId: $board_id, projectCompletedProjectColumnId: $project_completed_project_column_id, taskCompletedProjectColumnId: $task_completed_project_column_id) {\n        ...Board\n    }\n}\n\nfragment Board on Board {\n    __typename\n    archivedAt\n    emoji\n    id\n    lastViewedAt\n    name\n    projectCompletedProjectColumnId\n    taskCompletedProjectColumnId\n}" ;
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
        pub emoji: Option<String>,
        pub name: Option<String>,
        pub board_id: ID,
        pub project_completed_project_column_id: Option<ID>,
        pub task_completed_project_column_id: Option<ID>,
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
        #[serde(rename = "updateBoard")]
        pub update_board: UpdateBoardUpdateBoard,
    }
    pub type UpdateBoardUpdateBoard = Board;
}
impl graphql_client::GraphQLQuery for UpdateBoard {
    type Variables = update_board::Variables;
    type ResponseData = update_board::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_board::QUERY,
            operation_name: update_board::OPERATION_NAME,
        }
    }
}
