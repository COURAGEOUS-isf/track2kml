rm -r release
mkdir release

cp cli/README.md release

cargo build --target x86_64-unknown-linux-musl --release && mv target/x86_64-unknown-linux-musl/release/track2kml release/track2kml
cargo build --release --target x86_64-pc-windows-gnu && mv target/x86_64-pc-windows-gnu/release/track2kml.exe release/track2kml.exe


version=$(release/COURAGEOUS/track2kml --version | grep -oE "[^ ]+$")

zip -ur "release/COURAGEOUS-${version}.zip" release
