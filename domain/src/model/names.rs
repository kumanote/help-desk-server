use unicode_segmentation::UnicodeSegmentation;

macro_rules! def_name {
    ($struct_name:ident, $min_length:expr, $max_length:expr) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $struct_name(String);

        impl $struct_name {
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
                let s = s.trim();
                let length = s.graphemes(true).count();
                if $min_length <= length && length <= $max_length {
                    Ok(Self(s.to_owned()))
                } else {
                    Err(crate::Error::InvalidFormat)
                }
            }
        }

        impl From<String> for $struct_name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    };
}

def_name!(WorkspaceName, 3, 50);
def_name!(AgentName, 3, 50);
def_name!(GroupName, 3, 50);
def_name!(RoleName, 3, 50);
def_name!(RoleForGroupName, 3, 50);
