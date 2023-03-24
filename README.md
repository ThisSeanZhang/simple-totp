A very simple command line TOTP manager, under your control

# Usage
```shell
# get help
totp --help

# Usage: totp [OPTIONS] <COMMAND>
# Commands:
#   add   Add Key
#   list  List ALL Key
#   show  Show Code
#   help  Print this message or the help of the given subcommand(s)

# Options:
#   -d, --data-path <DATA_PATH>  [env: TOTP_DATABASE_URL=./data.db]
#       --log-level <LogLevel>   [default: info]
#   -h, --help                   Print help

# add new 
# totp add <TAGET> <SECRET_KEY
totp add test JBSWY3DPEHPK3PXP

# list all target
totp list
#$ ./totp list
# [2023-03-24T08:47:42Z INFO  totp] record size: 1
#          ID          |        LABEL         |         KEY         
#          1           |         test         |         ***         
# $ ./totp list -s
# [2023-03-24T08:47:50Z INFO  totp] record size: 1
#          ID          |        LABEL         |         KEY         
#          1           |         test         |   JBSWY3DPEHPK3PXP  

totp show 1
# $ ./totp show 1
# [2023-03-24T08:49:03Z INFO  totp] args Args { data_path: Some("./data.db"), log_level: "info", action: Show { id: 1, digits: 6, step: 30 } }
# [2023-03-24T08:49:03Z INFO  totp] sqlite path "./data.db"
# label: "test", token is: 815739
# label: "test", token is: 002526

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
