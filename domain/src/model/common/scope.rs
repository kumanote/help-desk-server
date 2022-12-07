use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const PROFILE: &'static str = "profile";
const READ_FAQ: &'static str = "read:faq";
const WRITE_FAQ: &'static str = "write:faq";
const ADMIN_FAQ: &'static str = "admin:faq";
const READ_ANNOUNCEMENT: &'static str = "read:announcement";
const WRITE_ANNOUNCEMENT: &'static str = "write:announcement";
const ADMIN_ANNOUNCEMENT: &'static str = "admin:announcement";
const READ_INQUIRY: &'static str = "read:inquiry";
const WRITE_INQUIRY: &'static str = "write:inquiry";
const ADMIN_INQUIRY: &'static str = "admin:inquiry";
const READ_WORKSPACE: &'static str = "read:workspace";
const ADMIN_WORKSPACE: &'static str = "admin:workspace";
const GROUP_MEMBER: &'static str = "group_member";
const GROUP_ADMIN: &'static str = "group_admin";
const GROUP_OWNER: &'static str = "group_owner";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    Profile,
    ReadFaq,
    WriteFaq,
    AdminFaq,
    ReadAnnouncement,
    WriteAnnouncement,
    AdminAnnouncement,
    ReadInquiry,
    WriteInquiry,
    AdminInquiry,
    ReadWorkspace,
    AdminWorkspace,
    GroupMember,
    GroupAdmin,
    GroupOwner,
}

impl Scope {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match self {
            Self::Profile => PROFILE,
            Self::ReadFaq => READ_FAQ,
            Self::WriteFaq => WRITE_FAQ,
            Self::AdminFaq => ADMIN_FAQ,
            Self::ReadAnnouncement => READ_ANNOUNCEMENT,
            Self::WriteAnnouncement => WRITE_ANNOUNCEMENT,
            Self::AdminAnnouncement => ADMIN_ANNOUNCEMENT,
            Self::ReadInquiry => READ_INQUIRY,
            Self::WriteInquiry => WRITE_INQUIRY,
            Self::AdminInquiry => ADMIN_INQUIRY,
            Self::ReadWorkspace => READ_WORKSPACE,
            Self::AdminWorkspace => ADMIN_WORKSPACE,
            Self::GroupMember => GROUP_MEMBER,
            Self::GroupAdmin => GROUP_ADMIN,
            Self::GroupOwner => GROUP_OWNER,
        }
    }
}

impl Deref for Scope {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for Scope {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for Scope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            PROFILE => Ok(Self::Profile),
            READ_FAQ => Ok(Self::ReadFaq),
            WRITE_FAQ => Ok(Self::WriteFaq),
            ADMIN_FAQ => Ok(Self::AdminFaq),
            READ_ANNOUNCEMENT => Ok(Self::ReadAnnouncement),
            WRITE_ANNOUNCEMENT => Ok(Self::WriteAnnouncement),
            ADMIN_ANNOUNCEMENT => Ok(Self::AdminAnnouncement),
            READ_INQUIRY => Ok(Self::ReadInquiry),
            WRITE_INQUIRY => Ok(Self::WriteInquiry),
            ADMIN_INQUIRY => Ok(Self::AdminInquiry),
            READ_WORKSPACE => Ok(Self::ReadWorkspace),
            ADMIN_WORKSPACE => Ok(Self::AdminWorkspace),
            GROUP_MEMBER => Ok(Self::GroupMember),
            GROUP_ADMIN => Ok(Self::GroupAdmin),
            GROUP_OWNER => Ok(Self::GroupOwner),
            _ => Err(Error::UnsupportedScope {
                value: s.to_owned(),
            }),
        }
    }
}

impl From<String> for Scope {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}

impl Serialize for Scope {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
