TOPDIR  := $(shell cd ..; cd ..; cd ..; pwd)
include $(TOPDIR)/Rules.make

APP = writeua

all: $(APP)

$(APP): main.c	
	$(CC) main.c -o $(APP) $(CFLAGS)
	
clean:
	-rm -f *.o ; rm $(APP)
