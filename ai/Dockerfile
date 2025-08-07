FROM node:latest

# 기본 유틸리티 설치 (vibe-kanban 지원 포함)
RUN apt-get update && \
    apt-get install -y screen curl gnupg lsb-release ca-certificates \
    git openssh-client wget unzip libnotify-bin dbus nano vim

# GitHub CLI GPG 키 등록 (최신 주소)
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
    -o /usr/share/keyrings/githubcli-archive-keyring.gpg

# GitHub CLI APT 저장소 등록
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] \
https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null

# GitHub CLI 설치
RUN apt-get update && apt-get install -y gh

# APT 캐시 정리
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# 글로벌 NPM 패키지 설치
RUN npm install -g @anthropic-ai/claude-code
RUN npm install -g @google/gemini-cli
RUN npm install -g vibe-kanban

# NPM 캐시 설정 (vibe-kanban용)
ENV NPM_CONFIG_CACHE=/tmp/npm-cache
ENV NPM_CONFIG_YES=true

# 필요한 디렉토리 생성
RUN mkdir -p /repos /root/.local/share/vibe-kanban

# 작업 디렉토리 지정
WORKDIR /workspace

# Copy vibe-kanban start script
COPY start-vibe.sh /usr/local/bin/start-vibe.sh
RUN chmod +x /usr/local/bin/start-vibe.sh

# 기본 쉘
CMD ["bash"]