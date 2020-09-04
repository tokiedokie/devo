# devo

devo is simple version manager for deno

Just use `devo` instead of `deno`, everything works great

## Installation

```
curl -fsSL https://raw.githubusercontent.com/tokiedokie/devo/master/install.sh | sh
```

## Usage

write version in `.devo`

```
v1.0.0
```

devo consumes same commands as deno and install a specific version you want automatically

```
devo <same subcommands as deno>
```

If you don't provide any version, devo just runs current `deno` version
