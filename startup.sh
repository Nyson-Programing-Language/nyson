LOC="$(pwd)"
cd ~
git clone https://github.com/Nyelsonon/nyson-programming-language.git
cd nyson-programming-language
cargo build --release
cd $LOC
sudo cp ~/nyson-programming-language/target/release/nyson /usr/bin