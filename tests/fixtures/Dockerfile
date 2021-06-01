# syntax=docker/dockerfile:1.3

### Global build args
ARG TMUX_VERSION=3.2a
ARG TMUX_BUILD_DIR=/tmp/tmux-${TMUX_VERSION}

### Build stage
FROM alpine:3.14 as builder
ARG TMUX_VERSION
ARG TMUX_BUILD_DIR

RUN --mount=type=cache,target=/var/cache/apk ln -vs /var/cache/apk /etc/apk/cache && \
	apk add --update curl tar make g++ bison libevent-dev ncurses-dev pkgconfig

ARG TMUX_RELEASE_URL=https://github.com/tmux/tmux/releases/download/${TMUX_VERSION}/tmux-${TMUX_VERSION}.tar.gz
RUN mkdir -p ${TMUX_BUILD_DIR} && \
    curl -L ${TMUX_RELEASE_URL} | tar -xz --strip=1 -C ${TMUX_BUILD_DIR} && \
    cd ${TMUX_BUILD_DIR} && \
    ./configure && make

### Testcontainer
FROM alpine:3.14 as testcontainer
ARG TMUX_BUILD_DIR

RUN --mount=type=cache,target=/var/cache/apk ln -vs /var/cache/apk /etc/apk/cache && \
	apk add --update libevent ncurses bash

COPY --from=builder ${TMUX_BUILD_DIR}/tmux /usr/local/bin/tmux

ENTRYPOINT ["bash"]