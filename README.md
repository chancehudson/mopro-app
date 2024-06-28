# mopro-app

Clone this repo then run `cargo run --bin ios`. This command will build a single `xcframework` containing libraries for the device and simulator. After this you can build in xcode without needing to run a cargo command again. Open `ios/mopro-test.xcodeproj` to run the app.

To build a release binary use the following command:

```
# CONFIGURATION is either debug or release
CONFIGURATION=release cargo run --bin ios
```