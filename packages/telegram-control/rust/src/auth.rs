use std::collections::HashSet;
use crate::error::{Error, Result};

/// Access control for bot users
#[derive(Clone, Debug, Default)]
pub struct AccessControl {
    whitelist: Option<HashSet<i64>>,
    admins: HashSet<i64>,
}

impl AccessControl {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Restrict access to specific user IDs
    pub fn with_whitelist(mut self, ids: Vec<i64>) -> Self {
        self.whitelist = Some(ids.into_iter().collect());
        self
    }
    
    /// Set admin users
    pub fn with_admins(mut self, ids: Vec<i64>) -> Self {
        self.admins = ids.into_iter().collect();
        self
    }
    
    /// Check if user is authorized
    pub fn is_authorized(&self, user_id: i64) -> bool {
        match &self.whitelist {
            Some(whitelist) => whitelist.contains(&user_id),
            None => true, // No whitelist = open access
        }
    }
    
    /// Check if user is an admin
    pub fn is_admin(&self, user_id: i64) -> bool {
        self.admins.contains(&user_id)
    }
    
    /// Authorize a user, returning error if not authorized
    pub fn authorize(&self, user_id: i64) -> Result<()> {
        if self.is_authorized(user_id) {
            Ok(())
        } else {
            Err(Error::Unauthorized(user_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_whitelist() {
        let auth = AccessControl::new().with_whitelist(vec![123, 456]);
        assert!(auth.is_authorized(123));
        assert!(auth.is_authorized(456));
        assert!(!auth.is_authorized(789));
    }
    
    #[test]
    fn test_no_whitelist() {
        let auth = AccessControl::new();
        assert!(auth.is_authorized(123));
        assert!(auth.is_authorized(999));
    }
    
    #[test]
    fn test_admins() {
        let auth = AccessControl::new().with_admins(vec![123]);
        assert!(auth.is_admin(123));
        assert!(!auth.is_admin(456));
    }
}
