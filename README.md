# Purs

[![Cargo Build & Test](https://github.com/TimB87/purs/actions/workflows/rust-build.yml/badge.svg)](https://github.com/TimB87/purs/actions/workflows/rust-build.yml)
[![rust-clippy analyze](https://github.com/TimB87/purs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/TimB87/purs/actions/workflows/rust-clippy.yml)

A [Pure](https://github.com/sindresorhus/pure)-inspired prompt in [Rust](https://www.rust-lang.org/).
<img src="https://github.com/TimB87/purs/blob/master/static/imgs/prompt.png?raw=true" align="right" width="420" />

Even more minimal, definitively faster and at least as pretty as the original Pure by [Sindre Sohrus](https://github.com/sindresorhus).

## Installation — Usage

1. Set up your Rust environment (use a Nightly build)
1. `$ cargo build --release`
1. Add the following to your ZSH configuration:

```
function zle-line-init zle-keymap-select {
  PROMPT=`/PATH/TO/PURS/target/release/purs prompt -k "$KEYMAP" -r "$?" --venv "${${VIRTUAL_ENV:t}%-*}"`
  zle reset-prompt
}
zle -N zle-line-init
zle -N zle-keymap-select

autoload -Uz add-zsh-hook

function _prompt_purs_precmd() {
  /PATH/TO/PURS/target/release/purs precmd
}
add-zsh-hook precmd _prompt_purs_precmd
```

## Why?

1. Learn some Rust
2. My Pure prompt felt slow on large repos (and indeed, was, compared to Purs)
3. Learn some Rust

## Questions

* I don't like...?

It's a pet project with wide areas for optimization and enhancement.
I'm really open to discussions, PRs a plus.

* Why doesn't it have...?

It's a pet project with wide areas for optimization and enhancement.
I'm really open to discussions, PRs a plus.


# License

MIT, see LICENSE file.
