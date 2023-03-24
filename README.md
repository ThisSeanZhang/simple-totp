A very simple command line TOTP manager, under your control

# Usage
```shell
# get help
totp --help

# add new 
totp add --taget test --secret-key JBSWY3DPEHPK3PXP

# list all target
totp list

# totp show the code
totp show --id 1

```

# Developer Notice 

## init database
```shell
diesel setup
```

## add new table
```shell
# Gen a new table
diesel migration generate create_totp_keys

# apply new migration
diesel migration run

# execute after alter migration
diesel migration redo

# if you want restore
diesel migration revert
```

## build
```shell
# window
cargo build --target x86_64-pc-windows-gnu

# linux 
cargo build
```

## Resource
* [diesel getting started](https://diesel.rs/guides/getting-started.html)
* [TOTP Token Generator, help you verify](https://totp.danhersam.com/)
