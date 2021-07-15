set "curpath=%cd%"
cd C:\Users\%username%
git clone https://github.com/Nyelsonon/nyson-programming-language.git
cd nyson-programming-language
cargo build --release
SET "PATH=%PATH%;C:\Users\%username%\nyson-programming-language\target\release"
setx /M PATH "%PATH%;C:\Users\%username%\nyson-programming-language\target\release"
cd %curpath%
