#![allow(clippy::all, warnings)]
pub struct TagTask;
pub mod tag_task {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TagTask";
    pub const QUERY : & str = "mutation TagTask($tag_id: ID!, $task_id: ID!) {\n    tagTask(tagId: $tag_id, taskId: $task_id) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
        pub tag_id: ID,
        pub task_id: ID,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Task {
        pub completed: Boolean,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        pub date: Option<Date>,
        pub description: Option<String>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<Date>,
        #[serde(rename = "groupIds")]
        pub group_ids: Vec<ID>,
        pub id: ID,
        #[serde(rename = "isRecurring")]
        pub is_recurring: Boolean,
        pub link: Option<String>,
        pub name: String,
        #[serde(rename = "priorityOrder")]
        pub priority_order: Option<Int>,
        pub spring: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "tagTask")]
        pub tag_task: TagTaskTagTask,
    }
    pub type TagTaskTagTask = Task;
}
impl graphql_client::GraphQLQuery for TagTask {
    type Variables = tag_task::Variables;
    type ResponseData = tag_task::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: tag_task::QUERY,
            operation_name: tag_task::OPERATION_NAME,
        }
    }
}
