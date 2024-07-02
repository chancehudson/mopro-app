## Android

When compiling the mopro default app using `cargo build --bin android`, an error about the NDK appeared. For that, [install the NDK](https://developer.android.com/studio/projects/install-ndk#default-version), and add its path to a variable named `ANDROID_NDK_HOME`. It is probably under `Sdk` and for me (running linux) was  `~/Android/Sdk/ndk`. [This question](https://stackoverflow.com/questions/39159357/how-to-set-android-ndk-home-so-that-android-studio-does-not-ask-for-ndk-location) helped setting the variable as well.
