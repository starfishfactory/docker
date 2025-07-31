#!/bin/bash

# n8n 설치 스크립트
# NAS에서 실행하세요

echo "=== n8n Docker Compose 설정 스크립트 ==="
echo ""

# 현재 디렉토리 확인
CURRENT_DIR=$(pwd)
echo "현재 디렉토리: $CURRENT_DIR"
echo ""

# .env 파일이 없으면 .env.example에서 복사
if [ ! -f ".env" ]; then
    echo ".env 파일을 생성합니다..."
    cp .env.example .env
else
    echo ".env 파일이 이미 존재합니다."
fi

# 환경변수 설정 함수
update_env() {
    local key=$1
    local value=$2
    local file=".env"
    
    if grep -q "^${key}=" "$file"; then
        # macOS와 Linux 모두에서 작동하도록 수정
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s|^${key}=.*|${key}=${value}|" "$file"
        else
            sed -i "s|^${key}=.*|${key}=${value}|" "$file"
        fi
    else
        echo "${key}=${value}" >> "$file"
    fi
}

echo "=== 환경변수 설정 ==="
echo ""

# PostgreSQL 비밀번호 설정
read -sp "PostgreSQL 비밀번호 입력: " PG_PASS
echo ""
update_env "POSTGRES_PASSWORD" "$PG_PASS"

# n8n 관리자 비밀번호 설정
read -sp "n8n 관리자 비밀번호 입력: " N8N_PASS
echo ""
update_env "N8N_BASIC_AUTH_PASSWORD" "$N8N_PASS"

# NAS IP 주소 설정
echo "NAS IP 주소를 입력하세요 (예: 192.168.1.100)"
read -p "IP 주소: " NAS_IP
update_env "N8N_HOST" "$NAS_IP"
update_env "WEBHOOK_URL" "http://${NAS_IP}:5678/"

# 암호화 키 생성
echo ""
echo "암호화 키를 생성합니다..."
if command -v openssl &> /dev/null; then
    ENCRYPTION_KEY=$(openssl rand -hex 32)
else
    # openssl이 없을 경우 대체 방법
    ENCRYPTION_KEY=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 64 | head -n 1)
fi
update_env "N8N_ENCRYPTION_KEY" "$ENCRYPTION_KEY"

echo ""
echo "=== 디렉토리 권한 설정 ==="
# 데이터 디렉토리 권한 설정 (n8n은 node 사용자로 실행됨)
chmod 755 data postgres-data

echo ""
echo "=== Docker Compose 실행 ==="
echo ""

# Docker Compose 명령어 확인
if command -v docker-compose &> /dev/null; then
    DOCKER_COMPOSE_CMD="docker-compose"
elif command -v docker &> /dev/null && docker compose version &> /dev/null; then
    DOCKER_COMPOSE_CMD="docker compose"
else
    echo "ERROR: Docker 또는 Docker Compose가 설치되어 있지 않습니다."
    echo "NAS 패키지 센터에서 Docker를 먼저 설치해주세요."
    exit 1
fi

echo "Docker Compose를 시작하시겠습니까? (y/n)"
read -p "선택: " START_NOW

if [ "$START_NOW" = "y" ] || [ "$START_NOW" = "Y" ]; then
    echo ""
    echo "Docker Compose를 시작합니다..."
    $DOCKER_COMPOSE_CMD up -d
    
    echo ""
    echo "=== 상태 확인 ==="
    sleep 5
    $DOCKER_COMPOSE_CMD ps
    
    echo ""
    echo "=== n8n 설치 완료! ==="
    echo ""
    echo "접속 주소: http://${NAS_IP}:5678"
    echo "사용자명: admin (또는 .env에 설정한 값)"
    echo "비밀번호: 위에서 설정한 비밀번호"
else
    echo ""
    echo "=== 설정 완료 ==="
    echo ""
    echo "나중에 시작하려면 다음 명령어를 실행하세요:"
    echo "$DOCKER_COMPOSE_CMD up -d"
fi

echo ""
echo "=== 유용한 명령어 ==="
echo "상태 확인: $DOCKER_COMPOSE_CMD ps"
echo "로그 확인: $DOCKER_COMPOSE_CMD logs -f"
echo "중지: $DOCKER_COMPOSE_CMD down"
echo "재시작: $DOCKER_COMPOSE_CMD restart"
echo ""