# AI 개발환경 통합 계획

## 📋 개요
Claude Code와 vibe-kanban을 통합한 AI 개발환경을 Docker 컨테이너로 구성합니다.

## 🎯 목표
- Claude Code, Gemini CLI 등 AI 도구를 터미널에서 사용
- vibe-kanban 웹 인터페이스로 프로젝트 관리
- 하나의 컨테이너에서 모든 도구 통합 실행

## 🏗️ 아키텍처

### 기반 이미지
- Node.js 18-slim (경량화)

### 포함 도구
1. **AI CLI 도구**
   - Claude Code (@anthropic-ai/claude-code)
   - Gemini CLI (@google/gemini-cli)
   
2. **개발 도구**
   - GitHub CLI (gh)
   - Git
   - Screen (세션 관리)
   
3. **프로젝트 관리**
   - vibe-kanban (웹 기반 칸반 보드)

## 📁 디렉토리 구조
```
docker/ai/
├── Dockerfile           # 통합 이미지
├── docker-compose.yml   # 컨테이너 설정
├── start.sh            # 시작 스크립트
├── .env.example        # 환경변수 템플릿
├── workspace/          # 작업 공간 (마운트)
├── repos/             # 저장소 (마운트)
└── data/              # 영구 데이터 (마운트)
```

## 🚀 실행 방식

### 1. vibe-kanban
- Screen 세션에서 백그라운드 실행
- 포트 3000으로 웹 인터페이스 제공
- 전용 사용자(vibe)로 실행

### 2. AI 도구
- 터미널에서 직접 사용
- API 키는 환경변수로 관리

### 3. 데이터 영속성
- 볼륨 마운트로 데이터 보존
- Git 설정 및 SSH 키 자동 연결

## 🔧 설정

### 필수 환경변수
```env
ANTHROPIC_API_KEY=xxx
GEMINI_API_KEY=xxx  
GITHUB_TOKEN=xxx
```

### 포트
- 3000: vibe-kanban 웹 인터페이스

### 볼륨
- `./workspace`: 메인 작업 디렉토리
- `./repos`: vibe-kanban 저장소
- `./data`: vibe-kanban 데이터
- `~/.ssh`, `~/.gitconfig`: 개발 설정 (읽기 전용)

## 📊 장점

1. **통합 환경**: 하나의 컨테이너에서 모든 도구 사용
2. **편의성**: 터미널과 웹 UI 동시 제공
3. **영속성**: 데이터와 설정 보존
4. **격리성**: Docker로 깨끗한 개발환경 유지

## 🔄 워크플로우

1. 컨테이너 시작
2. vibe-kanban 자동 실행 (백그라운드)
3. 터미널 접속
4. AI 도구로 코딩 작업
5. vibe-kanban으로 프로젝트 관리

## 📈 향후 개선사항

- [ ] 추가 AI 도구 통합
- [ ] 자동 백업 기능
- [ ] 멀티 프로젝트 지원
- [ ] 플러그인 시스템