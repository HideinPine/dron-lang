SHELL := powershell.exe
.SHELLFLAGS := -NoProfile -Command

C=cargo
LINE_NUM := $(SH)
all: rs
rs:
	$(C) run

read:
	$name = Read-Host
	$(C) $(name)

line_num:
	cd src
	powershell.exe -NoProfile -Command "Get-ChildItem -Path "C:/Users/Admin/Projects/dron-lang/dronc/src" -Recurse -Filter "*.rs" | Select-String -Pattern "SearchTerm" | Select-Object LineNumber, Filename, Line" 
	cd ..
rm:
	$(C) clean
