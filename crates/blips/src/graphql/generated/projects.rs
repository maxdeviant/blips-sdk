#![allow(clippy::all, warnings)]
pub struct Projects;
pub mod projects {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Projects";
    pub const QUERY : & str = "query Projects($date: Date, $limit: Int, $query: String, $board_id: ID) {\n    projects(date: $date, limit: $limit, query: $query, boardId: $board_id) {\n        ...Project\n    }\n}\n\nfragment Project on Project {\n    __typename\n    collapseCompleted\n    completed\n    completedAt\n    date\n    endDate\n    id\n    link\n    name\n    noteBody\n    order\n    springEnabled\n    supportsNotes\n}" ;
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
        pub limit: Option<Int>,
        pub query: Option<String>,
        pub board_id: Option<ID>,
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
        pub projects: Option<Vec<ProjectsProjects>>,
    }
    pub type ProjectsProjects = Project;
}
impl graphql_client::GraphQLQuery for Projects {
    type Variables = projects::Variables;
    type ResponseData = projects::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: projects::QUERY,
            operation_name: projects::OPERATION_NAME,
        }
    }
}
