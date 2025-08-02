# Docker 이미지 빌드 및 NAS 전송 가이드

이 가이드는 성능이 좋은 컴퓨터에서 Docker 이미지를 빌드하고, NAS로 전송하여 실행하는 방법을 설명합니다.

## 1. 성능 좋은 컴퓨터에서 이미지 빌드

### 방법 1: GitHub에서 직접 빌드
```bash
docker build -t vibe-kanban:latest https://github.com/BloopAI/vibe-kanban.git
```

### 방법 2: 로컬에 클론 후 빌드
```bash
git clone https://github.com/BloopAI/vibe-kanban.git
cd vibe-kanban
docker build -t vibe-kanban:latest .
```

## 2. 이미지를 파일로 저장

### 압축하여 저장 (권장)
```bash
docker save vibe-kanban:latest | gzip > vibe-kanban.tar.gz
```

### 압축 없이 저장
```bash
docker save -o vibe-kanban.tar vibe-kanban:latest
```

## 3. NAS로 파일 전송

### SCP 사용
```bash
scp vibe-kanban.tar.gz user@nas-ip:/path/to/destination
```

### rsync 사용 (대용량 파일에 유용)
```bash
rsync -avz --progress vibe-kanban.tar.gz user@nas-ip:/path/to/destination
```

## 4. NAS에서 이미지 로드 및 실행

### 이미지 로드
```bash
# 압축된 파일인 경우
docker load < vibe-kanban.tar.gz

# 또는
gunzip -c vibe-kanban.tar.gz | docker load

# 압축되지 않은 파일인 경우
docker load -i vibe-kanban.tar
```

### docker-compose.yml 수정
```yaml
# 기존 설정
# image: ghcr.io/bloopai/vibe-kanban:latest

# 로컬 이미지 사용으로 변경
image: vibe-kanban:latest
```

### 컨테이너 실행
```bash
docker-compose up -d
```

## 5. 자동화 스크립트

### build-and-transfer.sh
```bash
#!/bin/bash
# 빌드 및 전송 자동화 스크립트

# 설정
IMAGE_NAME="vibe-kanban:latest"
TAR_FILE="vibe-kanban.tar.gz"
NAS_USER="your-user"
NAS_HOST="nas-ip"
NAS_PATH="/path/to/destination"

# 1. 이미지 빌드
echo "Building Docker image..."
docker build -t $IMAGE_NAME https://github.com/BloopAI/vibe-kanban.git

# 2. 이미지 저장
echo "Saving image to file..."
docker save $IMAGE_NAME | gzip > $TAR_FILE

# 3. NAS로 전송
echo "Transferring to NAS..."
scp $TAR_FILE $NAS_USER@$NAS_HOST:$NAS_PATH

# 4. 로컬 tar 파일 삭제 (선택사항)
rm $TAR_FILE

echo "Done!"
```

### NAS에서 실행할 load-and-run.sh
```bash
#!/bin/bash
# NAS에서 이미지 로드 및 실행

# 이미지 로드
echo "Loading Docker image..."
docker load < vibe-kanban.tar.gz

# docker-compose 실행
echo "Starting container..."
docker-compose up -d

echo "Container started!"
```

## 팁

1. **이미지 크기 확인**: `docker images` 명령으로 이미지 크기를 미리 확인
2. **전송 시간 단축**: 압축을 사용하면 전송 시간을 크게 줄일 수 있음
3. **버전 관리**: 이미지에 태그를 붙여 버전 관리 (예: `vibe-kanban:v1.0`)
4. **정리**: 불필요한 이미지는 `docker image prune`으로 정리

## 문제 해결

### 이미지 로드 실패 시
```bash
# 이미지 목록 확인
docker images

# 기존 이미지 삭제 후 재로드
docker rmi vibe-kanban:latest
docker load < vibe-kanban.tar.gz
```

### 컨테이너 실행 실패 시
```bash
# 로그 확인
docker-compose logs

# 컨테이너 상태 확인
docker ps -a
```