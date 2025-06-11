# Codex_test

이 저장소는 최소한의 Axum 기반 REST API 예제 프로젝트입니다.

## 서버 실행

```bash
cargo run -p rest_api
```

서버는 기본적으로 `0.0.0.0:3000`에서 실행되므로 외부에서도 접근할 수 있습니다.

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
