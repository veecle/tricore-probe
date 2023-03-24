FROM ubuntu

RUN rm /etc/apt/sources.list
COPY ["sources.list", "/etc/apt/sources.list"]
RUN dpkg --add-architecture i386 && apt update && apt install -y wget
RUN mkdir -pm755 /etc/apt/keyrings
RUN wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key
RUN wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/focal/winehq-focal.sources
RUN apt update
RUN apt install -y --install-recommends wine
RUN apt install -y xvfb
RUN apt install -y binutils

WORKDIR "/root"
COPY ["pregenerated-wine.tar.gz", "."]
RUN tar -xf pregenerated-wine.tar.gz
RUN rm pregenerated-wine.tar.gz
RUN chown -R root:root /root/.wine

WORKDIR "/root/.wine/dosdevices/"
RUN ln -s ../drive_c /root/.wine/dosdevices/c\:
WORKDIR "/root/.wine/dosdevices/c:/"
RUN wine wineboot --init
    
COPY ["artifacts/ftd2xx.dll", "DAS64/servers/udas/"]
COPY ["artifacts/win-daemon.exe", "."]

ENV WINEDEBUG="-all"

CMD ["bash"]
