# counter(1)

A simple command line tool to write and modify number counters to a textfile.  
The tool is designed to be used in combination with OBS and a streamdeck, for example, to simply modify counters while streaming.

For example, create a binding in your streamdeck which executes following command:
```
$ counter --file /var/streamdata/mycounter --key "zekro's Fails" 1
```
This adds `+1` to the counter `"zekro's Fails"`. You can also add a binding to substract a value:
```
$ counter --file /var/streamdata/mycounter --key "zekro's Fails" --sub 1
```

## Usage

```
counter 1.0.0
Counts stuff, I guess.

USAGE:
    counter.exe [FLAGS] [OPTIONS] --file <file> [AMMOUNT]

FLAGS:
    -h, --help       Prints help information
        --set        Set value
    -s, --sub        Substract value
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    The counter file location
    -k, --key <key>      The value key to be modified

ARGS:
    <AMMOUNT>    The ammount to be added, substracted or set
```

---

© 2021 Ringo Hoffmann (zekro Development)  
Covered by the MIT License.