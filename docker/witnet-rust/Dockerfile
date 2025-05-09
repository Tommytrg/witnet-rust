FROM --platform=$TARGETPLATFORM ubuntu:noble

# Install basic environment dependencies
RUN apt update && apt install -y --no-install-recommends \
    ca-certificates \
    curl \
    netcat-traditional \
    jq

# Clean up apt packages so the docker image is as compact as possible
RUN apt clean && apt autoremove

# Set needed environment variables
ENV RUST_BACKTRACE=1
ARG WITNET_VERSION

# Copy context and cd into it
COPY / /tmp
WORKDIR /tmp

# Expose server ports
EXPOSE 21337
EXPOSE 21338
EXPOSE 11212

# Run the install script
RUN ["chmod", "+x", "./downloader.sh", "./ip_detector.sh", "./migrator.sh", "./runner.sh", "./executer.sh", "./grinch-recovery.sh"]
RUN ["./downloader.sh"]

# Set entry point (always gets executed)
ENTRYPOINT ["./runner.sh"]

# Set default command (can be overridden)
CMD ["node", "server"]
