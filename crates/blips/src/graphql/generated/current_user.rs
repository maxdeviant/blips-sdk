#![allow(clippy::all, warnings)]
pub struct CurrentUser;
pub mod current_user {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CurrentUser";
    pub const QUERY : & str = "query CurrentUser {\n    currentUser {\n        ...User\n    }\n}\n\nfragment User on User {\n    __typename\n    email\n    id\n    isMfaEnabled\n}" ;
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
        #[serde(rename = "currentUser")]
        pub current_user: CurrentUserCurrentUser,
    }
    pub type CurrentUserCurrentUser = User;
}
impl graphql_client::GraphQLQuery for CurrentUser {
    type Variables = current_user::Variables;
    type ResponseData = current_user::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: current_user::QUERY,
            operation_name: current_user::OPERATION_NAME,
        }
    }
}
