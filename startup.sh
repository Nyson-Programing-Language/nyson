cargo build --release
DIR="$( cd "$( dirname "$0" )" && pwd )"
sudo cp $DIR/target/release/nyelx /usr/bin