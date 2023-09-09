#![allow(clippy::all, warnings)]
pub struct PersistGroupOrder;
pub mod persist_group_order {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PersistGroupOrder";
    pub const QUERY : & str = "mutation PersistGroupOrder($order: [OrderInput!]) {\n    persistGroupOrder(order: $order) {\n        ...Group\n    }\n}\n\nfragment Group on Group {\n    __typename\n    collapsed\n    date\n    id\n    keepTasks\n    name\n    order\n    projectId\n}" ;
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
        #[serde(rename = "persistGroupOrder")]
        pub persist_group_order: Vec<PersistGroupOrderPersistGroupOrder>,
    }
    pub type PersistGroupOrderPersistGroupOrder = Group;
}
impl graphql_client::GraphQLQuery for PersistGroupOrder {
    type Variables = persist_group_order::Variables;
    type ResponseData = persist_group_order::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: persist_group_order::QUERY,
            operation_name: persist_group_order::OPERATION_NAME,
        }
    }
}
