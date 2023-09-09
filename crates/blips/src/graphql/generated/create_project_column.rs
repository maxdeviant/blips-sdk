#![allow(clippy::all, warnings)]
pub struct CreateProjectColumn;
pub mod create_project_column {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateProjectColumn";
    pub const QUERY : & str = "mutation CreateProjectColumn($board_id: ID!, $name: String!) {\n    createProjectColumn(boardId: $board_id, name: $name) {\n        ...ProjectColumn\n    }\n}\n\nfragment ProjectColumn on ProjectColumn {\n    __typename\n    collapsed\n    id\n    name\n    order\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub board_id: ID,
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ProjectColumn {
        pub collapsed: Boolean,
        pub id: ID,
        pub name: String,
        pub order: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "createProjectColumn")]
        pub create_project_column: CreateProjectColumnCreateProjectColumn,
    }
    pub type CreateProjectColumnCreateProjectColumn = ProjectColumn;
}
impl graphql_client::GraphQLQuery for CreateProjectColumn {
    type Variables = create_project_column::Variables;
    type ResponseData = create_project_column::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_project_column::QUERY,
            operation_name: create_project_column::OPERATION_NAME,
        }
    }
}
