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
 * MODULE:  mcd_loader_class.cpp
 * VERSION: $Revision: 1.4 $ $Date: 2012/04/11 17:45:37 $
 ******************************************************************************
 * DESCRIPTION:
 * Implementation of utility functions of MCD Loader Class             
 ******************************************************************************/

#include "mcd_loader_class.h"

#include "mcd_tools.h"

#include <assert.h>
#include <stdio.h>

//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::mAccess(mcd_core_st *core, const mcd_addr_st *addr, void *value, 
                                      uint32_t n_bytes, mcd_tx_access_type_et access_type)
{
  mcd_tx_st     tx;
  mcd_txlist_st txlist;
  txlist.tx     = &tx;
  txlist.num_tx = 1;

  memcpy(&tx.addr, addr, sizeof(mcd_addr_st));
  tx.data        = (uint8_t*) value;
  tx.num_bytes   = n_bytes;
  tx.access_type = access_type;
  tx.options     = MCD_TX_OPT_DEFAULT;

  return mcd_execute_txlist_f(core, &txlist);
}



//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::qry_all_devices(uint32_t *num_devices, mcd_core_con_info_st *device_con_info)
{
  mcd_return_et ret; 
  uint32_t  s, n, numSys, numDev, numDevParam, numDevOfThisSys;
  mcd_core_con_info_st  coreConInfoThisSys;

  numDevParam  = *num_devices;
  *num_devices = 0; // Default in case of errors

  numSys = 0;
  ret = mcd_qry_systems_f(0, &numSys, 0);
  if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
  if (numSys == 0) {
    return MCD_RET_ACT_NONE;
  }

  numDev = 0;
  for (s = 0; s < numSys; s++) {

    n = 1;
    ret = mcd_qry_systems_f(s, &n, &coreConInfoThisSys);
    if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
    assert(n == 1);

    numDevOfThisSys = 0;
    ret = mcd_qry_devices_f(&coreConInfoThisSys, 0, &numDevOfThisSys, 0);
    if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
    assert(numDevOfThisSys > 0);

    if (numDevParam == 0) {  // Just counting
      numDev += numDevOfThisSys;
    }
    else {
      if ((numDev + numDevOfThisSys) > numDevParam)
        numDevOfThisSys = numDevParam - numDev;

      ret = mcd_qry_devices_f(&coreConInfoThisSys, 0, &numDevOfThisSys, &device_con_info[numDev]);
      if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
 
      numDev += numDevOfThisSys;
      if (numDev == numDevParam)
        break;
    }
  }

  *num_devices = numDev;

  return MCD_RET_ACT_NONE;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::qry_all_cores(uint32_t *num_cores, mcd_core_con_info_st *core_con_info)
{
  mcd_return_et ret; 
  uint32_t  d, numDev, numCore, numCoreParam, numCoreOfThisDev;

  numCoreParam  = *num_cores;
  *num_cores = 0; // Default in case of errors

  numDev  = 0;
  ret = qry_all_devices(&numDev, 0);
  if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
  if (numDev == 0) {
    return MCD_RET_ACT_NONE;
  }

  const uint32_t maxNumDev = 256; // More than enough
  mcd_core_con_info_st  coreConInfoDev[maxNumDev];

  numDev = maxNumDev;
  ret = qry_all_devices(&numDev, coreConInfoDev);
  if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }

  numCore = 0;
  for (d = 0; d < numDev; d++) {

    numCoreOfThisDev = 0;
    ret = mcd_qry_cores_f(&coreConInfoDev[d], 0, &numCoreOfThisDev, 0);
    if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
    assert(numCoreOfThisDev > 0);

    if (numCoreParam == 0) {  // Just counting
      numCore += numCoreOfThisDev;
    }
    else {
      assert(numCore < numCoreParam);
      if ((numCore + numCoreOfThisDev) > numCoreParam)
        numCoreOfThisDev = numCoreParam - numCore;

      ret = mcd_qry_cores_f(&coreConInfoDev[d], 0, &numCoreOfThisDev, &core_con_info[numCore]);
      if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
 
      numCore += numCoreOfThisDev;
      if (numCore == numCoreParam)
        break;
    }
  }

  *num_cores = numCore;

  return MCD_RET_ACT_NONE;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::qry_servers_no_device(const char *host, uint32_t *num_servers, 
                                                    mcd_server_info_st *server_info)
{
  mcd_return_et ret; 
  uint32_t  sr, d, numSvParam, numSvRunning, numSvNoDev, numDev0, numDev1;

  numSvParam  = *num_servers;
  *num_servers = 0; // Default in case of errors

  // Get all running servers
  const unsigned numSvMax = 32;
  mcd_server_info_st serverInfo[numSvMax];
  numSvRunning = numSvMax;
  ret = mcd_qry_servers_f(host, true, 0, &numSvRunning, serverInfo);
  if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }

  // Get all devices
  numDev0 = 0;
  ret = qry_all_devices(&numDev0, 0);
  if (ret != MCD_RET_ACT_NONE) { assert(false); return ret; }
  assert(numDev0 < 1000); // Sanity check

  mcd_core_con_info_st *devConInfo = new mcd_core_con_info_st[numDev0];

  numDev1 = numDev0;
  qry_all_devices(&numDev1, devConInfo);
  assert(numDev1 == numDev0);

  numSvNoDev = 0;
  for (sr = 0; sr < numSvRunning; sr++) {
    for (d = 0; d < numDev1; d++) {
      if (strncmp(serverInfo[sr].acc_hw, devConInfo[d].acc_hw, MCD_UNIQUE_NAME_LEN) == 0) {
        break;  // for (d) - Device is connected
      }
    }
    if (d < numDev1)
      continue;  // for (sr) - Device is connected

    numSvNoDev++;

    if (numSvParam > 0) {
      if (numSvNoDev < numSvParam) {
        memcpy(&server_info[numSvNoDev-1], &serverInfo[sr], sizeof(mcd_server_info_st));
      } 
      else {
        assert(numSvNoDev == numSvParam);
        memcpy(&server_info[numSvNoDev-1], &serverInfo[sr], sizeof(mcd_server_info_st));
        break; // for (sr)
      }
    }
  }

  *num_servers = numSvNoDev;

  delete devConInfo;

  return ret;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::read   (mcd_core_st *core, const mcd_addr_st *addr, void *value,
                                      uint32_t n_bytes)
{
  return mAccess(core, addr, value, n_bytes, MCD_TX_AT_R); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::read8  (mcd_core_st *core, const mcd_addr_st *addr, uint8_t  *value)
{
  return mAccess(core, addr, value, 1, MCD_TX_AT_R); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::read16 (mcd_core_st *core, const mcd_addr_st *addr, uint16_t *value)
{
  return mAccess(core, addr, value, 2, MCD_TX_AT_R); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::read32 (mcd_core_st *core, const mcd_addr_st *addr, uint32_t *value)
{
  return mAccess(core, addr, value, 4, MCD_TX_AT_R); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::read64 (mcd_core_st *core, const mcd_addr_st *addr, uint64_t *value)
{
  return mAccess(core, addr, value, 8, MCD_TX_AT_R); // Assuming a little endian host
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::set_acc_hw_frequency(mcd_server_st *server, uint32_t *frequ)
{
  if (mIsMcdxdasDllV140)
    return mSetAccHwFrequencyMcdxdasDllV140(server, frequ);

  mcd_return_et ret;
  char configString[64];
  int32_t frequNew;

  assert(strstr(server->config_string, "McdAccHw") != NULL);  // Makes only sense for Real HW

  // Set Access HW frequency with server config string
  sprintf(configString, "McdAccHw.Frequency=%d", *frequ);
  ret = mcd_set_server_config_f(server, configString);
  if (ret == MCD_RET_ACT_NONE) {
    bool succ = mcdt_extract_param_int32(server->config_string, "McdAccHw.Frequency", &frequNew);
    assert(succ);
    if (succ)
      *frequ = frequNew;
  }
 
  return ret;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::mSetAccHwFrequencyMcdxdasDllV140(mcd_server_st *server, uint32_t *frequ)
{
  mcd_return_et ret;

  char accHw[MCD_UNIQUE_NAME_LEN];
  bool succ = mcdt_extract_param(server->config_string, "McdAccHw", accHw);
  assert(succ);

  uint32_t i, numCores = 64;
  mcd_core_con_info_st coreConInfo[64];
  
  ret = qry_all_cores(&numCores, coreConInfo);
  if (ret != MCD_RET_ACT_NONE)
    return ret;

  for (i = 0; i < numCores; i++) {
    if (strncmp(coreConInfo[i].acc_hw, accHw, MCD_UNIQUE_NAME_LEN) == 0)
      break;
  }
  if (i < numCores) {

    mcd_core_st *core;
    ret = mcd->mcd_open_core_f(&coreConInfo[i], &core);
    if (ret != MCD_RET_ACT_NONE)
      return ret;
 
    // Using some magic...
    mcd_addr_st addr;
    memset(&addr, 0, sizeof(mcd_addr_st));
    addr.address      = 0x000C0100;
    addr.addr_space_id = 0xDADADA84;

    mcd_return_et ret;
    ret = mcd->write32(core, &addr, *frequ);
    assert(ret == MCD_RET_ACT_NONE);

    uint32_t frequNew;
    ret = mcd->read32(core, &addr, &frequNew);
    assert(ret == MCD_RET_ACT_NONE);

    ret = mcd->mcd_close_core_f(core);
    assert(ret == MCD_RET_ACT_NONE);

    *frequ = frequNew;
  }

  return ret;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et McdLoaderClass::write  (mcd_core_st *core, const mcd_addr_st *addr, void *value, 
                           uint32_t n_bytes)
{
  return mAccess(core, addr, value, n_bytes, MCD_TX_AT_W); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::write8 (mcd_core_st *core, const mcd_addr_st *addr, uint8_t  value)
{
  return mAccess(core, addr, &value, 1, MCD_TX_AT_W); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::write16(mcd_core_st *core, const mcd_addr_st *addr, uint16_t value)
{
  return mAccess(core, addr, &value, 2, MCD_TX_AT_W); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::write32(mcd_core_st *core, const mcd_addr_st *addr, uint32_t value)
{
  return mAccess(core, addr, &value, 4, MCD_TX_AT_W); // Assuming a little endian host
}

mcd_return_et McdLoaderClass::write64(mcd_core_st *core, const mcd_addr_st *addr, uint64_t value)
{
  return mAccess(core, addr, &value, 8, MCD_TX_AT_W); // Assuming a little endian host
}


