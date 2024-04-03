@echo off
setlocal enabledelayedexpansion

echo Updating documentation...
set RUNNER=cargo doc --no-deps ^
    -p pyrev ^
    -p pyrev_ast ^
    -p pyrev_ast_derive

echo Try run: %RUNNER%
!RUNNER!

echo Delete old docs
echo Try run: rd /S /Q .\doc\
rd /S /Q .\doc\

echo Copy docs from target/ to crate root
echo Try run: xcopy /E /I .\target\doc\ .\doc\
xcopy /E /I .\target\doc\ .\doc\

endlocal