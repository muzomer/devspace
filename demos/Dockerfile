FROM ghcr.io/charmbracelet/vhs AS base

RUN apt-get update && apt-get install -y build-essential curl git pkg-config libssl-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:$PATH"

COPY . /devspace/

WORKDIR /devspace/

RUN cargo install --path .

RUN git config --global user.email "devspace-user@email.com" \
    && git config --global user.email "devspace-user" \
    && mkdir /repositories_dir /worktrees_dir && for demo_repo in backend-repo frontend-repo infra-repo; do \
          git init /repositories_dir/$demo_repo ; \
          cd /repositories_dir/$demo_repo; \
          git commit --allow-empty -m "initial commit"; \
      done 
