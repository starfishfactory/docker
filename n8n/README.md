# n8n Docker Compose 설정

n8n 워크플로우 자동화 도구를 Docker Compose로 설치하는 설정입니다.

## 시작하기

1. `.env` 파일 수정
   ```bash
   cp .env.example .env
   nano .env
   ```
   
   다음 항목들을 반드시 변경하세요:
   - `POSTGRES_PASSWORD`: PostgreSQL 데이터베이스 비밀번호
   - `N8N_BASIC_AUTH_PASSWORD`: n8n 웹 인터페이스 비밀번호
   - `N8N_HOST`: NAS의 실제 IP 주소나 도메인
   - `WEBHOOK_URL`: 웹훅 URL (NAS IP 포함)
   - `N8N_ENCRYPTION_KEY`: 랜덤 암호화 키 (openssl rand -hex 32)

2. Docker Compose 실행
   ```bash
   docker-compose up -d
   ```

3. n8n 접속
   - URL: `http://[NAS_IP]:5678`
   - 사용자명: `.env`에 설정한 `N8N_BASIC_AUTH_USER`
   - 비밀번호: `.env`에 설정한 `N8N_BASIC_AUTH_PASSWORD`

## 디렉토리 구조

```
n8n/
├── docker-compose.yml    # Docker Compose 설정
├── .env                  # 환경변수 (git에 포함되지 않음)
├── .env.example          # 환경변수 예시
├── data/                 # n8n 데이터 (워크플로우, 설정 등)
└── postgres-data/        # PostgreSQL 데이터베이스
```

## 유용한 명령어

```bash
# 컨테이너 상태 확인
docker-compose ps

# 로그 확인
docker-compose logs -f

# 중지
docker-compose down

# 업데이트
docker-compose pull
docker-compose up -d

# 데이터 백업
tar -czf n8n-backup-$(date +%Y%m%d).tar.gz data/ postgres-data/
```

## 주의사항

- `.env` 파일은 민감한 정보를 포함하므로 git에 커밋하지 마세요
- `N8N_ENCRYPTION_KEY`는 한번 설정 후 변경하면 기존 자격증명이 작동하지 않습니다
- 정기적으로 데이터를 백업하세요