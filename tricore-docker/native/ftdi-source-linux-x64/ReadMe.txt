D2XX for Linux
--------------

As Linux distributions vary these instructions are a guide to installation 
and use.  FTDI has tested the driver with LTS Ubuntu distributions for 
x86 and x86_64 architectures, and Raspbian on Raspberry Pi.

FTDI developed libftd2xx primarily to aid porting Windows applications 
written with D2XX to Linux.  We intend the APIs to behave the same on
Windows and Linux so if you notice any differences, please contact us 
(see http://www.ftdichip.com/FTSupport.htm).

FTDI do not release the source code for libftd2xx.  If you prefer to work
with source code and are starting a project from scratch, consider using
the open-source libFTDI.

libftd2xx uses an unmodified version of libusb (http://libusb.info) which
is distributed under the terms of the GNU Lesser General Public License 
(see libusb/COPYING or http://www.gnu.org/licenses).  Source code for 
libusb is included in this distribution.



Installing the D2XX shared library and static library.
------------------------------------------------------

1.  tar xfvz libftd2xx-x86_64-1.4.27.tgz

This unpacks the archive, creating the following directory structure:

    build
        libftd2xx        (re-linkable objects)
        libusb           (re-linkable objects)
        libftd2xx.a      (static library)
        libftd2xx.so.1.4.27   (dynamic library)
        libftd2xx.txt    (platform-specific information)
    examples
    libusb               (source code)
    ftd2xx.h
    WinTypes.h

2.  cd build

3.  sudo -s 
  or, if sudo is not available on your system: 
    su

Promotes you to super-user, with installation privileges.  If you're
already root, then step 3 (and step 7) is not necessary.

4.  cp libftd2xx.* /usr/local/lib

Copies the libraries to a central location.

5.  chmod 0755 /usr/local/lib/libftd2xx.so.1.4.27

Allows non-root access to the shared object.

6.  ln -sf /usr/local/lib/libftd2xx.so.1.4.27 /usr/local/lib/libftd2xx.so

Creates a symbolic link to the 1.4.27 version of the shared object.

7.  cd ..
    cp ftd2xx.h  /usr/local/include
    cp WinTypes.h  /usr/local/include
	
Copies the header files to a central location. 

8.  ldconfig -v

Update the linker shared object cache to include the newly installed library.

9.  exit

Ends your super-user session.

Notes on Kernel Built-in Support of FTDI devices.
-------------------------------------------------

On most distributions, the linux kernel will have either a built-in or optional
module called "ftdi_sio".  This will detect an FTDI device and automatically 
invoke the "usbserial" module and create devices such as "/dev/ttyUSB0".

When the ftdi_sio module is controlling an FTDI device it is not available to
libftd2xx.  If the library attempts to access the device it will receive a
message "FT_Open failed". 

There are several methods of preventing ftdi_sio from controlling FTDI devices.

1) Remove the ftdi_sio module from the running kernel:

        sudo lsmod | grep ftdi_sio

    If "ftdi_sio" is listed unload it (and its helper module, usbserial):

        sudo rmmod ftdi_sio
        sudo rmmod usbserial
        
    To reverse the operation the kernel modules can be reloaded using modprobe
    instead of rmmod.
    
2) Build a new kernel without the ftdi_sio module. 

    Refer to your distributions instructions for building a custom kernel.
    
3) Use a udev unbind sysfs interface to disable devices as they are connected.

    First identify the device identifier to remove.
        
        ls /sys/bus/usb/drivers/ftdi_sio
    
    This will show a list of ports on USB devices that are linked to the 
    ftdi_sio driver. 
    
    These are in the form 1-2:1.0 (bus number 1 - port number
    2 : device 1 . interface 0) for Port A, and 1-2:1.1 would be Port B on the 
    same device.
    
    The /dev/ttyUSBx node can be mapped to the device identifier using:
    
        lsusb -t
        
    The devices will be listed in a tree format which can be mapped to the 
    device identifiers.
    
    Identify the device which you wish to use and then send the identifier of the 
    device to the "unbind" interface in the driver. (The tee function just echos 
    stdin to stdout with privilege escalation through sudo).
    
        echo -n 1-2:1.1 | sudo tee /sys/bus/usb/drivers/ftdi_sio/unbind
    
    To reverse the process the same device number can be sent to the "bind"
    interface to re-enable the USB serial device. 
    
    This can be scripted through udev rules to happen when a device connection
    change is detected.


Adding Your Device's Vendor and Product Identifiers
---------------------------------------------------

If your FTDI device has a Vendor ID (VID) or Product ID (PID) that differs
from the standard devices then you can call FT_SetVIDPID before calling the
FT_Open, FT_OpenEx, FT_ListDevices or FT_CreateDeviceInfoList functions.

Modified VID and PIDs can also be added to the list of devices which invoke
ftdi_sio by sending the VID and PID of the device to the "new_id" interface.
The VID and PID can be found with lsusb.

    echo -n 0403 1001 | sudo tee /sys/bus/usb-serial/drivers/ftdi_sio/new_id

This can either be done after the device is installed or can be done before 
the device is inserted if an appropriate modprobe command is performed.


Building the shared-object examples.
------------------------------------

1.  cd examples

2.  make -B

This builds all the shared-object examples in subdirectories.

With an FTDI device connected to a USB port, try one of the 
examples, e.g. reading EEPROM.

3.  cd EEPROM/read

4.  sudo ./read


Building the static-library example.
------------------------------------

1.  cd examples/static

2.  rm lib*

Cleans out any existing libraries built for another target.

3.  cp /usr/local/lib/libftd2xx.a .

4.  make -B

5.  sudo ./static_link

This example demonstrates writing to, and reading from, a device with
a loop-back connector attached.



The examples show how to call a small subset of the D2XX API.  The full
API is available here:
http://www.ftdichip.com/Support/Documents/ProgramGuides/D2XX_Programmer%27s_Guide(FT_000071).pdf

