FROM rust:1-slim-trixie

# Python + pygame for the Python programs, and a JDK so java can run .java files directly
RUN apt-get update && apt-get install -y --no-install-recommends \
        python3 \
        python3-pygame \
        openjdk-21-jdk-headless \
        fontconfig \
        fonts-dejavu-core \
    && rm -rf /var/lib/apt/lists/*

# Use UTF-8 so emojis and special characters print properly instead of ????
ENV LANG=C.UTF-8

# No screen inside a container, so let pygame run without one instead of crashing
ENV SDL_VIDEODRIVER=dummy
ENV SDL_AUDIODRIVER=dummy

WORKDIR /app

# Build the dependencies first so Docker can cache them between builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Now copy the real code and build the menu
COPY src ./src
RUN touch src/main.rs && cargo build --release \
    && cp target/release/multi-game ./multi-game

CMD ["./multi-game"]
