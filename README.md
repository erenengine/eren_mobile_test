# eren_mobile_test

모바일에서 에렌엔진을 테스트하기 위한 소스코드 저장소입니다.

## Android Build
```
cargo ndk -t arm64-v8a -o target/android build --release
```

## 테스트 결과
- eren_vulkan_android의 경우, Android 디바이스 그래픽카드 드라이버가 Vulkan 1.3을 지원하지 않는 경우 실행 불가 (eren_android는 실행됨)
