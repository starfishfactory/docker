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

# yoojinhyung 사용자 생성 (호스트와 동일한 UID/GID)
RUN groupadd -g 100 users || true
RUN useradd -u 1026 -g 100 -m -s /bin/bash yoojinhyung

# 필요한 디렉토리 생성 및 권한 설정
RUN mkdir -p /repos /home/yoojinhyung/.local/share/vibe-kanban /tmp/npm-cache
RUN chown -R yoojinhyung:users /repos /home/yoojinhyung/.local /tmp/npm-cache

# vibe-kanban 시작 스크립트 복사 및 권한 설정 (root로)
COPY start-vibe.sh /usr/local/bin/start-vibe.sh
RUN chmod +x /usr/local/bin/start-vibe.sh

# 사용자 전환
USER yoojinhyung

# NPM 캐시 설정 (사용자 권한으로)
ENV NPM_CONFIG_CACHE=/tmp/npm-cache
ENV NPM_CONFIG_YES=true
ENV NPM_CONFIG_PREFIX=/home/yoojinhyung/.npm-global
ENV PATH=/home/yoojinhyung/.npm-global/bin:$PATH

# 사용자 권한으로 NPM 패키지 설치
RUN npm install -g @anthropic-ai/claude-code
RUN npm install -g @google/gemini-cli
RUN npm install -g vibe-kanban

# 작업 디렉토리 지정
WORKDIR /home/yoojinhyung

# 기본 쉘
CMD ["bash"]