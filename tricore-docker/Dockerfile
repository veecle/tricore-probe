FROM ubuntu:focal

RUN rm /etc/apt/sources.list
COPY ["sources.list", "/etc/apt/sources.list"]
RUN dpkg --add-architecture i386 && apt update && apt install -y wget
RUN mkdir -pm755 /etc/apt/keyrings
RUN wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key
RUN wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/focal/winehq-focal.sources
RUN apt update
# Virtual X server to make applications not crash without display.
RUN apt install -y xvfb
RUN apt install -y binutils
# Make etc.
RUN apt install -y build-essential
# Wine
RUN apt install -y  --install-recommends winehq-stable
# Mutli-architecture libraries required to build wineftd2xx.
RUN apt install -y  gcc-multilib
# Compiler for wineftd2xx.
RUN apt install -y  --install-recommends wine-stable-dev
# msiextract tool.
RUN apt install -y msitools

ARG AGREE_INFINEON_TERMS=0
RUN if [ "${AGREE_INFINEON_TERMS}" -ne "1" ]; then echo "This installation ships an AurixFlasher and DAS installation. Agree with these conditions by setting environment variable AGREE_INFINEON_TERMS to 1" && exit 1; fi

WORKDIR "/root"
# Copy DAS setup and install script.
COPY ["DAS_V8_0_5_SETUP.exe", "."]
COPY ["DAS_8_0_5_installer_script.qs", "."]

# Install DAS, this will accept the DAS license agreement, notice and terms of use.
RUN wine DAS_V8_0_5_SETUP.exe --script DAS_8_0_5_installer_script.qs -d --al

# Copy Aurix flasher setup.
COPY ["aurixflashersoftwaretool_1.0.10_Windows_x64.msi", "."]
# Extract Aurix flasher.
RUN msiextract -C /root/.wine/dosdevices/c:/AurixFlasher aurixflashersoftwaretool_1.0.10_Windows_x64.msi

# Install wine mono required for AurixFlasher.
RUN wget -O ./wine-mono-9.1.0-x86.msi https://dl.winehq.org/wine/wine-mono/9.1.0/wine-mono-9.1.0-x86.msi
RUN msiexec /i wine-mono-9.1.0-x86.msi

# Build FTD2XX DLL shim.
COPY ["wineftd2xx", "./wineftd2xx"]
WORKDIR "./wineftd2xx"
RUN make
RUN make install

WORKDIR "/root/.wine/dosdevices/c:/"
COPY ["artifacts/win-daemon.exe", "."]
# The DAS installer places TAS server executables in both directories, so we place the DLL in both directories.
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "Das/servers/ftd2xx.dll"
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "windows/system32/ftd2xx.dll"

# Wine debug channels https://wiki.winehq.org/Debug_Channels
# Turn on "+relay" for only one application: "win-daemon.exe:+relay"
# FTD2XX dll logging: "trace+ftd2xx"
# Disables all Wine debug logging.
ENV WINEDEBUG="-all"

# DAS home directory.
ENV DAS_HOME="C:\\Das"

CMD ["bash"]


