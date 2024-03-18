# Pyrev v1.0.5

Reverse tools for bytecode of python


## Build

```shell
/path/to/pyrev> cargo build --release [--target your_target_os]
```

**You can use `build` script to build to your target OS and add it to your PATH.**

**For Windows:**
```cmd
PS D:\path\to\pyrev> .\build.bat
```

*If you use WSL, you can use the follow command in windows terminal to build for linux:*
```cmd
PS D:\path\to\pyrev> .\build.bat --wsl
```

**For Linux:**
```shell
/path/to/pyrev> ./build.sh
```


## Usage

```cmd
PS D:\path\to\pyrev> pyrev help
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

**You can not specify the `-f/--file` option, and the program will read the bytecode from stdin.**
```powershell
PS D:\path\to\pyrev> echo 'print("Hello, World!")' | python -m dis | pyrev
 1| print("Hello, World!")
```


## Test

```powershell
PS D:\path\to\pyrev> ls test/
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---            2024/3/4    17:46            122 attr.py
-a---            2024/3/4    17:46           1742 attr.txt
-a---           2024/3/13    20:27            330 class.py
-a---           2024/3/13    20:27           3874 class.txt
-a---            2024/3/4    23:11            172 container.py
-a---            2024/3/4    23:15           2096 container.txt
-a---           2024/3/14     0:49            254 def.py
-a---           2024/3/14     0:49           5271 def.txt
-a---            2024/3/9     1:35            307 except.py
-a---            2024/3/9     1:35           4395 except.txt
-a---          2023/12/22    12:28            188 for.py
-a---          2023/12/22    12:28           1905 for.txt
-a---           2024/3/10    14:04            119 import.py
-a---           2024/3/10    14:04           1398 import.txt
-a---            2024/3/2     1:10            376 op.py
-a---            2024/3/2     1:11           8564 op.txt
```

**You can run like `pyrev --file/-f test/import.txt` and compare the result with `import.py` whom generate `import.txt`**

```powershell
PS D:\path\to\pyrev> pyrev --file ./test/for.txt
 1| arr = [1, 3, 5, 7, 9]
 2| for i, v in enumerate(arr):
 3|     line = i + 1
 4|     print(line, v)
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