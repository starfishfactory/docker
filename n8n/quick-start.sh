#!/bin/bash

# n8n 빠른 시작 스크립트 (환경변수 직접 설정)
# 주의: 이 스크립트는 테스트용입니다. 프로덕션에서는 setup.sh를 사용하세요.

echo "=== n8n 빠른 시작 (테스트용) ==="
echo ""

# 기본 환경변수 설정
cat > .env << EOF
# PostgreSQL 설정
POSTGRES_USER=n8n
POSTGRES_PASSWORD=n8n_password_$(date +%s)
POSTGRES_DB=n8n

# n8n 기본 인증 설정
N8N_BASIC_AUTH_USER=admin
N8N_BASIC_AUTH_PASSWORD=admin_$(date +%s)

# n8n 호스트 설정
N8N_HOST=$(hostname -I | awk '{print $1}')
N8N_PROTOCOL=http
WEBHOOK_URL=http://$(hostname -I | awk '{print $1}'):5678/

# 타임존 설정
GENERIC_TIMEZONE=Asia/Seoul

# 암호화 키
N8N_ENCRYPTION_KEY=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1)
EOF

echo "환경변수가 자동으로 설정되었습니다."
echo ""
cat .env
echo ""
echo "WARNING: 이 설정은 테스트용입니다. 프로덕션 환경에서는 비밀번호를 변경하세요!"
echo ""

# Docker Compose 실행
if command -v docker-compose &> /dev/null; then
    docker-compose up -d
elif command -v docker &> /dev/null && docker compose version &> /dev/null; then
    docker compose up -d
else
    echo "ERROR: Docker가 설치되어 있지 않습니다."
    exit 1
fi

echo ""
echo "=== n8n이 시작되었습니다! ==="
echo ""
echo "접속 정보는 위의 .env 파일을 확인하세요."