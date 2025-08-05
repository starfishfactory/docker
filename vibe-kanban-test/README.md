# Vibe Kanban Enhanced Docker 가이드

Vibe Kanban은 AI 코딩 에이전트(Claude Code, Gemini CLI, Amp 등)를 통합 관리하는 칸반 보드 시스템입니다.

이 Enhanced 버전은 추가적인 개발 도구들이 포함되어 있습니다:
- GitHub CLI (gh)
- Screen 유틸리티
- 향상된 Git 지원
- 기타 유용한 명령줄 도구들

## 특징

- 여러 AI 코딩 에이전트를 한 곳에서 관리
- 실시간 작업 상태 추적
- GitHub 통합 지원
- Git worktree를 통한 동시 개발 지원
- 웹 기반 인터페이스
- **추가된 개발 도구들로 향상된 개발 환경**

## 설치 방법

### 1. 필수 요구사항

- Docker 및 Docker Compose 설치
- NAS 또는 서버 터미널 접근 권한

### 2. 이미지 빌드 방법

#### 방법 A: 로컬 빌드 (권장)
```bash
# 저장소 클론
git clone https://github.com/starfishfactory/docker.git
cd docker/vibe-kanban

# Enhanced 이미지 빌드
./build-and-transfer.sh
```

#### 방법 B: GitHub Actions 자동 빌드
이 저장소는 GitHub Actions를 통한 자동 빌드를 지원합니다:
- `vibe-kanban/` 폴더 변경 시 자동으로 이미지 빌드
- 빌드된 이미지는 GitHub Container Registry와 Artifacts에서 다운로드 가능

#### 방법 C: 사전 빌드된 이미지 사용
```bash
# GitHub Container Registry에서 pull
docker pull ghcr.io/starfishfactory/vibe-kanban-enhanced:latest

# 또는 GitHub Actions Artifacts에서 다운로드 후
docker load < vibe-kanban-enhanced.tar.gz
```

### 3. 환경 설정

```bash
# .env.example을 복사하여 .env 파일 생성
cp .env.example .env

# 필요한 경우 .env 파일 편집
# GitHub OAuth 설정 (선택사항)
# GITHUB_CLIENT_ID와 GITHUB_CLIENT_SECRET 입력
```

### 4. 컨테이너 실행

```bash
# Enhanced 이미지 사용
docker-compose -f docker-compose-enhanced.yml up -d

# 또는 기본 설정 사용
docker-compose up -d
```

### 5. 접속 확인

웹 브라우저에서 접속:
- URL: http://[SERVER_IP]:8100

## CI/CD 자동 빌드

이 프로젝트는 GitHub Actions를 통한 자동 빌드를 지원합니다:

### 트리거 조건
- `main` 또는 `master` 브랜치에 push
- `vibe-kanban/` 폴더의 파일 변경사항이 있을 때
- 수동 실행 (workflow_dispatch)

### 빌드 결과물
1. **GitHub Container Registry**: `ghcr.io/[owner]/vibe-kanban-enhanced:latest`
2. **GitHub Artifacts**: 압축된 이미지 파일 다운로드 가능 (30일 보관)

### 빌드 스크립트 사용법
```bash
# 기본 빌드
./build-and-transfer.sh

# 빌드만 수행 (저장 및 전송 생략)
BUILD_ONLY=true ./build-and-transfer.sh

# NAS로 자동 전송
NAS_USER=user NAS_HOST=192.168.1.100 NAS_PATH=/path/to/destination ./build-and-transfer.sh

# 레지스트리에 push
PUSH_TO_REGISTRY=true REGISTRY=ghcr.io/owner ./build-and-transfer.sh
```

## 디렉토리 구조

```
vibe-kanban/
├── .github/
│   └── workflows/
│       └── build-vibe-kanban.yml  # GitHub Actions 워크플로우
├── docker-compose.yml             # 기본 Docker Compose 설정
├── docker-compose-enhanced.yml    # Enhanced 이미지용 설정
├── Dockerfile                     # Enhanced 이미지 빌드용
├── build-and-transfer.sh          # 빌드 및 전송 스크립트
├── .env                          # 환경 변수 (생성 필요)
├── .env.example                  # 환경 변수 예제
├── data/                         # 데이터베이스 저장
├── repos/                        # Git 저장소
├── config/                       # 설정 파일
└── README.md                     # 이 문서
```

## 환경 변수 설정

### GitHub OAuth (선택사항)

AI 에이전트가 GitHub와 연동하려면 OAuth 앱 생성이 필요합니다:

1. [GitHub Settings > Developer settings > OAuth Apps](https://github.com/settings/developers) 접속
2. "New OAuth App" 클릭
3. 다음 정보 입력:
   - Application name: Vibe Kanban
   - Homepage URL: http://[NAS_IP]:3000
   - Authorization callback URL: http://[NAS_IP]:3000/api/auth/github/callback
4. 생성된 Client ID와 Client Secret을 .env 파일에 입력

### PostHog Analytics (선택사항)

사용 분석을 위해 PostHog 설정 가능:
- POSTHOG_API_KEY: PostHog API 키
- POSTHOG_HOST: PostHog 호스트 URL

## 관리 명령어

```bash
# 컨테이너 상태 확인
docker-compose ps

# 로그 확인
docker-compose logs -f

# 컨테이너 중지
docker-compose stop

# 컨테이너 재시작
docker-compose restart

# 컨테이너 제거 (데이터는 유지됨)
docker-compose down

# 컨테이너 및 데이터 완전 제거
docker-compose down -v
rm -rf data/ repos/ config/
```

## 업데이트

최신 버전으로 업데이트:

```bash
docker-compose pull
docker-compose up -d
```

## 문제 해결

### 포트 충돌

기본 포트 3000이 사용 중인 경우 docker-compose.yml에서 포트 변경:

```yaml
ports:
  - "8080:3000"  # 8080으로 변경
```

### 권한 문제

데이터 디렉토리 권한 문제 발생 시:

```bash
chmod -R 755 data/ repos/ config/
```

### 컨테이너 재빌드

소스에서 빌드하려면 docker-compose.yml의 주석 처리된 부분 사용:

```bash
# image 섹션을 주석 처리하고
# vibe-kanban-build 섹션의 주석 해제 후
docker-compose up -d --build
```

## 추가 정보

- 공식 저장소: https://github.com/BloopAI/vibe-kanban
- 공식 웹사이트: https://www.vibekanban.com/
- 라이선스: MIT