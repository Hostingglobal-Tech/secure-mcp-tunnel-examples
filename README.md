# Secure MCP Tunnel 웨비나 예제

OpenAI Secure MCP Tunnel을 통해 ChatGPT의 사용자 지정 MCP 플러그인이 PC에서 실행 중인 도구를 호출하는 예제입니다. 두 Rust 예제는 가상 데이터로 동작합니다.

```text
ChatGPT → OpenAI 터널 서비스 ← 아웃바운드 HTTPS 폴링 ← tunnel-client
                                                          ↕ 표준 입출력
                                                       Rust MCP 서버
```

`tunnel-client`는 방화벽 안에서 OpenAI로 HTTPS 연결을 시작합니다. PC의 MCP 서버는 표준 입출력으로 요청을 받습니다. 외부에서 PC로 들어오는 방화벽 포트를 열 필요가 없습니다. 도구가 돌려준 결과는 터널을 통해 OpenAI에 전달됩니다.

| 예제 | MCP 도구 | 시연 입력 | 예상 결과 |
| --- | --- | --- | --- |
| [가상 서비스 상태](examples/service_catalog.rs) | `get_demo_service_status` | `{"service":"demo_queue"}` | `demo_queue: degraded (synthetic backlog)` |
| [가상 PC 화면](examples/workstation_view.rs) | `get_demo_workspace_view` | `{}` | `Window: Demo Console`, `Status: Ready` |

서비스 조회는 정해진 가상 서비스 이름만 받습니다. 화면 조회는 코드에 정의한 가상 화면의 글자를 반환합니다.

## 빌드

Rust 1.88 이상과 [OpenAI tunnel-client](https://github.com/openai/tunnel-client/releases/latest)를 설치하라. 저장소 루트에서 실행하라.

```sh
cargo build --release --locked --examples
```

실행 파일은 `target/release/examples/`에 생성됩니다. Windows 실행 파일에는 `.exe`가 붙습니다.

## API 키 없이 로컬에서 호출

PowerShell 7에서 다음 명령을 실행하라. 이 명령은 현재 PC의 루프백에서 터널을 모의 실행합니다.

```powershell
$server = (Resolve-Path './target/release/examples/service_catalog.exe').Path.Replace('\', '/')
tunnel-client dev proxy --mcp-command ('"' + $server + '"') --duration 5m --print-json
```

macOS와 Linux에서는 다음 명령을 실행하라.

```bash
tunnel-client dev proxy --mcp-command "\"$PWD/target/release/examples/service_catalog\"" --duration 5m --print-json
```

MCP 클라이언트에 출력된 MCP URL을 Streamable HTTP 주소로 입력하라. `get_demo_service_status`를 호출하고 `service`에 `demo_queue`를 넣어라. 가상 PC 화면을 시연할 때는 실행 파일을 `workstation_view`로 바꿔라.

## ChatGPT에서 개인 시연 터널로 호출

1. [Platform 터널 설정](https://platform.openai.com/settings/organization/tunnels)에서 시연 터널을 만들어라.
2. 터널을 시연에 사용할 ChatGPT 워크스페이스에 연결하라.
3. Tunnels **Read + Use** 권한이 있는 런타임 키를 준비하라.
4. 아래 명령으로 프로필을 만들고 `tunnel-client`를 실행하라.

PowerShell 7에서는 키를 마스킹해 입력합니다. 프로필은 키 값 대신 환경 변수 이름을 참조합니다.

```powershell
$tunnelId = Read-Host 'Demo tunnel ID'
$env:CONTROL_PLANE_API_KEY = Read-Host 'Tunnel runtime API key' -MaskInput
$server = (Resolve-Path './target/release/examples/service_catalog.exe').Path.Replace('\', '/')
tunnel-client init --sample sample_mcp_stdio_local --profile webinar-services --tunnel-id $tunnelId --mcp-command ('"' + $server + '"')
tunnel-client doctor --profile webinar-services --explain
tunnel-client run --profile webinar-services --mcp.stdio-send-initialized-notification
```

Bash에서는 다음 명령을 실행하라.

```bash
read -r -p 'Demo tunnel ID: ' TUNNEL_ID
read -r -s -p 'Tunnel runtime API key: ' CONTROL_PLANE_API_KEY
printf '\n'
export CONTROL_PLANE_API_KEY
tunnel-client init --sample sample_mcp_stdio_local --profile webinar-services --tunnel-id "$TUNNEL_ID" --mcp-command "\"$PWD/target/release/examples/service_catalog\""
tunnel-client doctor --profile webinar-services --explain
tunnel-client run --profile webinar-services --mcp.stdio-send-initialized-notification
```

[ChatGPT Plugins](https://chatgpt.com/plugins)에서 사용자 지정 MCP 서버를 추가하라. 연결 방식은 **Tunnel**을 선택하라. 위에서 사용한 터널을 연결하라. 아래 문장으로 도구가 실제로 응답하는지 확인하라.

> get_demo_service_status 도구로 demo_queue의 상태를 조회해 줘.

PC 화면 예제에는 별도 시연 터널을 써라. 프로필 이름을 `webinar-workstation`으로 바꿔라. 실행 파일을 `workstation_view`로 바꿔라. 두 프로필을 동시에 실행하려면 두 번째 프로필의 init 명령에 `--health-listen-addr 127.0.0.1:8081`을 추가하라.

> get_demo_workspace_view 도구를 호출해서 가상 화면의 창 이름과 상태를 알려 줘.

시연을 마치면 Ctrl+C로 `tunnel-client`를 종료하라. 런타임 키를 입력한 터미널을 닫아라.

[OpenAI Secure MCP Tunnel 공식 문서](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels)
