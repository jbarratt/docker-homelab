FROM rust:1.49 as builder

COPY ./sonnen-exporter /sonnen-exporter

RUN cd /sonnen-exporter && cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
		&& apt-get install -y ca-certificates tzdata \
	&& rm -rf /var/lib/apt/lists/*

EXPOSE 9422

COPY --from=builder /sonnen-exporter/target/release/sonnen-exporter /usr/local/bin/sonnen-exporter

CMD ["/usr/local/bin/sonnen-exporter", "/etc/sonnen-exporter/config.toml"]
