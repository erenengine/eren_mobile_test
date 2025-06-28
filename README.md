# eren_mobile_test

모바일에서 에렌엔진을 테스트하기 위한 소스코드 저장소입니다.

## 테스트 결과
- eren_vulkan의 경우, Android 디바이스 그래픽카드 드라이버가 Vulkan 1.3을 지원하지 않는 경우 실행 불가 (eren_android는 실행됨)
- eren의 경우, Android 시뮬레이터에서는 작동하지 않음(알려진 버그: https://github.com/gfx-rs/wgpu/issues/2384)
- iOS에서는 시뮬레이터용 라이브러리 파일(.a)과 실제 디바이스용 라이브러리 파일(.a)이 별도로 필요
