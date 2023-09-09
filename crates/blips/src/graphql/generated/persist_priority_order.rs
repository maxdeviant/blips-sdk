#![allow(clippy::all, warnings)]
pub struct PersistPriorityOrder;
pub mod persist_priority_order {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PersistPriorityOrder";
    pub const QUERY : & str = "mutation PersistPriorityOrder($order: [OrderInput!]) {\n    persistPriorityOrder(order: $order) {\n        ...Task\n    }\n}\n\nfragment Task on Task {\n    __typename\n    completed\n    completedAt\n    date\n    description\n    dueDate\n    groupIds\n    id\n    isRecurring\n    link\n    name\n    priorityOrder\n    spring\n}" ;
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
    pub struct OrderInput {
        pub id: Option<ID>,
        pub order: Option<Int>,
        #[serde(rename = "destinationGroupId")]
        pub destination_group_id: Option<ID>,
        #[serde(rename = "sourceGroupId")]
        pub source_group_id: Option<ID>,
        #[serde(rename = "projectColumnId")]
        pub project_column_id: Option<ID>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub order: Option<Vec<OrderInput>>,
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
        #[serde(rename = "persistPriorityOrder")]
        pub persist_priority_order: Vec<PersistPriorityOrderPersistPriorityOrder>,
    }
    pub type PersistPriorityOrderPersistPriorityOrder = Task;
}
impl graphql_client::GraphQLQuery for PersistPriorityOrder {
    type Variables = persist_priority_order::Variables;
    type ResponseData = persist_priority_order::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: persist_priority_order::QUERY,
            operation_name: persist_priority_order::OPERATION_NAME,
        }
    }
}
