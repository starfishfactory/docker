FROM node:latest

# 기본 유틸리티 설치
RUN apt-get update && \
    apt-get install -y screen curl gnupg lsb-release ca-certificates

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

# 작업 디렉토리 지정
WORKDIR /workspace

# 기본 쉘
CMD ["bash"]