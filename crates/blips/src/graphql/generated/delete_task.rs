#![allow(clippy::all, warnings)]
pub struct DeleteTask;
pub mod delete_task {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteTask";
    pub const QUERY : & str = "mutation DeleteTask($task_id: ID!) {\n    deleteTask(taskId: $task_id) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
        #[serde(rename = "deleteTask")]
        pub delete_task: DeleteTaskDeleteTask,
    }
    pub type DeleteTaskDeleteTask = Task;
}
impl graphql_client::GraphQLQuery for DeleteTask {
    type Variables = delete_task::Variables;
    type ResponseData = delete_task::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_task::QUERY,
            operation_name: delete_task::OPERATION_NAME,
        }
    }
}
