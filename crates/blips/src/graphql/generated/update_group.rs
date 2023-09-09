#![allow(clippy::all, warnings)]
pub struct UpdateGroup;
pub mod update_group {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateGroup";
    pub const QUERY : & str = "mutation UpdateGroup($collapsed: Boolean, $date: Date, $group_id: ID!, $keep_tasks: Boolean, $name: String) {\n    updateGroup(collapsed: $collapsed, date: $date, groupId: $group_id, keepTasks: $keep_tasks, name: $name) {\n        ...Group\n    }\n}\n\nfragment Group on Group {\n    __typename\n    collapsed\n    date\n    id\n    keepTasks\n    name\n    order\n    projectId\n}" ;
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
        pub collapsed: Option<Boolean>,
        pub date: Option<Date>,
        pub group_id: ID,
        pub keep_tasks: Option<Boolean>,
        pub name: Option<String>,
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
        #[serde(rename = "updateGroup")]
        pub update_group: UpdateGroupUpdateGroup,
    }
    pub type UpdateGroupUpdateGroup = Group;
}
impl graphql_client::GraphQLQuery for UpdateGroup {
    type Variables = update_group::Variables;
    type ResponseData = update_group::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_group::QUERY,
            operation_name: update_group::OPERATION_NAME,
        }
    }
}
