TOPDIR  := $(shell cd ..; cd ..; pwd)
include $(TOPDIR)/Rules.make

APP = erase

all: $(APP)

$(APP): main.c	
	$(CC) main.c -o $(APP) $(CFLAGS)
	
clean:
	-rm -f *.o ; rm $(APP)
