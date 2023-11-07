# Stage 1 - Generate a recipe file for dependencies
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2 - Build dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3 - Build app
# Use a Rust base image
FROM rust as builder

# Create a non-root user
ENV USER=myuser
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "$(pwd)" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Copy the current directory into the container
COPY . /app
WORKDIR /app

# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build for release
RUN cargo build --release

# Size optimization
RUN strip target/release/rust-web-docker

# Stage 4 - Final image
# Runtime image
FROM rust

# Import the user
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy executable to base image
COPY --from=builder /app/target/release/rust-web-docker /app/rust-web-docker
WORKDIR /app

# Set the user
USER myuser:myuser

# Run the app
ENTRYPOINT [ "./rust-web-docker" ]