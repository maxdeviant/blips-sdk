#![allow(clippy::all, warnings)]
pub struct CreateGroups;
pub mod create_groups {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateGroups";
    pub const QUERY : & str = "mutation CreateGroups($container_id: ID!, $container_type: ContainerTypeEnum!, $names: [String!]!) {\n    createGroups(containerId: $container_id, containerType: $container_type, names: $names) {\n        ...Group\n    }\n}\n\nfragment Group on Group {\n    __typename\n    collapsed\n    date\n    id\n    keepTasks\n    name\n    order\n    projectId\n}" ;
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
    #[derive(Debug)]
    pub enum ContainerTypeEnum {
        DIARY,
        INBOX,
        PROJECT,
        Other(String),
    }
    impl ::serde::Serialize for ContainerTypeEnum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ContainerTypeEnum::DIARY => "DIARY",
                ContainerTypeEnum::INBOX => "INBOX",
                ContainerTypeEnum::PROJECT => "PROJECT",
                ContainerTypeEnum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContainerTypeEnum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "DIARY" => Ok(ContainerTypeEnum::DIARY),
                "INBOX" => Ok(ContainerTypeEnum::INBOX),
                "PROJECT" => Ok(ContainerTypeEnum::PROJECT),
                _ => Ok(ContainerTypeEnum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub container_id: ID,
        pub container_type: ContainerTypeEnum,
        pub names: Vec<String>,
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
        #[serde(rename = "createGroups")]
        pub create_groups: Vec<CreateGroupsCreateGroups>,
    }
    pub type CreateGroupsCreateGroups = Group;
}
impl graphql_client::GraphQLQuery for CreateGroups {
    type Variables = create_groups::Variables;
    type ResponseData = create_groups::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_groups::QUERY,
            operation_name: create_groups::OPERATION_NAME,
        }
    }
}
