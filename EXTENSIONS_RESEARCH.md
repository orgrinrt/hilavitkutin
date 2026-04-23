# Hilavitkutin Extensions & Plugins: Research & Analysis

This document synthesizes research from across the workspace (`loimu`, `saalis`, `polka-dots`) regarding dynamic extension and plugin architectures. It analyzes the design decisions made in each project, identifies strengths and pitfalls, and establishes the architectural baseline for `hilavitkutin-extensions` and `hilavitkutin-plugins`.

## 1. Context & Prior Art

### 1.1 `loimu` (The Reference Architecture)
*References: `~/Dev/loimu/mock/crates/loimu-module/DESIGN.md.tmpl`, `loimu-plugin`, `loimu-extern`*

`loimu` establishes a strict, dual-layered conceptual split:
*   **Modules (`loimu-module`)**: Native Rust shared libraries (`.dylib`/`.so`). Full trust, zero-overhead vtable dispatch, loaded via `dlopen`. Used by framework developers to structure the internal architecture (e.g., `loimu-gui`, `loimu-input`).
*   **Plugins (`loimu-plugin`)**: WASM modules. Sandboxed, fuel-metered, dispatched via host functions. Used for end-user extensibility.
*   **Shared ABI (`loimu-extern`)**: Both approaches use a shared `#[repr(C)]` ABI surface. The library exports explicit symbols (`__loimu_abi_version`, `__loimu_manifest`, `__loimu_init`, `__loimu_shutdown`).
*   **Loading Lifecycle**: `dlopen` -> read version symbol (reject on mismatch) -> read manifest symbol -> validate permissions/risk -> call `init` symbol.

### 1.2 `saalis`
*References: `~/Dev/saalis/mock/crates/saalis-server/DESIGN.md.tmpl`, `saalis-sdk`*

`saalis` builds a headless server that dynamically loads subsystems at runtime:
*   **Extensions**: Compiled as `cdylib`.
*   **Loading**: Uses raw `dlopen` / `dlsym` / `dlclose` (strictly `no_std`).
*   **Discovery**: Uses the `inventory` crate to automatically collect descriptors (Connectors, Cataloguers, Enrichers, etc.) upon loading.
*   **Validation**: Checks `SdkVersion` on each collected descriptor and performs DAG cycle detection during the bootstrap phase.

### 1.3 `polka-dots`
*References: `~/Dev/polka-dots/.github/copilot-instructions.md`, `polka-sdk`*

`polka-dots` is a plugin-extensible dotfiles build engine:
*   **Extensions**: First-party plugins (`polka-lang-polka`, `polka-source-brew`, platform adapters) are compiled as `cdylib`.
*   **SDK**: The SDK crate exports only traits, `repr(C)` types, and proc macros. No concrete implementations exist in the SDK.
*   **Discovery**: Plugins register via a `#[register]` proc macro on trait impl blocks, and the `inventory` crate handles the discovery across the FFI boundary.

---

## 2. Comparative Analysis: Strengths and Pitfalls

### 2.1 The Good: Explicit ABI & Lifecycle (`loimu`)
`loimu`'s approach of defining explicit `#[repr(C)]` structs (`ExternManifest`) and exporting well-known C-ABI functions is incredibly robust. It completely avoids Rust ABI instability by forcing the boundary to be C-compatible. Furthermore, the explicit separation of "Modules" (native, trusted) and "Plugins" (sandboxed, untrusted) is conceptually clean and prevents architectural misuse. 

### 2.2 The Bad: `inventory` Across FFI Boundaries (`saalis` & `polka-dots`)
Both `saalis` and `polka-dots` heavily rely on the `inventory` crate for plugin discovery within dynamically loaded libraries (`cdylib`). 

**The Pitfall**: The `inventory` crate relies on linker sections (`.init_array` on Linux, `.CRT$XCU` on Windows, `__DATA,__mod_init_func` on macOS) to execute code before `main`. When a `.dylib`/`.so` is loaded dynamically at runtime via `dlopen`, these linker sections are not always reliably executed or merged with the host's global registry. This fragility is highly dependent on the OS, the dynamic linker implementation, and the Rust compiler version. It often leads to silent failures where plugins successfully `dlopen` but register zero descriptors in the host process.
*   **Verdict**: Relying on global static linker-magic for dynamic plugin discovery is an anti-pattern for reliable plugin ecosystems.

### 2.3 The Ugly: Tight Coupling to the Domain
In all three prior projects, the extension loaders inherently know about the domain concepts. `loimu` knows about "Behaviors" and "Resources"; `saalis` knows about "Connectors" and "WorkUnits". If the loader knows what a domain object is, the loader cannot be reused across different projects. `hilavitkutin` needs to be completely agnostic to serve downstream consumers like `viola-core`.

### 2.4 The Insight: Generics Across the Boundary (`saalis`)
A common misconception is that generics cannot cross a dynamic boundary. `saalis` proved this false. In its architecture, generic traits (e.g., `Cataloguer<E>`) are successfully passed across the boundary with zero `dyn Trait` overhead.
*   **The Mechanism**: Macro-driven static monomorphization. The `#[register]` macro explicitly instantiates the monomorphized generic methods *at compile time inside the plugin* and generates a `repr(C)` descriptor containing `extern "C"` function pointers to those specific instances. The host receives the descriptor and wraps the C pointers back into a generic Rust interface. This yields perfect Rust ergonomics for the plugin author while maintaining a stable C ABI.

### 2.5 The Benchmark Reality: Optimization Barriers (`polka-dots`)
The `polka-dots` benchmarks (`bench-framework-v2-design.md`) revealed critical details about LLVM optimization across boundaries. They found that if code is compiled together (or via Rust `dylib` with shared LTO), LLVM aggressively devirtualizes and monomorphizes, creating unfair performance advantages and tight coupling.
*   **The Finding**: Compiling plugins strictly as `cdylib` and calling them via `dlsym` function pointers creates a hard optimization barrier. LLVM has zero visibility across the `cdylib` boundary. This is highly desirable for a plugin system: it ensures stable, predictable execution without compiler-version coupling or unfair host-plugin optimization bleed.

---

## 3. Architectural Directives for `hilavitkutin`

Based on this research, `hilavitkutin` will adopt the `loimu` conceptual separation pattern but improve upon the fragile discovery mechanisms seen in `saalis` and `polka-dots`, leverage macro-driven monomorphization for ergonomics, and enforce a strict `cdylib` optimization barrier, while remaining strictly domain-agnostic.

### 3.1 Abstraction Boundary: `hilavitkutin-extensions`
*   **Responsibility**: The pure binary loading unit.
*   **Mechanism**: Wraps platform-specific loading (e.g., `libloading` or raw `dlopen`/`LoadLibrary`).
*   **Rule**: **No linker magic.** Do not use `inventory` or static global registration across the boundary.
*   **Design**: It must only export a safe API to load a library from disk and extract typed `Symbol<T>`. It handles OS-specific library extensions (`.so`, `.dylib`, `.dll`) and provides a structured, reusable error model for load/link failures. It does not know what a "plugin" is.

### 3.2 Abstraction Boundary: `hilavitkutin-plugins`
*   **Responsibility**: Contract-bound functionality hosting and orchestration.
*   **Discovery**: Instead of `inventory`, plugins must export a single, well-known C-ABI entry point function (e.g., `__hilavitkutin_plugin_descriptor() -> *const PluginDescriptor`). The host calls this function immediately after loading to receive the manifest, version info, and capability registry. This matches the `loimu` explicit-symbol approach.
*   **Lifecycle**: Orchestrates `Initialize` -> `Invoke` -> `Shutdown` in a deterministic order.
*   **Policy**: Handles host-side execution policies (e.g., `fail-closed` for missing required plugins, graceful degradation for optional ones, capability verification).

### 3.3 What is left for the Consumer (e.g., `viola-core`)?
*   **Domain Contracts**: The exact shape of the configuration, the `Normalized Analysis Model` (NAM), the AST, and the specific lint rules are strictly consumer-owned.
*   **Macro SDKs**: The consumer must provide its own contract crate (e.g., `viola-plugin-abi`) and a procedural macro (e.g., `#[export_plugin]`). The macro is responsible for monomorphizing the user's generic Rust code into the `repr(C)` function pointers required by the ABI.
*   **Separation of Concerns**: `hilavitkutin-plugins` only provides the generic orchestration frame (the "how"), not the specific domain types being orchestrated or the macros that generate the C bindings (the "what").

## 4. Conclusion

The `loimu` "Modules vs Plugins" separation provides the correct mental model, but for `hilavitkutin`, we will formalize it as "Extensions (Binary Mechanics) vs Plugins (Contract Orchestration)". 

By combining `loimu`'s explicit C-ABI symbol exports to fix discovery, `saalis`'s macro-driven monomorphization for perfect Rust ergonomics, and `polka-dots`'s strict `cdylib` boundaries for compiler isolation, `hilavitkutin` will provide a perfectly stable, ergonomic, and reusable plugin foundation.