#![allow(clippy::all, warnings)]
pub struct EnableOtp;
pub mod enable_otp {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "EnableOtp";
    pub const QUERY : & str = "mutation EnableOtp($otp_attempt: String!, $otp_secret: String!) {\n    enableOtp(otpAttempt: $otp_attempt, otpSecret: $otp_secret) {\n        ...EnableOtpResult\n    }\n}\n\nfragment EnableOtpResult on EnableOtpResult {\n    __typename\n    \n}" ;
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
        pub otp_attempt: String,
        pub otp_secret: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum EnableOtpResult {
        InvalidOtpAttempt,
        OtpEnabled,
        UserAlreadyHasOtp,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "enableOtp")]
        pub enable_otp: EnableOtpEnableOtp,
    }
    pub type EnableOtpEnableOtp = EnableOtpResult;
}
impl graphql_client::GraphQLQuery for EnableOtp {
    type Variables = enable_otp::Variables;
    type ResponseData = enable_otp::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: enable_otp::QUERY,
            operation_name: enable_otp::OPERATION_NAME,
        }
    }
}
