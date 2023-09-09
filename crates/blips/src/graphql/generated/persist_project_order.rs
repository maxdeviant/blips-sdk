#![allow(clippy::all, warnings)]
pub struct PersistProjectOrder;
pub mod persist_project_order {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PersistProjectOrder";
    pub const QUERY : & str = "mutation PersistProjectOrder($order: [OrderInput!]) {\n    persistProjectOrder(order: $order) {\n        ...ProjectColumn\n    }\n}\n\nfragment ProjectColumn on ProjectColumn {\n    __typename\n    collapsed\n    id\n    name\n    order\n}" ;
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
    pub struct ProjectColumn {
        pub collapsed: Boolean,
        pub id: ID,
        pub name: String,
        pub order: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "persistProjectOrder")]
        pub persist_project_order: Vec<PersistProjectOrderPersistProjectOrder>,
    }
    pub type PersistProjectOrderPersistProjectOrder = ProjectColumn;
}
impl graphql_client::GraphQLQuery for PersistProjectOrder {
    type Variables = persist_project_order::Variables;
    type ResponseData = persist_project_order::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: persist_project_order::QUERY,
            operation_name: persist_project_order::OPERATION_NAME,
        }
    }
}
