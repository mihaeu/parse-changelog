// This file is @generated by parse-changelog-internal-codegen.
// It is not intended for manual editing.

#![allow(clippy::wildcard_imports)]

use crate::*;
const _: fn() = || {
    fn assert_send<T: ?Sized + Send>() {}
    fn assert_sync<T: ?Sized + Sync>() {}
    fn assert_unpin<T: ?Sized + Unpin>() {}
    assert_send::<error::Error>();
    assert_sync::<error::Error>();
    assert_unpin::<error::Error>();
    assert_send::<Changelog<'_>>();
    assert_sync::<Changelog<'_>>();
    assert_unpin::<Changelog<'_>>();
    assert_send::<Release<'_>>();
    assert_sync::<Release<'_>>();
    assert_unpin::<Release<'_>>();
    assert_send::<Parser>();
    assert_sync::<Parser>();
    assert_unpin::<Parser>();
};
