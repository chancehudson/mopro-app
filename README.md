# mopro-app

Clone this repo then run `cargo run --bin ios`. After that you can build in xcode without needing to run a cargo command again. Open `ios/mopro-test.xcodeproj` to build for iOS.

Xcode _should_ automatically call cargo and build the appropriate binary by looking at the active target and profile. Apple has broken a bunch of things with library pathing though, which is why the initial cargo command above is needed.