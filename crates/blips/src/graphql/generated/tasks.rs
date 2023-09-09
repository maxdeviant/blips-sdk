#![allow(clippy::all, warnings)]
pub struct Tasks;
pub mod tasks {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Tasks";
    pub const QUERY : & str = "query Tasks($completed: Boolean, $date: Date, $due_date: Date, $focus: Boolean, $inbox: Boolean, $project_id: ID) {\n    tasks(completed: $completed, date: $date, dueDate: $due_date, focus: $focus, inbox: $inbox, projectId: $project_id) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
    type DateTime = crate::graphql::custom_scalars::DateTime;
    type Date = crate::graphql::custom_scalars::Date;
    #[derive(Serialize)]
    pub struct Variables {
        pub completed: Option<Boolean>,
        pub date: Option<Date>,
        pub due_date: Option<Date>,
        pub focus: Option<Boolean>,
        pub inbox: Option<Boolean>,
        pub project_id: Option<ID>,
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
        pub tasks: Option<Vec<TasksTasks>>,
    }
    pub type TasksTasks = Task;
}
impl graphql_client::GraphQLQuery for Tasks {
    type Variables = tasks::Variables;
    type ResponseData = tasks::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: tasks::QUERY,
            operation_name: tasks::OPERATION_NAME,
        }
    }
}
