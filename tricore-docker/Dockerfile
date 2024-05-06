FROM ubuntu:focal

RUN rm /etc/apt/sources.list
COPY ["sources.list", "/etc/apt/sources.list"]
RUN dpkg --add-architecture i386 && apt update && apt install -y wget
RUN mkdir -pm755 /etc/apt/keyrings
RUN wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key
RUN wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/focal/winehq-focal.sources
RUN apt update

RUN apt install -y xvfb
RUN apt install -y binutils
RUN apt install -y build-essential

RUN apt install -y  --install-recommends winehq-stable
RUN apt install -y  gcc-multilib
RUN apt install -y  --install-recommends wine-stable-dev


WORKDIR "/root"
# Copy DAS setup and install script.
COPY ["DAS_V8_0_5_SETUP.exe", "."]
COPY ["DAS_8_0_5_installer_script.qs", "."]

# Install DAS, this will accept the DAS license agreement, notice and terms of use.
RUN wine DAS_V8_0_5_SETUP.exe --script DAS_8_0_5_installer_script.qs -d --al

WORKDIR "/root"
COPY ["wineftd2xx", "./wineftd2xx"]
WORKDIR "./wineftd2xx"
RUN make
RUN make install


WORKDIR "/root/.wine/dosdevices/c:/"
COPY ["artifacts/win-daemon.exe", "."]
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "Das/servers/ftd2xx.dll"
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "windows/system32/ftd2xx.dll"

#ENV WINEDEBUG="+loaddll,trace+ftd2xx"
# Makes is work for some reason...
#ENV WINEDEBUG="trace+ftd2xx,+relay"
# Also makes stuff work:
#ENV WINEDEBUG="trace+ftd2xx,win-daemon.exe:+relay,das_dashpas.exe:+relay,UDAS_Console.exe:+relay"
# "trace+ftd2xx,das_dashpas.exe:+relay" -> crash
# "trace+ftd2xx,win-daemon.exe:+relay" -> crash
# "trace+ftd2xx,UDAS_Console.exe:+relay" -> works
#ENV WINEDEBUG="UDAS_Console.exe:+relay"
ENV WINEDEBUG="trace+ftd2xx"
#ENV WINEDEBUG="trace+ftd2xx,+seh,UDAS_Console.exe:+relay"

ENV DAS_HOME="C:\\Das"

ARG AGREE_INFINEON_TERMS=0
RUN if [ "${AGREE_INFINEON_TERMS}" -ne "1" ]; then echo "This installation ships a Infineon Memtool and DAS installation. Agree with these conditions by setting environment variabl AGREE_INFINEON_TERMS to 1" && exit 1; fi

CMD ["bash"]


