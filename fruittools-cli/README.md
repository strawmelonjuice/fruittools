# fruittools_cli

[![Package Version](https://img.shields.io/hexpm/v/fruittools_cli)](https://hex.pm/packages/fruittools_cli)
[![Hex Docs](https://img.shields.io/badge/hex-docs-ffaff3)](https://hexdocs.pm/fruittools_cli/)

## What it does

This is the cli to fruittools, running in bun calling the deamon process, which is responsible for managing the fruittools services.
If the deamon process is not running, the cli will start it.

The cli should be used with mostly aliases, such as bananen (the changelog manager), or pulp (the watcher/runner).
When used on it's own, likely you want to get some logs.

```sh
$> bananen add fix "Fixed all the things" --breaking
# # Is an alias for
# $> fruittools tool bananen add fix "Fixed all the things" --breaking

$> fruittools logs
# -> [bananen] fix: 'Fixed all the things' --> unreleased@"~/project1/changelog.md"
# -> [fs] wrote to `bananen.json`
# -> [fs] wrote to `changelog.md`

# # The logs are stored by the deamon process, which is responsible for managing the fruittools services.
# # but not only for the current directory, but also for the entire system.
$> fruittools logs --global --follow
# -> < ~/project1/ > [bananen] fix: 'Fixed all the things' --> unreleased@"~/project1/changelog.md"
# -> < ~/project1/ > [fs] bananen wrote to `bananen.json`
# -> < ~/project1/ > [fs] bananen wrote to `changelog.md`
# -> < ~/project2/ > [pulp] watchexec started command `cargo check`
```
