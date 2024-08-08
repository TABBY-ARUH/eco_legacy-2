FROM ubuntu:20.04

# Non-interactive frontend
ENV DEBIAN_FRONTEND=noninteractive

# Set timezone environment variables
ENV TZ=America/New_York

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    tzdata \
    libclang-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Ensure the timezone is set correctly
RUN ln -fs /usr/share/zoneinfo/$TZ /etc/localtime && dpkg-reconfigure --frontend noninteractive tzdata

# Install DFX non-interactively
RUN curl -o /tmp/dfx.sh https://internetcomputer.org/install.sh && \
    /bin/echo -e "y\n" | sh /tmp/dfx.sh

# Add DFX to PATH
ENV PATH="/root/.local/share/dfx/bin:${PATH}"

CMD ["/bin/bash"]

