#![allow(clippy::all, warnings)]
pub struct UpdateUserSettings;
pub mod update_user_settings {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateUserSettings";
    pub const QUERY : & str = "mutation UpdateUserSettings($badge_count_mode: BadgeCountModeEnum) {\n    updateUserSettings(badgeCountMode: $badge_count_mode) {\n        ...User\n    }\n}\n\nfragment User on User {\n    __typename\n    email\n    id\n    isMfaEnabled\n}" ;
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
    #[derive(Debug)]
    pub enum BadgeCountModeEnum {
        DUE,
        NONE,
        PRIORITY,
        PRIORITY_DUE,
        Other(String),
    }
    impl ::serde::Serialize for BadgeCountModeEnum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                BadgeCountModeEnum::DUE => "DUE",
                BadgeCountModeEnum::NONE => "NONE",
                BadgeCountModeEnum::PRIORITY => "PRIORITY",
                BadgeCountModeEnum::PRIORITY_DUE => "PRIORITY_DUE",
                BadgeCountModeEnum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BadgeCountModeEnum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "DUE" => Ok(BadgeCountModeEnum::DUE),
                "NONE" => Ok(BadgeCountModeEnum::NONE),
                "PRIORITY" => Ok(BadgeCountModeEnum::PRIORITY),
                "PRIORITY_DUE" => Ok(BadgeCountModeEnum::PRIORITY_DUE),
                _ => Ok(BadgeCountModeEnum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub badge_count_mode: Option<BadgeCountModeEnum>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct User {
        pub email: String,
        pub id: ID,
        #[serde(rename = "isMfaEnabled")]
        pub is_mfa_enabled: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "updateUserSettings")]
        pub update_user_settings: UpdateUserSettingsUpdateUserSettings,
    }
    pub type UpdateUserSettingsUpdateUserSettings = User;
}
impl graphql_client::GraphQLQuery for UpdateUserSettings {
    type Variables = update_user_settings::Variables;
    type ResponseData = update_user_settings::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_user_settings::QUERY,
            operation_name: update_user_settings::OPERATION_NAME,
        }
    }
}
