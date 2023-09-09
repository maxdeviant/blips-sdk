#![allow(clippy::all, warnings)]
pub struct DeleteGroup;
pub mod delete_group {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteGroup";
    pub const QUERY : & str = "mutation DeleteGroup($delete_tasks: Boolean, $group_id: ID!) {\n    deleteGroup(deleteTasks: $delete_tasks, groupId: $group_id) {\n        ...Group\n    }\n}\n\nfragment Group on Group {\n    __typename\n    collapsed\n    date\n    id\n    keepTasks\n    name\n    order\n    projectId\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub delete_tasks: Option<Boolean>,
        pub group_id: ID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Group {
        pub collapsed: Option<Boolean>,
        pub date: Option<Date>,
        pub id: ID,
        #[serde(rename = "keepTasks")]
        pub keep_tasks: Boolean,
        pub name: String,
        pub order: Option<Int>,
        #[serde(rename = "projectId")]
        pub project_id: Option<ID>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "deleteGroup")]
        pub delete_group: DeleteGroupDeleteGroup,
    }
    pub type DeleteGroupDeleteGroup = Group;
}
impl graphql_client::GraphQLQuery for DeleteGroup {
    type Variables = delete_group::Variables;
    type ResponseData = delete_group::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_group::QUERY,
            operation_name: delete_group::OPERATION_NAME,
        }
    }
}
