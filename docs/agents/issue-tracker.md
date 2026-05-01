# Issue Tracker: Codeberg / Forgejo

Issues and PRDs for this repo live as Forgejo issues. Use the `fj` CLI for all operations.

## Conventions

- **Create an issue**: `fj issue create "title" --body "..."`. Title is positional (no `--title` flag). Use a heredoc for multi-line bodies.
- **Read an issue**: `fj issue view <number>`, with subcommands:
  - `fj issue view <number> body` — title and body
  - `fj issue view <number> comments` — list all comments
  - `fj issue view <number> comment <id>` — a specific comment by numeric ID
- **List issues**: `fj issue search --state open` with optional `--labels <labels>` filter. No dedicated `list` subcommand.
- **Comment on an issue**: `fj issue comment <number> --body "..."`
- **Apply / remove labels**: `fj issue edit <number> labels --add "..." --rm "..."`
- **Close**: `fj issue close <number> --with-msg "closing note"`. The `--with-msg` flag takes an optional message value.

`fj` infers the repo from the local git remote with `--remote` / `-R`. For explicit repo: `--repo` /
`-r <owner>/<repo>`.

## When a Skill Says "Publish to the Issue Tracker"

Create a Forgejo issue.

## When a Skill Says "Fetch the Relevant Ticket"

Run `fj issue view <number>`.
