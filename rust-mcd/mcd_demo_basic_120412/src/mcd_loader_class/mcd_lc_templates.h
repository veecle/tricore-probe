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
 * MODULE:  mcd_lc_templates.h
 * VERSION: $Revision: 1.2 $ $Date: 2012/04/09 17:18:40 $
 ******************************************************************************
 * DESCRIPTION:
 * Templates for MCD Loader Class             
 ******************************************************************************/

#ifndef __mcd_lc_template_h_
#define __mcd_lc_template_h_

#include "mcd_lc_utilities.h"

template<class tRet> class LibCallback0
{
public:
	typedef tRet (*tCallback)(void);
	LibCallback0() : _hLib(NULL), call(NULL) {  }
	LibCallback0(const char * dllPath, const char *fName)    
	{ 
		LibCallback0();
		load(dllPath, fName);
	}
	virtual ~LibCallback0()	{ unload(); }
	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()() { return call(); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};

template<class tRet, class tParam> class LibCallback1
{
public:
	typedef tRet (*tCallback)(tParam);
	LibCallback1() : _hLib(NULL), call(NULL) {  }
	LibCallback1(const char * dllPath, const char *fName)    
	{ 
		LibCallback1();
		load(dllPath, fName);
	}
	virtual ~LibCallback1()	{ unload(); }
	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()(tParam v) { return call(v); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};


template<class tRet, class tParam1, class tParam2> class LibCallback2
{
public:
	typedef tRet (*tCallback)(tParam1, tParam2);
	LibCallback2() : _hLib(NULL), call(NULL) {  }
	LibCallback2(const char * dllPath, const char *fName)    
	{ 
		LibCallback2();
		load(dllPath, fName);
	}
	virtual ~LibCallback2()	{ unload(); }
	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()(tParam1 p1, tParam2 p2) { return call(p1, p2); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};


template<class tRet, class tParam1, class tParam2, class tParam3> class LibCallback3
{
public:
	typedef tRet (*tCallback)(tParam1, tParam2, tParam3);
	LibCallback3() : _hLib(NULL), call(NULL) {  }
	LibCallback3(const char * dllPath, const char *fName)    
	{ 
		LibCallback3();
		load(dllPath, fName);
	}
	virtual ~LibCallback3()	{ unload(); }
	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()(tParam1 p1, tParam2 p2, tParam3 p3) { return call(p1, p2, p3); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};

template<class tRet, class tParam1, class tParam2, class tParam3, class tParam4> class LibCallback4
{
public:
	typedef tRet (*tCallback)(tParam1, tParam2, tParam3, tParam4);
	LibCallback4() : _hLib(NULL), call(NULL) {  }
	LibCallback4(const char * dllPath, const char *fName)    
	{ 
		LibCallback4();
		load(dllPath, fName);
	}
	virtual ~LibCallback4()	{ unload(); }

	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()(tParam1 p1, tParam2 p2, tParam3 p3, tParam4 p4) { return call(p1, p2, p3, p4); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};

template<class tRet, class tParam1, class tParam2, class tParam3, class tParam4, class tParam5> class LibCallback5
{
public:
	typedef tRet (*tCallback)(tParam1, tParam2, tParam3, tParam4, tParam5);
	LibCallback5() : _hLib(NULL), call(NULL) {  }
	LibCallback5(const char * dllPath, const char *fName)    
	{ 
		LibCallback5();
		load(dllPath, fName);
	}
	virtual ~LibCallback5()	{ unload(); }

	bool load(const char * dllPath, const char *fName) 
	{
		call = ((_hLib = LoadLibrary(dllPath))!=NULL) ? (tCallback)GetProcAddress(_hLib, fName) : NULL;
		return good();
	}

	tRet operator()(tParam1 p1, tParam2 p2, tParam3 p3, tParam4 p4, tParam5 p5) { return call(p1, p2, p3, p4, p5); }
	void unload() { if(_hLib) FreeLibrary(_hLib); }
	bool good() const { return call!=NULL; }

private:
	tCallback call;
	HINSTANCE _hLib;
};

#endif //__mcd_lc_template_h_
