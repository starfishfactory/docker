# AI Development Environment

Claude Code와 vibe-kanban을 통합한 AI 개발 환경입니다.

## 포함된 도구

- **Claude Code**: Anthropic의 AI 코딩 어시스턴트
- **Gemini CLI**: Google의 AI CLI 도구
- **GitHub CLI**: GitHub 작업을 위한 CLI
- **vibe-kanban**: 프로젝트 관리를 위한 칸반 보드 웹 애플리케이션

## 설치 및 실행

### 1. 환경 변수 설정

```bash
cp .env.example .env
```

`.env` 파일을 편집하여 API 키를 설정하세요:
- `ANTHROPIC_API_KEY`: Claude API 키
- `GEMINI_API_KEY`: Gemini API 키
- `GITHUB_TOKEN`: GitHub 개인 액세스 토큰

### 2. 컨테이너 빌드 및 실행

```bash
# 빌드
docker compose build

# 실행 (개발 환경 - SSH/Git 설정 포함)
docker compose up -d

# 또는 실행 (프로덕션 - SSH/Git 설정 제외)
docker compose -f docker-compose.prod.yml up -d

# 컨테이너 접속
docker compose exec ai-dev bash
```

**참고**: SSH 키나 Git 설정이 없는 경우 `docker-compose.prod.yml`을 사용하거나, `docker-compose.yml`의 해당 라인을 주석 해제하세요.

### 3. 서비스 접근

- **vibe-kanban**: http://localhost:8100
- **터미널**: `docker compose exec ai-dev bash`

## 사용 방법

### Claude Code 사용

컨테이너 내에서:
```bash
claude-code
```

### Gemini CLI 사용

```bash
gemini [명령어]
```

### GitHub CLI 사용

```bash
gh [명령어]
```

### vibe-kanban 접근

브라우저에서 http://localhost:8100 접속

### Screen 세션 관리

vibe-kanban은 screen 세션에서 실행됩니다:
```bash
# 세션 확인
screen -ls

# vibe-kanban 세션 접속
screen -r vibe-kanban

# 세션에서 나오기 (세션 유지)
Ctrl+A, D
```

## 디렉토리 구조

```
ai/
├── Dockerfile          # 통합 Docker 이미지
├── docker-compose.yml  # Docker Compose 설정
├── start.sh           # 시작 스크립트
├── .env.example       # 환경 변수 예제
├── .gitignore         # Git 무시 파일
├── workspace/         # 메인 작업 디렉토리 (마운트됨)
├── repos/            # vibe-kanban 저장소 디렉토리 (마운트됨)
└── data/             # vibe-kanban 데이터 (영구 저장)
```

## 볼륨 마운트

- `./workspace:/workspace`: 메인 작업 공간
- `./repos:/repos`: vibe-kanban 프로젝트 저장소
- `./data:/home/vibe/.local/share/vibe-kanban`: vibe-kanban 데이터 영구 저장
- `~/.ssh`: SSH 키 (읽기 전용)
- `~/.gitconfig`: Git 설정 (읽기 전용)

## 문제 해결

### vibe-kanban이 시작되지 않을 때

```bash
# 컨테이너 내에서
screen -r vibe-kanban
# 로그 확인 후 수동 시작
cd /repos && su vibe -c 'npx --yes vibe-kanban@latest'
```

### 권한 문제

```bash
# 호스트에서
sudo chown -R $USER:$USER ./data ./repos ./workspace
```

## 종료

```bash
# 컨테이너 중지
docker compose down

# 컨테이너 및 볼륨 완전 삭제
docker compose down -v
```