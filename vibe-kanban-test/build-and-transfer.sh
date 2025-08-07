#!/bin/bash
# vibe-kanban 이미지 빌드 및 전송 스크립트

# 기본 설정
IMAGE_NAME="${IMAGE_NAME:-vibe-kanban-enhanced:latest}"
TAR_FILE="${TAR_FILE:-vibe-kanban-enhanced.tar.gz}"
BUILD_ONLY="${BUILD_ONLY:-false}"
PUSH_TO_REGISTRY="${PUSH_TO_REGISTRY:-false}"
REGISTRY="${REGISTRY:-}"

# NAS 전송 설정 (선택사항)
NAS_USER="${NAS_USER:-}"
NAS_HOST="${NAS_HOST:-}"
NAS_PATH="${NAS_PATH:-}"

echo "========================================="
echo "vibe-kanban Enhanced Image Build Script"
echo "========================================="

# Docker 설치 확인
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed!"
    echo "Please install Docker first."
    exit 1
fi

# 1. 이미지 빌드
echo ""
echo "1. Building Docker image: $IMAGE_NAME"
echo "This may take several minutes..."

# 멀티플랫폼 빌드 지원
if command -v docker buildx &> /dev/null && [ "$PUSH_TO_REGISTRY" = "true" ] && [ -n "$REGISTRY" ]; then
    echo "Building multi-platform image and pushing to registry..."
    docker buildx build --platform linux/amd64,linux/arm64 -t $REGISTRY/$IMAGE_NAME --push .
else
    echo "Building single-platform image..."
    docker build -t $IMAGE_NAME .
fi

if [ $? -ne 0 ]; then
    echo "Error: Image build failed!"
    exit 1
fi

echo "✓ Image built successfully!"

# 2. 이미지 크기 확인
echo ""
echo "2. Checking image size..."
docker images $IMAGE_NAME

# Build-only 모드인 경우 여기서 종료
if [ "$BUILD_ONLY" = "true" ]; then
    echo ""
    echo "Build-only mode: Skipping image save and transfer"
    echo "========================================="
    exit 0
fi

# 3. 이미지 저장 및 압축
echo ""
echo "3. Saving and compressing image..."
docker save $IMAGE_NAME | gzip > $TAR_FILE

if [ $? -ne 0 ]; then
    echo "Error: Failed to save image!"
    exit 1
fi

# 파일 크기 확인
FILE_SIZE=$(ls -lh $TAR_FILE | awk '{print $5}')
echo "✓ Image saved: $TAR_FILE (Size: $FILE_SIZE)"

# 4. NAS로 전송 (선택사항)
if [ -n "$NAS_USER" ] && [ -n "$NAS_HOST" ] && [ -n "$NAS_PATH" ]; then
    echo ""
    echo "4. Transferring to NAS..."
    echo "Target: $NAS_USER@$NAS_HOST:$NAS_PATH"
    
    scp $TAR_FILE $NAS_USER@$NAS_HOST:$NAS_PATH
    
    if [ $? -eq 0 ]; then
        echo "✓ Transfer completed successfully!"
        
        # 로컬 tar 파일 정리 (선택사항)
        if [ "${CLEANUP_LOCAL:-false}" = "true" ]; then
            rm $TAR_FILE
            echo "✓ Local tar file cleaned up"
        fi
    else
        echo "Error: Transfer failed!"
        exit 1
    fi
else
    echo ""
    echo "4. NAS transfer skipped (credentials not provided)"
fi

echo ""
echo "========================================="
echo "Build completed!"
echo ""
echo "Usage options:"
echo "1. Manual NAS transfer:"
echo "   scp $TAR_FILE user@nas-ip:/path/to/destination"
echo ""
echo "2. Load image on target system:"
echo "   docker load < $TAR_FILE"
echo ""
echo "3. Use with docker-compose:"
echo "   Update image name to '$IMAGE_NAME' in docker-compose.yml"
echo ""
echo "Environment variables:"
echo "  IMAGE_NAME=$IMAGE_NAME"
echo "  BUILD_ONLY=$BUILD_ONLY"
echo "  PUSH_TO_REGISTRY=$PUSH_TO_REGISTRY"
echo "========================================="