/*****************************************************************************
 *
 * Copyright (C) 2010 Infineon Technologies AG. All rights reserved.
 *
 * Infineon Technologies AG (Infineon) is supplying this software for use with
 * Infineon's microcontrollers.  This file can be freely used for creating
 * development tools that are supporting such microcontrollers.
 *
 * THIS SOFTWARE IS PROVIDED "AS IS".  NO WARRANTIES, WHETHER EXPRESS, IMPLIED
 * OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
 * INFINEON SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR DIRECT, INDIRECT, 
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES, FOR ANY REASON 
 * WHATSOEVER.
 *
 ******************************************************************************
 * MODULE:  mcd_lc_utilities.h
 * VERSION: $Revision: 1.2 $ $Date: 2012/04/09 17:17:57 $
 ******************************************************************************
 * DESCRIPTION:
 * OS dependent Utilities for MCD Loader Class             
 ******************************************************************************/


#ifndef __mcd_lc_utilities_h_
#define __mcd_lc_utilities_h_

#ifdef WIN32

  #ifndef WIN32_LEAN_AND_MEAN
  #define WIN32_LEAN_AND_MEAN
  #endif
  #include <Windows.h>

#else

  #include <dlfcn.h>

  #define LoadLibrary(dllName)              dlopen(dllName, RTLD_LAZY | RTLD_GLOBAL)
  #define GetProcAddress(dllInst, config)   dlsym(dllInst, config)
  #define FreeLibrary(dllInst)              (!dlclose(dllInst))
  #define GetLibError()                     dlerror()

  #define HMODULE                           void*
  typedef int(*FARPROC)();

#endif 


#endif // __mcd_lc_utilities_h_

