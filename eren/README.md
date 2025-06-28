## Android Build
```
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release
```

## iOS Build
```
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
```
