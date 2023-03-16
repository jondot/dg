# dg - find dirty git repos 

Ever forgot to push a commit or lost your work because you assumed it was pushed to Github but it wasn't?

`dg` finds local git repos with pending changes, un-added files, branches that are completely new, and branches that were not pushed upstream.

Run it before you go on holiday 🎃, or every month just to check that you're not forgetting anything 📅.

```
$ dg -b
┌───────────────────────┬──────────────┬────────┬──────────────────────┐
│ repository            │ changes      │ ahead  │ missing              │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./logolang.org        │              │        │ master               │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./init-tslib          │              │        │ master               │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./sqlx-crud           │ •••••••••••• │        │                      │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./interactive-actions │              │        │ master               │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./sign-rs             │              │        │ master               │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./teller-docs         │ •••          │        │ adding-styling-fwks, │
│                       │              │        │  master,             │
│                       │              │        │ switch-to-theme-ui   │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./init-rs             │              │        │ implement-eject,     │
│                       │              │        │ master, simplified   │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./foobar3             │ •••          │        │                      │
├───────────────────────┼──────────────┼────────┼──────────────────────┤
│ ./pattern_match       │ ••••••       │        │ master               │
└───────────────────────┴──────────────┴────────┴──────────────────────┘
```

For example - this discovered that I have local changes on `sqlx-crud`, and that `teller-docs` contains branches that I created locally but never pushed.

## Download

```
$ brew tap jondot/tap && brew install dg
```
Otherwise, grab a release from [releases](https://github.com/rusty-ferris-club/recon/releases).

## Usage

```
Usage: dg [-b] [-p <path>]

dg: find dirty local Git repos with pending changes or unpushed content

Options:
  -b, --branches    include analysis for local branches
  -p, --path        root path (default ".")
  --help            display usage information
  -V, --version     print version information and exit
```

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/jondot/dg/pulls).

To all [Contributors](https://github.com/jondot/dg/graphs/contributors) - you make this happen, thanks!

# License

Copyright (c) 2023 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
