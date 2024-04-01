rm -r release/track2kml
mkdir -p release/track2kml 2> /dev/null

cp cli/README.md release/track2kml/

cargo build -p track2kml-cli --target x86_64-unknown-linux-musl --release && mv target/x86_64-unknown-linux-musl/release/track2kml release/track2kml/track2kml
cargo build -p track2kml-cli --release --target x86_64-pc-windows-gnu && mv target/x86_64-pc-windows-gnu/release/track2kml.exe release/track2kml/track2kml.exe


version=$(release/track2kml/track2kml --version | grep -oE "[^ ]+$")

cd release
zip -urq "${version}.zip" track2kml
