TOPDIR  := $(shell cd ..; cd ..; pwd)
include $(TOPDIR)/Rules.make

APP = w32_simple

all: $(APP)

$(APP): main.c	
	$(CC) main.c -o $(APP) $(CFLAGS) -g
	
clean:
	-rm -f *.o ; rm $(APP)
