#![allow(clippy::all, warnings)]
pub struct CreateTasks;
pub mod create_tasks {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateTasks";
    pub const QUERY : & str = "mutation CreateTasks($after: ID, $date: Date, $group_id: ID, $link: String, $due_date: Date, $names: [String!]!, $prioritized: Boolean, $project_id: ID, $tag_slug: String) {\n    createTasks(after: $after, date: $date, groupId: $group_id, link: $link, dueDate: $due_date, names: $names, prioritized: $prioritized, projectId: $project_id, tagSlug: $tag_slug) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
        pub after: Option<ID>,
        pub date: Option<Date>,
        pub group_id: Option<ID>,
        pub link: Option<String>,
        pub due_date: Option<Date>,
        pub names: Vec<String>,
        pub prioritized: Option<Boolean>,
        pub project_id: Option<ID>,
        pub tag_slug: Option<String>,
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
        #[serde(rename = "createTasks")]
        pub create_tasks: Vec<CreateTasksCreateTasks>,
    }
    pub type CreateTasksCreateTasks = Task;
}
impl graphql_client::GraphQLQuery for CreateTasks {
    type Variables = create_tasks::Variables;
    type ResponseData = create_tasks::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_tasks::QUERY,
            operation_name: create_tasks::OPERATION_NAME,
        }
    }
}
