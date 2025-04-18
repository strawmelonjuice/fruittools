# Fruittools

**A collection of tools, utilities and libraries by Mar [@strawmelonjuice](https://github.com/strawmelonjuice).**

_Fair warning: I just made this to expel my boredom.
Might not be of use at all :P_

## Tools

### Echo

A simple echo tool that echoes back the input.

### Bananen _**`AKA`**_ `banana`, `changelog`

Changelog generator/manager for your projects.

> **My use:**
>
> I often forget stuff, a tool and a
> funny, memorable name helped a lot!

### Pulp _**`AKA`**_ `run`

Runner helper, either loads config from `fruit-pulp.toml` or auto-pulp searches for `gleam.toml`, `cargo.toml`, `package.json` etc.

> **My use:**
>
> I use a zellij layout called `ide` for
> most code editing, zellij likes to pretend
> making 9000 layouts is easy peasy, but
> I'd like to keep it at just ONE.
> So I either gotta type the command
> (having nothing auto started in-layout)
> ... or... I create something that chooses
> the right tool for the right CWD I'm in
>
> I think I chose the latter

#### Auto-pulp

Auto-pulp supports what its code knows, and tbh I don't know a whole lot. Do PR if you know a nice one!

| Priority for `dev` 0 = lowest | Priority for `build` 0 = lowest | Priority for `watch` 0 = lowest | Language                      | Runs                                   | Manifest     |
| ----------------------------- | ------------------------------- | ------------------------------- | ----------------------------- | -------------------------------------- | ------------ |
| 6                             | 0                               | 2                               | Gleam (with Lustre dev tools) | `gleam run -m  lustre/dev start`       | `gleam.toml` |
| 3                             | 0                               | 1                               | Gleam                         | `gleam run`                            | `gleam.toml` |
| 2                             | 0                               | 3                               | Gleam                         | `watchexec gleam run`                  | `gleam.toml` |
| 0                             | 5                               | 0                               | Gleam (with Lustre dev tools) | `gleam run -m lustre/dev build`        | `gleam.toml` |
| 0                             | 3                               | 0                               | Gleam                         | `gleam build`                          | `gleam.toml` |
| 0                             | 6                               | 0                               | Gleam (with gleescript)       | `gleam build; gleam run -m gleescript` | `gleam.toml` |
| 4                             | 0                               | 6                               | Rust                          | `cargo watch -x run`                   | `cargo.toml` |

There shall be more.
