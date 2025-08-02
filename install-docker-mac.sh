#!/bin/bash
# Docker Desktop for Mac 설치 스크립트

echo "========================================="
echo "Docker Desktop for Mac 설치 가이드"
echo "========================================="
echo ""
echo "시스템 정보:"
echo "- Architecture: arm64 (Apple Silicon)"
echo "- macOS Version: 15.5"
echo ""

# Homebrew 설치 확인
if ! command -v brew &> /dev/null; then
    echo "Homebrew가 설치되어 있지 않습니다."
    echo "Homebrew를 먼저 설치하시겠습니까? (y/n)"
    read -r response
    if [[ "$response" == "y" ]]; then
        echo "Homebrew 설치 중..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        
        # Apple Silicon의 경우 PATH 설정
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
        eval "$(/opt/homebrew/bin/brew shellenv)"
    else
        echo "Homebrew 없이는 자동 설치가 어렵습니다."
        echo "Docker Desktop을 수동으로 설치하려면:"
        echo "1. https://www.docker.com/products/docker-desktop/ 방문"
        echo "2. 'Download for Mac - Apple Chip' 클릭"
        echo "3. 다운로드된 Docker.dmg 파일 실행"
        exit 0
    fi
fi

echo ""
echo "Docker Desktop 설치 옵션:"
echo "1. Homebrew를 통한 설치 (권장)"
echo "2. 수동 다운로드 안내"
echo ""
echo "선택하세요 (1 또는 2):"
read -r choice

case $choice in
    1)
        echo ""
        echo "Homebrew를 통해 Docker Desktop 설치 중..."
        brew install --cask docker
        
        if [ $? -eq 0 ]; then
            echo ""
            echo "✓ Docker Desktop이 성공적으로 설치되었습니다!"
            echo ""
            echo "다음 단계:"
            echo "1. Applications 폴더에서 Docker 앱 실행"
            echo "2. 메뉴바에서 Docker 아이콘이 나타날 때까지 대기"
            echo "3. Docker Desktop 초기 설정 완료"
            echo ""
            echo "설치 확인 명령어:"
            echo "  docker --version"
            echo "  docker compose version"
        else
            echo "설치 중 오류가 발생했습니다."
        fi
        ;;
    2)
        echo ""
        echo "Docker Desktop 수동 설치 안내:"
        echo ""
        echo "1. 다음 URL 방문:"
        echo "   https://www.docker.com/products/docker-desktop/"
        echo ""
        echo "2. 'Download for Mac - Apple Chip' 버튼 클릭"
        echo ""
        echo "3. Docker.dmg 파일 다운로드 완료 후:"
        echo "   - Docker.dmg 더블클릭"
        echo "   - Docker 아이콘을 Applications 폴더로 드래그"
        echo "   - Applications에서 Docker 실행"
        echo ""
        echo "4. 초기 설정 완료 후 터미널에서 확인:"
        echo "   docker --version"
        ;;
    *)
        echo "잘못된 선택입니다."
        exit 1
        ;;
esac

echo ""
echo "========================================="
echo "설치 완료 후 vibe-kanban 빌드를 진행하세요:"
echo "cd /Users/yujinhyeong/IdeaProjects/docker/vibe-kanban"
echo "./build-and-transfer.sh"
echo "========================================="