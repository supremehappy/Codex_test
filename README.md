# Codex_test

이 저장소는 최소한의 Axum 기반 REST API 예제 프로젝트입니다.

## 로컬 환경 준비

이 예제를 실행하려면 먼저 러스트 도구체인이 필요합니다. 아래 명령어로 설치할 수
있습니다.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 후 새로운 셸을 열어 `cargo --version` 명령어가 동작하는지 확인합니다.

또한 필요한 의존성은 `cargo build` 또는 `cargo run` 시 자동으로 내려받아 설치되므로 별도 작업이 필요하지 않습니다.
프로젝트의 `Cargo.toml` 파일에는 `axum`이 이미 명시되어 있으므로 러스트가 준비되면 곧바로 빌드하여 사용할 수 있습니다.

## 서버 실행

```bash
cargo run -p rest_api
```

환경 변수 `HOST`와 `PORT`를 이용해 바인딩 주소를 설정할 수 있으며, 기본값은
`127.0.0.1:3000`입니다. 따라서 기본 설정에서는 로컬호스트에서만 접근이 가능합
니다.

## API 명세

### `GET /sample/getHello`
간단한 인사말을 반환합니다.

응답 예시:
```json
{ "message": "Hello" }
```

### `POST /sample/postContent`
JSON 형식의 `{ "content": "텍스트" }`를 받아 동일한 내용을 되돌려 줍니다.

요청 예시:
```bash
curl -X POST http://<서버주소>:3000/sample/postContent \
     -H 'Content-Type: application/json' \
     -d '{"content":"데이터"}'
```

## 외부에서 검증하는 방법

가상 머신이나 컨테이너에서 실행 중이라면 포트 `3000`을 외부에 노출해야 합니다. 
예를 들어 Docker를 사용한다면 `-p 3000:3000` 옵션을 통해 포트를 매핑한 뒤 다음과 같이 확인할 수 있습니다.

```bash
curl http://<서버주소>:3000/sample/getHello
```

## 테스트 실행

```bash
cargo test -p rest_api
```

## 보안 개선 사항

- 서버 주소는 `HOST`와 `PORT` 환경 변수로 지정하며 기본값은 `127.0.0.1:3000`입니다.
- `POST /sample/postContent` 요청은 본문 길이를 최대 256자로 제한합니다. 초과하면
  `400 Bad Request`를 반환합니다.
- 서버 실행 중 오류가 발생해도 패닉하지 않고 로그를 출력합니다.
