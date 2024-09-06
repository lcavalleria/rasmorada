# base pre-built cross image
ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

# Download the latest flip-link release
RUN apt-get update && apt-get -y install wget
RUN wget -qO ./flip-link-installer https://github.com/knurling-rs/flip-link/releases/latest/download/flip-link-installer.sh
RUN chmod +x /flip-link-installer
#RUN ./flip-link-installer

# Set the working directory
WORKDIR /project
