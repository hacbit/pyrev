@echo off
setlocal

echo This script will delete all .txt files in test/
echo And get the new python bytecode from python files in test/ and save them as .txt files

for %%f in (test\*.txt) do (
    echo Deleting %%f
    del %%f
)

for %%f in (test\*.py) do (
    echo Try run: python -m dis %%f ^> %%~nf.txt
    python -m dis %%f > test\%%~nf.txt
)

echo Done!

endlocal