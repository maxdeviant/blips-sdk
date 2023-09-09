#![allow(clippy::all, warnings)]
pub struct Container;
pub mod container {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Container";
    pub const QUERY : & str = "query Container($date: Date, $inbox: Boolean, $project_id: ID) {\n    container(date: $date, inbox: $inbox, projectId: $project_id) {\n        ...Container\n    }\n}\n\nfragment Container on Container {\n    __typename\n    \n}" ;
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
        pub date: Option<Date>,
        pub inbox: Option<Boolean>,
        pub project_id: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum Container {
        Diary,
        Inbox,
        Project,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub container: ContainerContainer,
    }
    pub type ContainerContainer = Container;
}
impl graphql_client::GraphQLQuery for Container {
    type Variables = container::Variables;
    type ResponseData = container::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: container::QUERY,
            operation_name: container::OPERATION_NAME,
        }
    }
}
