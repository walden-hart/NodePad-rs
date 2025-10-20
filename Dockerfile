FROM rust:1.90.0 as build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
RUN trunk build


FROM nginx:stable
COPY --from=build /usr/src/myapp/dist /usr/share/nginx/html