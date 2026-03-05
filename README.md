# psg-cli

`psg-cli` — это CLI-инструмент на Rust для генерации файлов и папок из описания структуры. Он поддерживает несколько форматов входных файлов (JSON, TOML, YAML, текстовое дерево) и встроенные шаблоны для быстрого создания проектов на популярных языках.

## Возможности

- Генерация структуры из файлов `.json`, `.toml`, `.yaml`/`.yml`, `.txt` (дерево).
- Создание проектов по шаблонам: `rust`, `node`, `python`, `go`, `generic`.
- Интерактивный режим с запросом на перезапись существующих файлов.
- Принудительная перезапись (`--force`).
- Кроссплатформенность.

## Содержание

- [Установка](#установка)
- [Команды и опции](#команды-и-опции)
    - [`generate`](#generate)
    - [`init`](#init)
    - [`list-templates`](#list-templates)
- [Форматы описания структуры](#форматы-описания-структуры)
    - [JSON](#json)
    - [TOML](#toml)
    - [YAML](#yaml)
    - [TXT (дерево)](#txt-дерево)
- [Встроенные шаблоны](#встроенные-шаблоны)
    - [Rust](#rust)
    - [Node.js](#nodejs)
    - [Python](#python)
    - [Go](#go)
    - [Generic](#generic)
- [Примеры использования](#примеры-использования)
    - [Генерация из JSON](#генерация-из-json)
    - [Генерация из TOML](#генерация-из-toml)
    - [Генерация из YAML](#генерация-из-yaml)
    - [Генерация из текстового дерева](#генерация-из-текстового-дерева)
    - [Инициализация проекта](#инициализация-проекта)
- [Примечания](#примечания)
- [Лицензия](#лицензия)

## Установка

### Из исходного кода

```bash
git clone https://github.com/yourname/psg-cli.git
cd psg-cli
cargo build --release
sudo cp target/release/psg-cli /usr/local/bin/
cargo install psg-cli
```

### Альтернативный способ – использовать `cargo install --path .` прямо из папки проекта:
```bash
cargo install --path .
```
Эта команда соберёт проект и поместит бинарник в ~/.cargo/bin/ (обычно этот каталог уже добавлен в PATH после установки Rust).

После этого вы сможете вызывать программу:
```bash
psg-cli --help
```

## Команды и опции
### `generate`
Создаёт файловую структуру на основе файла описания.
```bash
gen-cli generate <ФАЙЛ> [опции]
```
#### Аргументы:
- `<ФАЙЛ>` — путь к файлу с описанием (поддерживаются расширения
- `.json`, `.toml`, `.yaml`, `.yml`, `.txt`).
#### Опции:

| Короткая      | Длинная          | Описание
|---------------|------------------|---------
| `-o` <DIR>    | `--output` <DIR> | Целевая директория (по умолчанию текущая).
| `-f`          | `--force`        | Принудительно перезаписывать все существующие файлы.
| `-i`          | `--interactive`  | Спрашивать подтверждение перед перезаписью каждого файла.

### `init`
Создаёт проект по одному из встроенных шаблонов.
```bash
gen-cli init <ШАБЛОН> [опции]
```
#### Аргументы:
- <ШАБЛОН> — имя шаблона: `rust`, `node`, `python`, `go`, `generic`.
#### Опции:
| Короткая   | Длинная          | Описание |
|------------|------------------|----------|
| `-o` <DIR> | `--output` <DIR> | Целевая директория (по умолчанию текущая).

### `list-templates`
Выводит список доступных шаблонов.
```bash
gen-cli list-templates
```

## Форматы описания структуры
### JSON
Корневой элемент — объект. Ключи — имена файлов или папок. Значения могут быть:
- строкой (содержимое файла);
- null (пустой файл);
- объектом (подпапка).
```json
{
  "README.md": "# My Project",
  "src": {
    "main.rs": "fn main() {\n    println!(\"Hello\");\n}",
    "lib.rs": null
  },
  "Cargo.toml": "[package]\nname = \"myapp\"\nversion = \"0.1.0\""
}
```

### TOML
Аналогично JSON. Многострочное содержимое задаётся в тройных кавычках.
```toml
"README.md" = "# My Project"

"src/main.rs" = '''
fn main() {
    println!("Hello");
}
'''

"src/lib.rs" = ""

"Cargo.toml" = '''
[package]
name = "myapp"
version = "0.1.0"
'''
```

### YAML
```yaml
README.md: "# My Project"
src:
  main.rs: |
    fn main() {
        println!("Hello");
    }
  lib.rs: ~
Cargo.toml: |
  [package]
  name = "myapp"
  version = "0.1.0"
```

### TXT (дерево)
Отступы 4 пробела. Символы псевдографики игнорируются. Если имя заканчивается на `/` — это папка.
```text
src/
    main.rs
    lib.rs
README.md
Cargo.toml
empty_dir/
```

## Встроенные шаблоны
### Rust
```toml
"Cargo.toml" = '''
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"

[dependencies]
'''

"src/main.rs" = '''
fn main() {
    println!("Hello, world!");
}
'''

"src/lib.rs" = '''
//! My Rust library

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
'''

"README.md" = '''
# My Rust Application

Generated with `gen-cli`.
'''

".gitignore" = '''
/target
Cargo.lock
*.rs.bk
'''
```

### Node.js
```toml
"package.json" = '''
{
  "name": "myapp",
  "version": "1.0.0",
  "description": "A Node.js project generated with gen-cli",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {}
}
'''

"index.js" = '''
console.log("Hello, Node.js!");
'''

"README.md" = '''
# My Node.js App

Run with `npm start`.
'''

".gitignore" = '''
node_modules/
.env
'''
```

### Python
```toml
"setup.py" = '''
from setuptools import setup, find_packages

setup(
    name="myapp",
    version="0.1",
    packages=find_packages(),
    install_requires=[],
    entry_points={
        "console_scripts": [
            "myapp=myapp.main:main",
        ],
    },
)
'''

"myapp/__init__.py" = '''"""My app package."""'''

"myapp/main.py" = '''
def main():
    print("Hello, Python!")

if __name__ == "__main__":
    main()
'''

"README.md" = '''
# My Python App

Install with: `pip install -e .`
'''

".gitignore" = '''
__pycache__/
*.pyc
*.pyo
*.egg-info/
dist/
build/
'''
```

### Go
```toml
"go.mod" = '''
module myapp

go 1.21
'''

"main.go" = '''
package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
'''

"README.md" = '''
# My Go App

Run with: `go run main.go`
'''

".gitignore" = '''
myapp
*.exe
'''
```

### Generic
```toml
"README.md" = '''
# Generic Project

Created with gen-cli.
'''

".gitignore" = '''
# Игнорируемые файлы
.DS_Store
*.log
'''

"src/" = {}   # пустая папка
"docs/" = {}  # пустая папка
```

## Примеры использования
### Генерация из JSON
```bash
gen-cli generate structure.json -o ./c_project
```

### Генерация из TOML (интерактивно)
```bash
gen-cli generate project.toml --interactive
```

### Генерация из YAML
```bash
gen-cli generate structure.yaml
```

### Генерация из текстового дерева
```bash
gen-cli generate tree.txt -o ./my_project
```

### Инициализация проекта
```bash
gen-cli init rust -o ./hello
gen-cli init node
gen-cli init python -o ./my_python_app
gen-cli init go
gen-cli init generic
gen-cli list-templates
```

## Примечания
- При генерации, если файл уже существует, поведение определяется
опциями --force или --interactive. Иначе существующие файлы пропускаются.
- Имена файлов и папок могут содержать любые символы, кроме запрещённых ОС.
- В текстовом дереве отступ должен быть строго 4 пробела.
- Шаблоны встроены в исполняемый файл.

## Лицензия
```text
MIT
```
