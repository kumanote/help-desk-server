macro_rules! def_id {
    ($struct_name:ident) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $struct_name(String);

        impl $struct_name {
            pub fn generate() -> Self {
                let id = ulid::Ulid::new();
                Self(id.to_string())
            }

            #[inline(always)]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = str;
            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl Into<String> for $struct_name {
            fn into(self) -> String {
                self.0
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = crate::Error;
            fn from_str(s: &str) -> crate::Result<Self> {
                ulid::Ulid::from_string(s)
                    .map(Into::into)
                    .map_err(Into::into)
            }
        }

        impl From<ulid::Ulid> for $struct_name {
            fn from(value: ulid::Ulid) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $struct_name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S: serde::ser::Serializer>(
                &self,
                serializer: S,
            ) -> core::result::Result<S::Ok, S::Error> {
                self.to_string().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D: serde::de::Deserializer<'de>>(
                deserializer: D,
            ) -> core::result::Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                let id = ulid::Ulid::from_string(&s).map_err(serde::de::Error::custom)?;
                Ok(id.into())
            }
        }
    };
}

def_id!(WorkspaceId);
def_id!(AgentId);
def_id!(GroupId);
def_id!(RoleId);
def_id!(RoleForGroupId);
def_id!(FileId);
def_id!(FaqSettingsId);
def_id!(FaqCategoryId);
def_id!(FaqItemId);
def_id!(InquirySettingsId);
def_id!(InquiryContactId);
def_id!(InquiryChannelId);
def_id!(InquiryThreadId);
def_id!(InquiryMessageId);
