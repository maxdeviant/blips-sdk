#![allow(clippy::all, warnings)]
pub struct ProjectColumns;
pub mod project_columns {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectColumns";
    pub const QUERY : & str = "query ProjectColumns {\n    projectColumns {\n        ...ProjectColumn\n    }\n}\n\nfragment ProjectColumn on ProjectColumn {\n    __typename\n    collapsed\n    id\n    name\n    order\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ProjectColumn {
        pub collapsed: Boolean,
        pub id: ID,
        pub name: String,
        pub order: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "projectColumns")]
        pub project_columns: Option<Vec<ProjectColumnsProjectColumns>>,
    }
    pub type ProjectColumnsProjectColumns = ProjectColumn;
}
impl graphql_client::GraphQLQuery for ProjectColumns {
    type Variables = project_columns::Variables;
    type ResponseData = project_columns::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_columns::QUERY,
            operation_name: project_columns::OPERATION_NAME,
        }
    }
}
