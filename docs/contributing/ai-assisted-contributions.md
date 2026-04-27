# AI-Assisted Contributions

AI assistance is fine for this project when the resulting changes stay accurate, minimal, and easy to review.

## Expectations

- Verify generated code against the current crate APIs before committing it.
- Keep edits focused on the requested behavior.
- Run the standard formatting and lint checks after changes.
- Avoid introducing unreviewed abstractions when a direct implementation is clearer.

## Good Uses

- Boilerplate reduction
- Documentation drafts
- Small refactors that preserve behavior
- Test scaffolding

## Bad Uses

- Rewriting the project architecture without a reason
- Guessing at Hyprland IPC details
- Leaving behind placeholder code or unused helpers
