#![allow(clippy::all, warnings)]
pub struct Me;
pub mod me {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Me";
    pub const QUERY : & str = "query Me {\n    me {\n        ...User\n    }\n}\n\nfragment User on User {\n    __typename\n    email\n    id\n    isMfaEnabled\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct User {
        pub email: String,
        pub id: ID,
        #[serde(rename = "isMfaEnabled")]
        pub is_mfa_enabled: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub me: Option<MeMe>,
    }
    pub type MeMe = User;
}
impl graphql_client::GraphQLQuery for Me {
    type Variables = me::Variables;
    type ResponseData = me::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: me::QUERY,
            operation_name: me::OPERATION_NAME,
        }
    }
}
