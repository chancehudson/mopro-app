# mopro-app

Clone this repo then run either `cargo run --bin ios` or `cargo run --bin android`. This command will build either an `xcframework` or a `jniLibs` folder containing libraries for the device and simulator. After this you can build in xcode/android studio without needing to run a cargo command again. Open the contents of `ios` or `android` to build the apps.

To build a release binary use the following commands:

```
# CONFIGURATION is either debug or release
CONFIGURATION=release cargo run --bin ios
CONFIGURATION=release cargo run --bin android
```