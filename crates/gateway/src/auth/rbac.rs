use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Owner,
    Admin,
    Engineer,
    Viewer,
}

#[allow(dead_code)]
impl UserRole {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "owner" => UserRole::Owner,
            "admin" => UserRole::Admin,
            "engineer" => UserRole::Engineer,
            _ => UserRole::Viewer,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UserRole::Owner => "owner",
            UserRole::Admin => "admin",
            UserRole::Engineer => "engineer",
            UserRole::Viewer => "viewer",
        }
    }

    pub fn permits(&self, required: UserRole) -> bool {
        match (self, required) {
            (UserRole::Owner, _) => true,
            (UserRole::Admin, UserRole::Admin | UserRole::Engineer | UserRole::Viewer) => true,
            (UserRole::Engineer, UserRole::Engineer | UserRole::Viewer) => true,
            (UserRole::Viewer, UserRole::Viewer) => true,
            _ => false,
        }
    }
}
