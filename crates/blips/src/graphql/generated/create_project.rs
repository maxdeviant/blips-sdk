#![allow(clippy::all, warnings)]
pub struct CreateProject;
pub mod create_project {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateProject";
    pub const QUERY : & str = "mutation CreateProject($board_id: ID, $name: String, $project_column_id: ID, $source_task_id: ID) {\n    createProject(boardId: $board_id, name: $name, projectColumnId: $project_column_id, sourceTaskId: $source_task_id) {\n        ...Project\n    }\n}\n\nfragment Project on Project {\n    __typename\n    collapseCompleted\n    completed\n    completedAt\n    date\n    endDate\n    id\n    link\n    name\n    noteBody\n    order\n    springEnabled\n    supportsNotes\n}" ;
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
        pub board_id: Option<ID>,
        pub name: Option<String>,
        pub project_column_id: Option<ID>,
        pub source_task_id: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Project {
        #[serde(rename = "collapseCompleted")]
        pub collapse_completed: Boolean,
        pub completed: Boolean,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        pub date: Option<Date>,
        #[serde(rename = "endDate")]
        pub end_date: Option<Date>,
        pub id: ID,
        pub link: Option<String>,
        pub name: String,
        #[serde(rename = "noteBody")]
        pub note_body: Option<String>,
        pub order: Option<Int>,
        #[serde(rename = "springEnabled")]
        pub spring_enabled: Boolean,
        #[serde(rename = "supportsNotes")]
        pub supports_notes: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "createProject")]
        pub create_project: CreateProjectCreateProject,
    }
    pub type CreateProjectCreateProject = Project;
}
impl graphql_client::GraphQLQuery for CreateProject {
    type Variables = create_project::Variables;
    type ResponseData = create_project::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_project::QUERY,
            operation_name: create_project::OPERATION_NAME,
        }
    }
}
