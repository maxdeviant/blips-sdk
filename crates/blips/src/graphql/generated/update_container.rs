#![allow(clippy::all, warnings)]
pub struct UpdateContainer;
pub mod update_container {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateContainer";
    pub const QUERY : & str = "mutation UpdateContainer($collapse_completed: Boolean, $date: Date, $note_body: String, $project_id: ID, $state: DiaryStateEnum) {\n    updateContainer(collapseCompleted: $collapse_completed, date: $date, noteBody: $note_body, projectId: $project_id, state: $state) {\n        ...Container\n    }\n}\n\nfragment Container on Container {\n    __typename\n    \n}" ;
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
    pub enum DiaryStateEnum {
        FRESH,
        PLANNED,
        Other(String),
    }
    impl ::serde::Serialize for DiaryStateEnum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                DiaryStateEnum::FRESH => "FRESH",
                DiaryStateEnum::PLANNED => "PLANNED",
                DiaryStateEnum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiaryStateEnum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "FRESH" => Ok(DiaryStateEnum::FRESH),
                "PLANNED" => Ok(DiaryStateEnum::PLANNED),
                _ => Ok(DiaryStateEnum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub collapse_completed: Option<Boolean>,
        pub date: Option<Date>,
        pub note_body: Option<String>,
        pub project_id: Option<ID>,
        pub state: Option<DiaryStateEnum>,
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
        #[serde(rename = "updateContainer")]
        pub update_container: UpdateContainerUpdateContainer,
    }
    pub type UpdateContainerUpdateContainer = Container;
}
impl graphql_client::GraphQLQuery for UpdateContainer {
    type Variables = update_container::Variables;
    type ResponseData = update_container::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_container::QUERY,
            operation_name: update_container::OPERATION_NAME,
        }
    }
}
