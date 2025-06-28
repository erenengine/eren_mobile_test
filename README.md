# eren_mobile_test

모바일에서 에렌엔진을 테스트하기 위한 소스코드 저장소입니다.

## 테스트 결과
- eren_vulkan의 경우, Android 디바이스 그래픽카드 드라이버가 Vulkan 1.3을 지원하지 않는 경우 실행 불가 (eren_android는 실행됨)
- eren_vulkan의 경우, pre_transform에 따라 프로젝션 행렬이 다르게 적용되어야 함
- eren의 경우, Android 시뮬레이터에서는 작동하지 않음(알려진 버그: https://github.com/gfx-rs/wgpu/issues/2384)
- iOS에서는 시뮬레이터용 라이브러리 파일(.a)과 실제 디바이스용 라이브러리 파일(.a)이 별도로 필요
- iOS에서는 winit 버그로 인해, about_to_wait를 활용해 redraw 요청을 처리해야 함
