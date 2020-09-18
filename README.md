# devo

devo is simple version manager for deno

Just use `devo` instead of `deno`, everything works great

![demo](https://raw.githubusercontent.com/tokiedokie/devo/master/.github/images/demo.gif)

## Dependency

- bash
- unzip

## Installation and upgrade

```
curl -fsSL https://raw.githubusercontent.com/tokiedokie/devo/master/install.sh | sh
```

## Usage

devo consumes same commands as deno and install a specific version you want automatically

### cli args

you can specify deno version in cli args

```
devo v1.2.0 <same subcommands as deno>
```

### `.devo` file

write version in `.devo`

```
v1.0.0
```

run devo in cli

```
devo <same subcommands as deno>
```

If you don't provide any version, devo just runs current `deno` version

## TODO

- [ ] add `devo global` to switch deno binary
