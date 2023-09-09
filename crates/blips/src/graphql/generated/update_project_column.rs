#![allow(clippy::all, warnings)]
pub struct UpdateProjectColumn;
pub mod update_project_column {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateProjectColumn";
    pub const QUERY : & str = "mutation UpdateProjectColumn($collapsed: Boolean, $name: String, $project_column_id: ID!) {\n    updateProjectColumn(collapsed: $collapsed, name: $name, projectColumnId: $project_column_id) {\n        ...ProjectColumn\n    }\n}\n\nfragment ProjectColumn on ProjectColumn {\n    __typename\n    collapsed\n    id\n    name\n    order\n}" ;
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
        pub collapsed: Option<Boolean>,
        pub name: Option<String>,
        pub project_column_id: ID,
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
        #[serde(rename = "updateProjectColumn")]
        pub update_project_column: UpdateProjectColumnUpdateProjectColumn,
    }
    pub type UpdateProjectColumnUpdateProjectColumn = ProjectColumn;
}
impl graphql_client::GraphQLQuery for UpdateProjectColumn {
    type Variables = update_project_column::Variables;
    type ResponseData = update_project_column::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_project_column::QUERY,
            operation_name: update_project_column::OPERATION_NAME,
        }
    }
}
