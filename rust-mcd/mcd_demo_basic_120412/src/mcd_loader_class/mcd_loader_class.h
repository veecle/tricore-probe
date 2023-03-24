/*****************************************************************************
 *
 * Copyright (C) 2010-2011 Infineon Technologies AG. All rights reserved.
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
 * MODULE:  mcd_loader_class.h
 * VERSION: $Revision: 1.4 $ $Date: 2012/04/11 17:45:04 $
 ******************************************************************************
 * DESCRIPTION:
 * MCD Loader Class with additional utility functions             
 ******************************************************************************/

#ifndef __mcd_loader_class_h_
#define __mcd_loader_class_h_

#include "mcd_lc_basic.h"

//-------------------------------------------------------------------------------------------------
class McdLoaderClass : public McdLoaderClassBasic
{
  public:

  // Constructor loads all MCD API functions
  McdLoaderClass(const char* path)
    :McdLoaderClassBasic(path) 
  { 
    if (strstr(path, "mcdxdas.dll") != NULL)
      mIsMcdxdasDll = true;
    else
      mIsMcdxdasDll = false;
  }

  // Overloaded functions
  mcd_return_et mcd_initialize_f(const mcd_api_version_st *version_req, mcd_impl_version_info_st *impl_info)
  {
    mcd_return_et ret = McdLoaderClassBasic::mcd_initialize_f(version_req, impl_info);
    memcpy(&mImplInfo, impl_info, sizeof(mcd_impl_version_info_st));
    if ((mIsMcdxdasDll == true) &&
        (impl_info->v_imp_major == 1)  && (impl_info->v_imp_minor == 4) && (impl_info->v_imp_build == 0))
      mIsMcdxdasDllV140 = true;
    else
      mIsMcdxdasDllV140 = false;
    return ret;
  }

  mcd_return_et mcd_qry_max_payload_size_f(const mcd_core_st *core, uint32_t *max_payload)
  {
    mcd_return_et ret = McdLoaderClassBasic::mcd_qry_max_payload_size_f(core, max_payload);
    if (mIsMcdxdasDllV140)
      *max_payload = 1024;  // Due to an implementation bug in this version of the DLL
    return ret;
  }

  // Additional utility functions

  // Basic access
  mcd_return_et read   (mcd_core_st *core, const mcd_addr_st *addr, void *value, uint32_t n_bytes);
  mcd_return_et read8  (mcd_core_st *core, const mcd_addr_st *addr, uint8_t  *value);
  mcd_return_et read16 (mcd_core_st *core, const mcd_addr_st *addr, uint16_t *value);
  mcd_return_et read32 (mcd_core_st *core, const mcd_addr_st *addr, uint32_t *value);
  mcd_return_et read64 (mcd_core_st *core, const mcd_addr_st *addr, uint64_t *value);
  mcd_return_et write  (mcd_core_st *core, const mcd_addr_st *addr, void *value, uint32_t n_bytes);
  mcd_return_et write8 (mcd_core_st *core, const mcd_addr_st *addr, uint8_t  value);
  mcd_return_et write16(mcd_core_st *core, const mcd_addr_st *addr, uint16_t value);
  mcd_return_et write32(mcd_core_st *core, const mcd_addr_st *addr, uint32_t value);
  mcd_return_et write64(mcd_core_st *core, const mcd_addr_st *addr, uint64_t value);

  // Query all devices of all systems (parameter definition as for mcd_qry_devices_f())
  mcd_return_et qry_all_devices(uint32_t *num_devices, mcd_core_con_info_st *device_con_info);

  // Query all cores of all devices of all systems (parameter definition as for mcd_qry_cores_f())
  mcd_return_et qry_all_cores(uint32_t *num_cores, mcd_core_con_info_st *core_con_info);

  // Query running servers without device connection (parameter definition as for mcd_qry_servers_f())
  mcd_return_et qry_servers_no_device(const char *host, uint32_t *num_servers, mcd_server_info_st *server_info);

  // Set the JTAG/DAP/SWD/etc. clock frequency of the Access HW. 
  // *frequ IN is the requested and *frequ OUT the actual set frequency [Hz].
  mcd_return_et set_acc_hw_frequency(mcd_server_st *server, uint32_t *frequ);

  const mcd_impl_version_info_st *qry_mcd_impl_version_info() { return &mImplInfo; }

  private:

  mcd_return_et mAccess(mcd_core_st *core, const mcd_addr_st *addr, void *value, 
                        uint32_t n_bytes, mcd_tx_access_type_et access_type);

  mcd_return_et mSetAccHwFrequencyMcdxdasDllV140(mcd_server_st *server, uint32_t *frequ);

  mcd_impl_version_info_st mImplInfo;

  bool mIsMcdxdasDll;

  bool mIsMcdxdasDllV140;  // Controls workarounds for mcd_qry_max_payload_size_f() and set_acc_hw_frequency()


};


//-------------------------------------------------------------------------------------------------
// Usually there is a global MCD API loader class pointer
extern McdLoaderClass* mcd;



#endif //__mcd_loader_class_h_
