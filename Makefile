.PHONY: test

all: test index.html

index.html: index-template.md sync-index.py $(wildcard src/*rs)
	markdown index-template.md | python sync-index.py > index.html

test:
	rustc src/ownership.rs -o src/ownership.exe && src/ownership.exe
	rustc --test src/structs.rs -o src/structs.exe && src/structs.exe
	rustc --test src/enums.rs -o src/enums.exe && src/enums.exe
	rustc --test src/options.rs -o src/options.exe && src/options.exe
	rustc src/threads.rs -o src/threads.exe && src/threads.exe
