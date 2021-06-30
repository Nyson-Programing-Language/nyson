## Dependencies
- Rust
- Git

## --------------------

## Self compile
```shell
git clone https://github.com/Nyelsonon/nyson-programming-language.git
cd nyson-programming-language
cargo build --release
cd target/release
```

it will be called Nylex

## Linux or Mac
```shell
curl "https://raw.githubusercontent.com/Nyelsonon/nyson-programming-language/main/startup.sh" | sh
```

## Windows
(have to run in cmd with administrator)
```shell
curl "https://raw.githubusercontent.com/Nyelsonon/nyson-programming-language/main/startup.bat" -o startup.bat; ./startup.bat; del ./startup.bat
```