//! Permission system

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    DbRead,
    DbWrite,
    FilesystemExport,
    NetworkLoopback,
    NetworkExternal,
    ClipboardRead,
    ClipboardWrite,
}

impl Permission {
    /// Parse permission from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "db_read" => Some(Permission::DbRead),
            "db_write" => Some(Permission::DbWrite),
            "filesystem_export" => Some(Permission::FilesystemExport),
            "network_loopback" => Some(Permission::NetworkLoopback),
            "network_external" => Some(Permission::NetworkExternal),
            "clipboard_read" => Some(Permission::ClipboardRead),
            "clipboard_write" => Some(Permission::ClipboardWrite),
            _ => None,
        }
    }

    /// Whether this permission requires explicit user approval
    pub fn requires_approval(&self) -> bool {
        matches!(
            self,
            Permission::DbWrite | Permission::NetworkExternal | Permission::ClipboardRead
        )
    }

    /// Risk level for UI display
    pub fn risk_level(&self) -> &'static str {
        match self {
            Permission::DbRead => "medium",
            Permission::DbWrite => "high",
            Permission::FilesystemExport => "low",
            Permission::NetworkLoopback => "low",
            Permission::NetworkExternal => "high",
            Permission::ClipboardRead => "medium",
            Permission::ClipboardWrite => "low",
        }
    }
}

/// Permission set for a plugin
#[derive(Debug, Clone, Default)]
pub struct PermissionSet {
    approved: HashSet<Permission>,
    pending: HashSet<Permission>,
    revoked: HashSet<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has(&self, permission: Permission) -> bool {
        self.approved.contains(&permission)
    }

    pub fn approve(&mut self, permission: Permission) {
        self.pending.remove(&permission);
        self.revoked.remove(&permission);
        self.approved.insert(permission);
    }

    pub fn revoke(&mut self, permission: Permission) {
        self.approved.remove(&permission);
        self.pending.remove(&permission);
        self.revoked.insert(permission);
    }

    pub fn request(&mut self, permission: Permission) {
        if !self.approved.contains(&permission) && !self.revoked.contains(&permission) {
            self.pending.insert(permission);
        }
    }
}
