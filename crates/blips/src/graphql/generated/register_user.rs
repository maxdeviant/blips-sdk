#![allow(clippy::all, warnings)]
pub struct RegisterUser;
pub mod register_user {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RegisterUser";
    pub const QUERY : & str = "mutation RegisterUser($email: String!, $password: String!, $secret_code: String!) {\n    registerUser(email: $email, password: $password, secretCode: $secret_code) {\n        ...LoginResponse\n    }\n}\n\nfragment LoginResponse on LoginResponse {\n    __typename\n    accessToken\n}" ;
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
    pub struct Variables {
        pub email: String,
        pub password: String,
        pub secret_code: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct LoginResponse {
        #[serde(rename = "accessToken")]
        pub access_token: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "registerUser")]
        pub register_user: RegisterUserRegisterUser,
    }
    pub type RegisterUserRegisterUser = LoginResponse;
}
impl graphql_client::GraphQLQuery for RegisterUser {
    type Variables = register_user::Variables;
    type ResponseData = register_user::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: register_user::QUERY,
            operation_name: register_user::OPERATION_NAME,
        }
    }
}
