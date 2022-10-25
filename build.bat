@del *.exe
@del *.pdb
@rustc .\%1%.rs -o build.exe
@echo.
@echo ---output---
@echo.
@build.exe