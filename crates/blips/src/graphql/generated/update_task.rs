#![allow(clippy::all, warnings)]
pub struct UpdateTask;
pub mod update_task {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateTask";
    pub const QUERY : & str = "mutation UpdateTask($date: Date, $description: String, $due_date: Date, $link: String, $name: String, $project_id: ID, $recurrence: RecurrenceInput, $task_id: ID!) {\n    updateTask(date: $date, description: $description, dueDate: $due_date, link: $link, name: $name, projectId: $project_id, recurrence: $recurrence, taskId: $task_id) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
    pub struct RecurrenceInput {
        pub id: Option<ID>,
        pub friday: Option<Boolean>,
        pub kind: String,
        pub monday: Option<Boolean>,
        pub rule: Option<String>,
        pub separation: Option<Int>,
        pub saturday: Option<Boolean>,
        pub sunday: Option<Boolean>,
        pub thursday: Option<Boolean>,
        pub tuesday: Option<Boolean>,
        pub wednesday: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub date: Option<Date>,
        pub description: Option<String>,
        pub due_date: Option<Date>,
        pub link: Option<String>,
        pub name: Option<String>,
        pub project_id: Option<ID>,
        pub recurrence: Option<RecurrenceInput>,
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
        #[serde(rename = "updateTask")]
        pub update_task: UpdateTaskUpdateTask,
    }
    pub type UpdateTaskUpdateTask = Task;
}
impl graphql_client::GraphQLQuery for UpdateTask {
    type Variables = update_task::Variables;
    type ResponseData = update_task::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_task::QUERY,
            operation_name: update_task::OPERATION_NAME,
        }
    }
}
