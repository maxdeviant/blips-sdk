#![allow(clippy::all, warnings)]
pub struct GenerateNewOtp;
pub mod generate_new_otp {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GenerateNewOtp";
    pub const QUERY : & str = "mutation GenerateNewOtp {\n    generateNewOtp {\n        ...GenerateNewOtpResult\n    }\n}\n\nfragment GenerateNewOtpResult on GenerateNewOtpResult {\n    __typename\n    \n}" ;
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
    #[serde(tag = "__typename")]
    pub enum GenerateNewOtpResult {
        NewOtpGenerated,
        UserAlreadyHasOtp,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "generateNewOtp")]
        pub generate_new_otp: GenerateNewOtpGenerateNewOtp,
    }
    pub type GenerateNewOtpGenerateNewOtp = GenerateNewOtpResult;
}
impl graphql_client::GraphQLQuery for GenerateNewOtp {
    type Variables = generate_new_otp::Variables;
    type ResponseData = generate_new_otp::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: generate_new_otp::QUERY,
            operation_name: generate_new_otp::OPERATION_NAME,
        }
    }
}
