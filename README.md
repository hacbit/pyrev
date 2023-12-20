# repybytecode
Reverse tools for bytecode of python



## Build

```shell
/$path_to/repybytecode> cargo build --release [--target your_target_os]
```



## Usage

```shell
/$path_to/repybytecode> cargo run help
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\repybytecode.exe help`
Usage: repybytecode.exe [OPTIONS] --file <FILE> [name] [COMMAND]

Commands:
  test  run the example
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [name]  Optional name

Options:
  -f, --file <FILE>    specify a bytecode file
  -o, --output <FILE>  specify an output file
  -h, --help           Print help
  -V, --version        Print version
```



## Example

```shell
/$path_to/repybytecode> ls test/
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---           2023/8/27    13:15          21763 Bytecode.txt
-a---          2023/12/19    23:43            139 import.py
-a---          2023/12/19    23:29           1723 import.txt
-a---          2023/12/20    10:58            355 op.py
-a---          2023/12/20    10:58           7057 op.txt
```

**you can run like `repybytecode --file/-f test/import.txt` and compare the result with `import.py` whom generate `import.txt`**

```shell
/$path_to/repybytecode> repybytecode --file ./test/import.txt
 1| import sys
 2| from pwn import *
 3| import PIL.Image as Image
 4| import numpy as np
 5| import matplotlib.pyplot as plt
 6| from os import system, popen
```

