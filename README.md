# Pyrev v1.1.0-alpha

A Python bytecode reversing tool built in pure Rust



**!!! Rust nightly is recommended**

**use this command to switch to the `nightly` (it may not compile, if it is `stable`)**

```powershell
rustup default nightly
```



## Build

```shell
/path/to/pyrev> cargo build --release [--target your_target_os]
```

**You can use `build` script to build release to your target OS and copy it to default path (`.cargo/bin/`).**

**For Windows:**

```cmd
PS D:\path\to\pyrev> .\build.bat
```

*If you use WSL, you can use the follow command in windows terminal (not in WSL) to build for linux:*

```cmd
PS D:\path\to\pyrev> .\build.bat --wsl
```

**For Linux:**

```shell
/path/to/pyrev> ./build.sh
```



**If you only want to build and test all tests and doc-test, you also type command `cargo make`**



## Usage

```cmd
PS D:\path\to\pyrev> pyrev help
A Python bytecode reverse engineering tool.

Usage: pyrev.exe [OPTIONS] [name] [COMMAND]

Commands:
  test  test by your given python code
  pyc   decompile pyc files
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [name]  Optional name

Options:
  -f, --file <FILE>    specify bytecode files
  -o, --output <FILE>  set name of output file which contains the decompiled result
  -h, --help           Print help
  -V, --version        Print version
```

**You can not specify the `-f/--file` option, and the program will read the bytecode from stdin.**
```powershell
PS D:\path\to\pyrev> echo 'a.b.c(); print()' | python -m dis | pyrev
Try to decompile [Temp file]
  1| a.b.c()
  2| print()
```

## Test

```powershell
PS D:\path\to\pyrev> ls test/

    Directory: D:\Rust-test\pyrev\test

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d----           2024/4/15    17:42                pyc_test
-a---           2024/4/13    13:41            980 async.py
-a---           2024/4/12     2:30          14174 async.txt
-a---            2024/3/4    17:46            122 attr.py
-a---           2024/4/12     2:30           1742 attr.txt
-a---           2024/4/13     0:14            112 branch.py
-a---            2024/4/6     0:55            768 class.py
-a---           2024/4/12     2:30           7118 class.txt
-a---            2024/4/3    14:45            185 container.py
-a---           2024/4/12     2:30           2200 container.txt
-a---            2024/4/6     0:12            264 def.py
-a---           2024/4/12     2:30           5411 def.txt
-a---           2024/3/27    18:25            414 demo.py
-a---           2024/4/12     2:30           3150 demo.txt
-a---            2024/4/3    14:44            320 except.py
-a---           2024/4/12     2:30           4392 except.txt
-a---            2024/4/3    14:41            121 for.py
-a---           2024/4/12     2:30           1803 for.txt
-a---            2024/4/3    14:42            117 import.py
-a---           2024/4/12     2:30           1399 import.txt
-a---            2024/3/2     1:10            376 op.py
-a---           2024/4/12     2:30           8564 op.txt
-a---           2024/3/19    16:04            119 with.py
-a---           2024/4/12     2:30           3116 with.txt
-a---           2024/3/18    18:16            108 yield.py
-a---           2024/4/12     2:30           2391 yield.txt
```

**You can run like `pyrev --file/-f test/import.txt` and compare the result with `import.py` whom generate `import.txt`**

```powershell
PS D:\path\to\pyrev> pyrev --file ./test/for.txt
[INFO] Try to decompile test/for.txt
  1| arr = [1, 3, 5, 7, 9]
  2| for (i, v) in enumerate(arr):
  3|     line = i + 1
  4|     print(line, v)
  5| print('end')
  6| print('Test')
```

**You can also run the `run-all` script to run all .txt files in test/ dict**

**Windows:**
```cmd
PS D:\path\to\pyrev> .\run-all.bat [--release]
```

**Linux:**
```shell
/path/to/pyrev> ./run-all.sh [--release]
```

**You can add your custom test file to test/ dict and run the `get-dis` script to generate the .txt file**
**Windows:**

```powershell
PS D:\path\to\pyrev> .\get-dis.bat
This script will delete all .txt files in test/
And get the new python bytecode from python files in test/ and save them as .txt files
Deleting test\attr.txt
Deleting test\class.txt
Deleting test\container.txt
Deleting test\def.txt
Deleting test\demo.txt
Deleting test\except.txt
Deleting test\for.txt
Deleting test\import.txt
Deleting test\op.txt
Deleting test\with.txt
Deleting test\yield.txt
Try run: python -m dis test\attr.py > attr.txt
Try run: python -m dis test\class.py > class.txt
Try run: python -m dis test\container.py > container.txt
Try run: python -m dis test\def.py > def.txt
Try run: python -m dis test\demo.py > demo.txt
Try run: python -m dis test\except.py > except.txt
Try run: python -m dis test\for.py > for.txt
Try run: python -m dis test\import.py > import.txt
Try run: python -m dis test\op.py > op.txt
Try run: python -m dis test\with.py > with.txt
Try run: python -m dis test\yield.py > yield.txt
Done!
```

**Linux:**
```shell
/path/to/pyrev> ./get-dis.sh
(output)
```



## Document

run the `update-doc` script to generate doc

You can double-click the index.html in doc/ to open in browser



## Todo List

-   [x] if-else (unsound)
-   [x] for
-   [ ] while
-   [x] async
-   [x] generator
-   [ ] assertion
-   [ ] try-except
-   [x] doc comment
-   [x] pyc decompile plugin (unsound)
-   [ ] pretty console output plugin
