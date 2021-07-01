set "curpath=%cd%"
cd C:\
git clone https://github.com/Nyelsonon/nyson-programming-language.git
cd nyson-programming-language
cargo build --release
SET "PATH=%PATH%;C:\nyson-programming-language\target\release"
setx /M PATH "%PATH%;C:\nyson-programming-language\target\release"
cd %curpath%