use crate::{service::request::set_token, types::auth::UserInfo};
use std::{fmt, ops::Deref};
use yew::{UseStateHandle, use_context};

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_token(value.token.clone());
        self.inner.set(value);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_token(None);
        self.inner.set(UserInfo::default());
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

/// This hook is used to manage user context.
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();

    UseUserContextHandle { inner }
}
