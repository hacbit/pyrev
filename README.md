# Pyrev

Reverse tools for bytecode of python


## Build

```shell
/$path_to/pyrev> cargo build --release [--target your_target_os]
```


## Usage

```shell
/$path_to/pyrev> cargo run help
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\pyrev.exe help`
Usage: pyrev.exe [OPTIONS] --file <FILE> [name] [COMMAND]

Commands:
  test  run the example
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [name]  Optional name

Options:
  -f, --file <FILE>    specify bytecode files
  -o, --output <FILE>  set name of output file which contains the decompiled result
  -h, --help           Print help
  -V, --version        Print version
```


## Test

```shell
/$path_to/pyrev> ls test/
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---           2024/2/24    16:24            220 def.py
-a---           2024/2/24    16:24           4714 def.txt
-a---          2023/12/22    12:28            188 for.py
-a---          2023/12/22    12:28           1905 for.txt
-a---          2023/12/21    20:34            112 import.py
-a---          2023/12/21    20:35           1437 import.txt
-a---          2023/12/20    10:58            355 op.py
-a---          2023/12/20    10:58           7057 op.txt
```

**You can run like `pyrev --file/-f test/import.txt` and compare the result with `import.py` whom generate `import.txt`**

```shell
/$path_to/pyrev> pyrev --file ./test/for.txt
 1| arr = [1, 3, 5, 7, 9]
 2| for i, v in enumerate(arr):
 3|     line = i + 1
 4|     print(line, v)
```

## Example

**Run the command `cargo run [--release] --example run-all` to test all the test files in test/ dict**
