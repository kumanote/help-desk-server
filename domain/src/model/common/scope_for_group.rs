use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const GROUP_MEMBER: &'static str = "group_member";
const GROUP_ADMIN: &'static str = "group_admin";
const GROUP_OWNER: &'static str = "group_owner";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeForGroup {
    GroupMember,
    GroupAdmin,
    GroupOwner,
}

impl ScopeForGroup {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for ScopeForGroup {
    fn as_ref(&self) -> &str {
        match self {
            Self::GroupMember => GROUP_MEMBER,
            Self::GroupAdmin => GROUP_ADMIN,
            Self::GroupOwner => GROUP_OWNER,
        }
    }
}

impl Deref for ScopeForGroup {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ScopeForGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for ScopeForGroup {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for ScopeForGroup {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            GROUP_MEMBER => Ok(Self::GroupMember),
            GROUP_ADMIN => Ok(Self::GroupAdmin),
            GROUP_OWNER => Ok(Self::GroupOwner),
            _ => Err(Error::UnsupportedScope {
                value: s.to_owned(),
            }),
        }
    }
}

impl From<String> for ScopeForGroup {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}

impl Serialize for ScopeForGroup {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ScopeForGroup {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
