# 끝말잇기 — Rust Android APK

Rust + Slint로 만든 모바일 끝말잇기 게임입니다.

## `txt.txt` 준비

`txt.txt`는 별도로 준비합니다.
프로젝트 루트에 `txt.txt`를 넣고 한 줄에 하나의 단어를 적습니다. `#`로 시작하는 줄은 무시됩니다.

`txt.txt`는 빌드할 때 앱에 포함되므로, APK 빌드 전에 반드시 저장소 루트에 추가해야 합니다.

## 설치 없이 APK 만들기

1. 이 저장소 루트에 `txt.txt`를 추가합니다.
2. GitHub의 **Actions → Build Android APK → Run workflow**를 실행합니다.
3. 빌드가 끝나면 `rust-wordchain-apk` Artifact에서 APK를 받습니다.

GitHub Actions가 Rust, Java, Android SDK/NDK를 준비하므로 PC에 개발 도구를 직접 설치할 필요가 없습니다.

## 게임 규칙

- 컴퓨터가 먼저 단어를 냅니다.
- 표시된 마지막 글자로 시작하는 단어를 입력합니다.
- `txt.txt`에 있는 단어만 사용할 수 있습니다.
- 같은 단어는 한 번만 사용할 수 있습니다.
- 성공하면 점수와 콤보가 올라갑니다.
- 컴퓨터가 다음 단어를 찾지 못하면 승리합니다.

## 제작 기술

- Rust
- Slint 1.17.1
- Android NativeActivity backend
- cargo-apk
