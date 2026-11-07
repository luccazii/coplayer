# Coplayer

Read `README.md` and the active subphase in `docs/implementation-plan.md` before changing code. Product rules live in `docs/product-decisions.md`, acceptance scenarios in `docs/acceptance-criteria.md`, and terminology in `CONTEXT.md`.

- Keep code, identifiers, and commit messages in English. Project documentation and initial UI are in Brazilian Portuguese; bilingual product acceptance is scheduled separately.
- Keep the UI in Vue/TypeScript and business rules in Rust. Platform integrations and provider adapters must remain separate from domain rules. Do not introduce a distributed Node/Python backend.
- Work within the requested subphase. Do not present a scaffold, simulated provider, browser preview, or successful build as evidence of subscription access, capture, overlay, or Gaming Mode support.
- Preserve unrelated changes and the approved specifications. Record decisions and evidence in the implementation plan; mark a subphase complete only after its acceptance checks pass.
- Use npm and commit both package lockfiles when dependencies change. The Rust toolchain is pinned in `rust-toolchain.toml`.
- Run `npm run build`, `npm run check:rust`, and the relevant tests. Run `npm run build:desktop` for desktop/bootstrap changes. Inspect the native app when changing its integration; browser rendering alone does not validate Tauri IPC.
- Never commit credentials, runtime authentication directories, screenshots of real campaigns, or local user data. Use ignored local paths for temporary validation artifacts.

The project currently has no gameplay, capture, database, or provider implementation. Do not add empty abstraction layers or mock product features to imply otherwise.
