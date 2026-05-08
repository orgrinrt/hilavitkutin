**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-extensions (host.rs + extension.rs + DESIGN paragraph)
**Source topics:** task #346 (PLUGIN-HOST-D1, plugin-host audit F4)

# Topic: shutdown observer hook on ExtensionHost

## Background

`Extension::Drop` runs the extension's `shutdown_fn` and silently discards the status:

```rust
// src/extension.rs:123-132
impl Drop for Extension {
    fn drop(&mut self) {
        if let Some(shutdown) = self.descriptor.shutdown_fn {
            // SAFETY: shutdown is declared by the extension; host_ctx
            // is the pointer it received at init time.
            let _ = unsafe { shutdown(self.host_ctx) };
        }
        // Library's own Drop runs after this returns.
    }
}
```

In a host that loads several extensions and unwinds early because of an error in the prior frame, Drop is the shutdown path that fires for every extension that has not been explicitly closed. The status from each extension's shutdown is currently invisible. Loimu and viola need to observe these failures (log them, attribute them, decide whether the host process is in a recoverable state) but cannot do so today without injecting a global. The plugin-host audit (2026-05-04) called this out as F4.

`close()` is the explicit shutdown path and returns the error directly to the caller, so its observability story is fine. The gap is the Drop path.

## Proposed shape

Add a host-supplied observer fn pointer (optional) that fires from both `close()` and `Drop`. Symmetric: a consumer that wants to log all shutdowns installs one observer and gets notified on either path. The audit asked for Drop only; making it symmetric costs nothing extra and is the more useful idiom.

`mock/crates/hilavitkutin-extensions/src/host.rs`:

```rust
/// Observer signature for extension shutdown completion.
///
/// Receives the extension's declared name and the status returned
/// from the extension's `shutdown_fn` (or `ExtensionAbiStatus::Ok`
/// if the extension declared no shutdown). Fires once per extension
/// from either `Extension::close()` or the `Drop` path.
pub type ShutdownObserverFn = fn(name: &[u8], status: ExtensionAbiStatus);

pub struct ExtensionHost {
    host_capabilities: &'static [CapabilityId],
    policy: FailurePolicyFn,
    observer: Maybe<ShutdownObserverFn>,  // NEW
}

impl ExtensionHost {
    pub fn new(host_capabilities: &'static [CapabilityId]) -> Self {
        Self {
            host_capabilities,
            policy: default_policy,
            observer: Maybe::Isnt,  // NEW
        }
    }

    pub fn with_policy(mut self, policy: FailurePolicyFn) -> Self {
        self.policy = policy;
        self
    }

    /// Override the shutdown observer.
    pub fn with_shutdown_observer(
        mut self,
        observer: ShutdownObserverFn,
    ) -> Self {
        self.observer = Maybe::Is(observer);
        self
    }
    // ... rest unchanged
}
```

`Extension` carries the observer as a copy at load time so its Drop path does not need to reach back to the host (which would require a borrow lifetime, breaking existing API):

```rust
pub struct Extension {
    library: Library,
    descriptor: &'static ExtensionDescriptor,
    host_ctx: *mut c_void,
    observer: Maybe<ShutdownObserverFn>,  // NEW
}

impl Extension {
    pub(crate) fn from_parts(
        library: Library,
        descriptor: &'static ExtensionDescriptor,
        host_ctx: *mut c_void,
        observer: Maybe<ShutdownObserverFn>,  // NEW
    ) -> Self { ... }
}

impl Drop for Extension {
    fn drop(&mut self) {
        let status = if let Some(shutdown) = self.descriptor.shutdown_fn {
            unsafe { shutdown(self.host_ctx) }
        } else {
            ExtensionAbiStatus::Ok
        };
        if let Maybe::Is(observer) = self.observer {
            observer(self.name_bytes(), status);
        }
    }
}
```

`close()` fires the observer too, then returns the error if any. The status passed to the observer is the same status the close path inspects.

`ExtensionHost::load` threads the observer into `Extension::from_parts`.

## Why a `fn` pointer, not a closure or trait

A `fn` pointer is the simplest no_alloc shape. Closures with captures need either heap allocation or trait-object-like indirection that we forbid. A `fn` pointer requires the consumer to keep their state in static memory or pass it through opaque means — the same constraint already applied to `FailurePolicyFn`, so this is consistent.

## What stays untouched

- `ExtensionHost::new` signature unchanged. The observer defaults to `Maybe::Isnt`.
- `FailurePolicyFn` and `default_policy`. No changes.
- `Extension::descriptor`, `capability`, `capabilities`, `name`, `version` accessors. Unchanged.
- The `ExtensionRequirement` and `PolicyVerdict` enums. Unchanged.
- Public `Extension::close` signature. Still returns `Outcome<(), ExtensionError>`. The observer fires inside it before the error propagates.
- `Extension::from_parts` is `pub(crate)`, so the additional parameter is not a breaking change to the public API.

## Decision

Adopt as proposed. New optional builder method on `ExtensionHost`, new `pub type` for the observer, threaded into `Extension` for Drop-path observability. Symmetric: fires from both `close()` and `Drop`. No ABI break.
