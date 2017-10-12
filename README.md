Install python dependencies:
```
$ pip install -r requirements.txt
```

Generate 4-letter names starting with `r` and ending with `o`
```
$ cargo run --release -- -s n -e on -c 5
necon
nelon
nicon
nikon
nilon
Generated: 5
```

Check DNS records and Google search results count
```
$ ./check.py < check.txt
=============================
   google
-----------------------------
:: google.com
google.com. 299 IN A 216.58.206.174
:: google.io
google.io. 299 IN A 172.217.17.196
:: google os
497000 results
:: google application
228000 results
:: google server
450000 results
...
```

```
$ cargo run --release -- --help
companame 0.1.0

USAGE:
    companame [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <count>        Character count [default: 3]
    -e <end>          End sequence [default: ]
    -s <start>        Start sequence [default: ]
```
