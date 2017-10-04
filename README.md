# companame

```
$ pip install -r requirements.txt

# Generate 4-letter names starting with `r` and ending with `o`
$ ./gen.py -s r -e o -c 4
raco
ralo
ramo
rano
rato
reco
relo
remo
reno
reto
reyo
rico
rilo
rimo
rino
rito
roco
rolo
romo
rono
roto

# Check DNS records and Google search results count
$ ./check.py < check.txt
=============================
   google
-----------------------------
:: google.com
google.com. 299 IN A 216.58.206.174
:: google.io
google.io. 299 IN A 172.217.17.196
...
```

```
usage: gen.py [-h] [-c C] [-s S] [-e E]

optional arguments:
  -h, --help  show this help message and exit
  -c C        character count (default: 5)
  -s S        start sequence
  -e E        end sequence
```
