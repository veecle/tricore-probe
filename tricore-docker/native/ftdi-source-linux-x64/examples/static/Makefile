CC=gcc
UNAME := $(shell uname)
# Assume target is Mac OS if build host is Mac OS; any other host targets Linux
ifeq ($(UNAME), Darwin)
	DEPENDENCIES := -lpthread -lobjc -framework IOKit -framework CoreFoundation
else
	DEPENDENCIES := -lpthread -lrt
endif
CFLAGS=../../build/libftd2xx.a $(DEPENDENCIES) -Wall -Wextra
APP = static_link

all: $(APP)

$(APP): main.c	
	$(CC) main.c -o $(APP) $(CFLAGS)
	
clean:
	-rm -f *.o ; rm $(APP)
