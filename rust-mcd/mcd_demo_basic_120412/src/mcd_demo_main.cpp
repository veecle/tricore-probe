/*****************************************************************************
 *
 * Copyright (C) 2007-2012 Infineon Technologies AG. All rights reserved.
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
 * MODULE:  mcd_demo_main.cpp
 * VERSION: $Revision: 1.14 $ $Date: 2012/04/12 14:24:11 $
 ******************************************************************************
 * DESCRIPTION: 
 * Simple example for the usage of the MCD API
 * Tested with TriCore and XMC4000 evaluation boards using DAS release V4.0.5.
 * DAS Installer (includes mcdxdas.dll) is available at www.infineon.com/DAS             
 ******************************************************************************/

#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "mcd_api.h"
#include "mcd_tools.h"
#include "mcd_loader_class.h"


//-------------------------------------------------------------------------------------------------
// Global MCD API loader class pointer
McdLoaderClass* mcd;

//-------------------------------------------------------------------------------------------------
void mcdd_get_core_ip_addr(mcd_core_st *core, mcd_register_info_st *core_ip_reg);

void mcdd_handle_err(FILE *lf, mcd_core_st **core, mcd_return_et ret);

mcd_return_et mcdd_read_block(mcd_core_st *core, const mcd_addr_st *addr, uint32_t num_bytes);

mcd_return_et mcdd_read_core_ip(mcd_core_st *core, const mcd_register_info_st *core_ip_reg, 
                                uint32_t *core_ip);

//-------------------------------------------------------------------------------------------------
// Server related functions
void mcdd_open_servers(const char *system_key, const char *config_string,
                       uint32_t *num_servers, mcd_server_st *server);

void mcdd_select_running_server(const char *host, const char *system_key, uint32_t *num_servers, 
                                mcd_server_st **server);

void mcdd_start_servers(const char *host, const char *system_key, uint32_t *num_servers, 
                        mcd_server_st **server);

mcd_return_et mcdd_set_acc_hw_frequency(mcd_server_st *server, uint32_t frequ);


//-------------------------------------------------------------------------------------------------
int main(int argc, char** argv) 
{
  mcd_return_et         ret, ret1;
  mcd_core_con_info_st  coreConInfo;
  uint32_t              i, sv, tmp, numOpenServers, numSystems;

  mcd_api_version_st versionReq;
  versionReq.v_api_major  = MCD_API_VER_MAJOR;
  versionReq.v_api_minor  = MCD_API_VER_MINOR;
  strcpy(versionReq.author, MCD_API_VER_AUTHOR);

  // Get name of MCD API lib
  char  lib_name[64];
  printf("\nEnter name of MCD API lib (0 for mcdxdas.dll):\n");
  scanf("%64s", lib_name);
  if (strlen(lib_name) == 1) { // Just one character -> mcdxdas.dll
    strcpy(lib_name, "mcdxdas.dll");
  }
  printf("\n");

  // Load and initialize MCD API 
  mcd_impl_version_info_st mcd_impl_info;
  assert (mcd == 0);
  mcd = new McdLoaderClass(lib_name);
  assert(mcd->lib_loaded());
  ret = mcd->mcd_initialize_f(&versionReq, &mcd_impl_info);
  assert(ret == MCD_RET_ACT_NONE);
  mcdt_print_mcd_impl_info(stdout, &mcd_impl_info);

  // Get IP address of server host
  char  host[64];
  printf("\nEnter IP address of server host (0 for localhost):\n");
  scanf("%64s", host);
  if (strlen(host) == 1) { // Just one character -> localhost
    strcpy(host, "localhost");
  }
  printf("\n");

  // System key
  printf("\nEnter system key (0 for none):\n");
  char system_key[MCD_KEY_LEN];
  scanf("%s", system_key);  // scanf() is not safe!
  if (strlen(system_key) <= 1) { // Just one character -> no key
    system_key[0] = 0;
  }

  // Query number of running servers and start server if none is running
  numOpenServers = 0;
  ret = mcd->mcd_qry_servers_f(host, TRUE, 0, &numOpenServers, 0);
  assert(ret == MCD_RET_ACT_NONE);

  const uint32_t maxNumServers = 16;
  mcd_server_st *openServers[maxNumServers];
  if (numOpenServers == 0) {
    numOpenServers = maxNumServers;
    mcdd_start_servers(host, system_key, &numOpenServers, openServers);
  }
  else {
    numOpenServers = maxNumServers;
    mcdd_select_running_server(host, system_key, &numOpenServers, openServers);
  }


  printf("\nSYSTEM LEVEL ##################################################################\n");

  // Number of systems
  numSystems = 0;
  ret = mcd->mcd_qry_systems_f(0, &numSystems, 0);
  assert(ret == MCD_RET_ACT_NONE);
  assert(numSystems >= 1);

  printf("Found %d systems on host %s\n\n", numSystems, host);

  for (i = 0; i < numSystems; i++) {
    tmp = 1;
    ret = mcd->mcd_qry_systems_f(i, &tmp, &coreConInfo);
    mcdt_print_core_con_info(stdout, &coreConInfo);
    printf("\n");
  }

  // Select system
  uint32_t  i_system = 0;
  if (numSystems > 1) {
    printf("\nEnter system index (0...%d):\n", numSystems - 1);
    scanf("%i", &i_system);
  }

  mcd_core_con_info_st core_con_info_system_common;
  tmp = 1;
  ret = mcd->mcd_qry_systems_f(i_system, &tmp, &core_con_info_system_common);
  assert(ret == MCD_RET_ACT_NONE);


  printf("\nDEVICE LEVEL ##################################################################\n");

  // Number of devices
  uint32_t num_devices = 0;
  ret = mcd->mcd_qry_devices_f(&core_con_info_system_common, 0, &num_devices, 0);
  assert(ret == MCD_RET_ACT_NONE);
  assert(num_devices > 0);


  printf("Found %d devices within system %s\n\n", num_devices, core_con_info_system_common.system);

  for (i = 0; i < num_devices; i++) {
    tmp = 1;
    ret = mcd->mcd_qry_devices_f(&core_con_info_system_common, i, &tmp, &coreConInfo);
    assert(ret == MCD_RET_ACT_NONE);
    mcdt_print_core_con_info(stdout, &coreConInfo);
    printf("\n");
  }

  // Select device
  uint32_t  i_device = 0;
  if (num_devices > 1) {
    printf("\nEnter device index (0...%d):\n", num_devices - 1);
    scanf("%i", &i_device);
  }

  mcd_core_con_info_st core_con_info_device_common;
  tmp = 1;
  ret = mcd->mcd_qry_devices_f(&core_con_info_system_common, i_device, &tmp, 
                          &core_con_info_device_common);
  assert(ret == MCD_RET_ACT_NONE);

  printf("\nCORE LEVEL ####################################################################\n");

  // Number of cores
  uint32_t num_cores = 0;  // Just to get the number of cores
  ret = mcd->mcd_qry_cores_f(&core_con_info_device_common, 0, &num_cores, 0);
  assert(ret == MCD_RET_ACT_NONE);
  assert(num_cores >= 1);

  printf("Found %d cores within device %s\n\n", num_cores, core_con_info_device_common.device);

  for (i = 0; i < num_cores; i++) {
    tmp = 1;
    ret = mcd->mcd_qry_cores_f(&core_con_info_device_common, i, &tmp, &coreConInfo);
    assert(ret == MCD_RET_ACT_NONE);
    assert(strcmp(coreConInfo.host, core_con_info_device_common.host) == 0);
    assert(strcmp(coreConInfo.system_key, core_con_info_device_common.system_key) == 0);
    assert(coreConInfo.device_key[0] == 0);  // Safe assumption for models
    assert(strcmp(coreConInfo.system, core_con_info_device_common.system) == 0);
    mcdt_print_core_con_info(stdout, &coreConInfo);
    printf("\n");
  }

  // Select core
  uint32_t  i_core = 0;
  if (num_cores > 1) {
    printf("\nEnter core index (0...%d):\n", num_cores - 1);
    scanf("%i", &i_core);
  }

  mcd_core_con_info_st core_con_info_core;
  tmp = 1;
  ret = mcd->mcd_qry_cores_f(&core_con_info_device_common, i_core, &tmp, &core_con_info_core);
  assert(ret == MCD_RET_ACT_NONE);

  // Open core
  mcd_core_st *core;
  ret = mcd->mcd_open_core_f(&core_con_info_core, &core);
  mcdd_handle_err(stdout, 0, ret);
  assert(core != NULL);

  mcd_register_info_st core_ip_reg;
  mcdd_get_core_ip_addr(core, &core_ip_reg);

  // Close not needed open servers
  mcd_server_st *server = NULL; // Needed to set frequency of Access HW
  for (sv = 0; sv < numOpenServers; sv++) {
    if (mcdt_check_if_server_used(&core_con_info_core, openServers[sv]->config_string)) {
      assert(server == NULL);
      server = openServers[sv];
      continue;
    }
    ret = mcd->mcd_close_server_f(openServers[sv]);
  }
  assert(server != NULL);

  // Key(s) for Locked Devices
  // It is assumed that a device type recognition (device type ID) is possible for a locked device.
  // Note that as a difference to silicon, device models are never locked (server key is sufficient).
  // The device key only needs to be provided with mcd_core_con_info_st, when the core is openend.

  uint32_t  value, core_ip;

  // Use strongest reset
  uint32_t rstClassVectorAvail, rstClassVector = 1; 
  ret = mcd->mcd_qry_rst_classes_f(core, &rstClassVectorAvail);
  assert(ret == MCD_RET_ACT_NONE);
  assert(rstClassVectorAvail & rstClassVector);

  // tx and txlist setup
  mcd_tx_st txDemo;
  memset(&txDemo, 0, sizeof(txDemo));  // Set all to default values
  mcd_txlist_st  txlistDemo;
  txlistDemo.tx     = &txDemo;
  txlistDemo.num_tx = 1;
  txDemo.num_bytes = sizeof(value);
  txDemo.data      = (uint8_t*) &value;

  // trig setup
  mcd_trig_simple_core_st trigDemo;
  memset(&trigDemo, 0, sizeof(mcd_trig_simple_core_st));  // Set all to default values
  trigDemo.struct_size = sizeof(mcd_trig_simple_core_st);
  trigDemo.type        = MCD_TRIG_TYPE_IP;
  trigDemo.action      = MCD_TRIG_ACTION_DBG_DEBUG;
  trigDemo.option      = MCD_TRIG_OPT_DEFAULT;
  trigDemo.addr_range  = 0;  // Single IP trigger

  printf("\n\nControl core:         run, stop, step <steps>\n");
  printf(    "Reset (and Halt):     rst, rsthlt\n");
  printf(    "Read 32 bit word:     read <addr>\n");
  printf(    "Read N bytes:         read <addr> <n_bytes>\n");
  printf(    "Write 32 bit word:    write <addr> <value>\n");
  printf(    "Set IP breakpoint:    bpt <addr> \n");
  printf(    "Clear IP breakpoint:  bpt\n");
  printf(    "Query state:          s\n");
  printf(    "Access HW frequency:  frequ <f_hz>\n");
  printf(    "Exit:                 exit\n\n");
  
  char      line[256], cmd[16], parStr0[64], parStr1[64];
  gets(line);  // Not clear why this is needed here to empty the input buffer?

  mcd_core_state_st  state;

  // Main loop
  while (true) {

    core_ip = 0xEEEEEEEE;
    state.state = MCD_CORE_STATE_UNKNOWN;

    if (core == NULL) {  // E.g. miniWiggler was unplugged
      // Trying to reconnect core
      ret = mcd->mcd_open_core_f(&core_con_info_core, &core);
      if (ret == MCD_RET_ACT_NONE)
        printf("Core successfully reconnected\n");
      else
        mcdd_handle_err(stdout, &core, ret);
    }

    if (core != NULL) {
      ret1 = mcd->mcd_qry_state_f(core, &state);

      if (ret1 != MCD_RET_ACT_HANDLE_ERROR)  // Avoid double notification
        ret1 = mcdd_read_core_ip(core, &core_ip_reg, &core_ip);

      mcdd_handle_err(stdout, &core, ret1);
    }

    printf("IP 0x%8.8X State %-8s Enter command: ", core_ip, mcdt_get_core_state_string(state.state)); 

    uint32_t  param0, param1, n_items;
    gets(line);  // Not safe I know...
    n_items = sscanf(line, "%s %s %s", cmd, parStr0, parStr1);

    if (core == NULL)
      continue; // Try to reconnect core in the beginning of the while loop

    if (n_items > 1) {
      if (strncmp(parStr0, "0x", 2) == 0)
        sscanf(parStr0, "%x", &param0);
      else
        sscanf(parStr0, "%d", &param0);
    }
    if (n_items > 2) {
      if (strncmp(parStr1, "0x", 2) == 0)
        sscanf(parStr1, "%x", &param1);
      else
        sscanf(parStr1, "%d", &param1);
    }

    if (strncmp(cmd, "exit", 8) == 0) {
      break;
    }
    else if (strncmp(cmd, "bpt", 8) == 0) {
      if (state.state != MCD_CORE_STATE_DEBUG) {
        printf("Breakpoints can be only changed in Debug state\n");
      }
      else {
        ret = mcd->mcd_remove_trig_set_f(core); 
        mcdd_handle_err(stdout, &core, ret);
        if (n_items > 1) { // Setup new IP breakpoint
          trigDemo.addr_start.address = param0;
          uint32_t trigId;
          ret = mcd->mcd_create_trig_f(core, &trigDemo, &trigId);
          mcdd_handle_err(stdout, &core, ret);
        }
      }
    }
    else if (strncmp(cmd, "read", 8) == 0) {
      if (n_items < 2) {
        printf("Syntax: read <addr>\n");
        continue;
      }

      txDemo.addr.address = param0;

      if (n_items == 3) {
        ret = mcdd_read_block(core, &txDemo.addr, param1);
        mcdd_handle_err(stdout, &core, ret);
      }
      else {
        value = 0xEEEEEEEE;   
        ret = mcd->read32(core, &txDemo.addr, &value); // Loader class utility function
        mcdd_handle_err(stdout, &core, ret);
        printf("Read value: 0x%8.8X\n", value);
      }
    }
    else if (strncmp(cmd, "rst", 8) == 0) {
      ret =  mcd->mcd_rst_f(core, rstClassVector, FALSE); 
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "rsthlt", 8) == 0) {
      ret =  mcd->mcd_rst_f(core, rstClassVector, TRUE); 
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "run", 8) == 0) {
      ret =  mcd->mcd_activate_trig_set_f(core); 
      mcdd_handle_err(stdout, &core, ret);
      ret =  mcd->mcd_run_f(core, FALSE); 
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "s", 8) == 0) {
      continue;
    }
    else if (strncmp(cmd, "stop", 8) == 0) {
      ret =  mcd->mcd_stop_f(core, FALSE); 
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "step", 8) == 0) {
      uint32_t n_steps = 1;
      if (n_items > 1)
        n_steps = param0;
      ret =  mcd->mcd_step_f(core, FALSE, MCD_CORE_STEP_TYPE_INSTR, n_steps); 
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "write", 8) == 0) {
      if (n_items < 3) {
        printf("Syntax: write <addr> <value>\n");
        continue;
      }
      txDemo.access_type  = MCD_TX_AT_W;
      txDemo.addr.address = param0;
      value               = param1;

      ret = mcd->mcd_execute_txlist_f(core, &txlistDemo);
      mcdd_handle_err(stdout, &core, ret);
    }
    else if (strncmp(cmd, "frequ", 8) == 0) {
      if (n_items < 2) {
        printf("Syntax: frequ <f_hz>\n");
        continue;
      }
      ret = mcdd_set_acc_hw_frequency(server, param0);
      mcdd_handle_err(stdout, NULL, ret);  // Note not core related
    }
    else {
      if (strlen(cmd) != 0) {
        printf("Unknown command\n");
      }
    }
  }

  // Close core
  ret = mcd->mcd_close_core_f(core);
  assert(ret == MCD_RET_ACT_NONE);

  // Cleanup
  mcd->mcd_exit_f(); // Enforce cleanup of all core and server connections 
  delete mcd;        // Unloads lib (destructor of McdLoaderClass)

  return 0;
}


//-------------------------------------------------------------------------------------------------
void mcdd_get_core_ip_addr(mcd_core_st *core, mcd_register_info_st *core_ip_reg)
{
  mcd_return_et ret;
  uint32_t reg_group_id, i, num_regs, num_regs_tmp;

  reg_group_id = 0; // Default (at least for TriCore, XMC4000, XC2000, XE166 and XC800)

  num_regs = 0; // Just query number
  ret = mcd->mcd_qry_reg_map_f(core, reg_group_id, 0, &num_regs, core_ip_reg);
  assert(ret == MCD_RET_ACT_NONE);

  for (i = 0; i < num_regs; i++) {
    num_regs_tmp = 1;
    ret = mcd->mcd_qry_reg_map_f(core, reg_group_id, i, &num_regs_tmp, core_ip_reg);
    assert(ret == MCD_RET_ACT_NONE);
    if (   (strcmp(core_ip_reg->regname, "PC") == 0) 
        || (strcmp(core_ip_reg->regname, "IP") == 0) ) {
      break;
    }
  }
  assert(i < num_regs);
}


//-------------------------------------------------------------------------------------------------
void mcdd_handle_err(FILE *lf, mcd_core_st **core, mcd_return_et ret)
{
  if (ret == MCD_RET_ACT_NONE)
    return;

  mcd_error_info_st errInfo;

  if (core == NULL)
    mcd->mcd_qry_error_info_f(NULL, &errInfo);
  else
    mcd->mcd_qry_error_info_f(*core, &errInfo);

  // Handle events
  if (errInfo.error_events & MCD_ERR_EVT_RESET)
    fprintf(lf, "EVENT: Target has been reset\n");
  if (errInfo.error_events & MCD_ERR_EVT_PWRDN)
    fprintf(lf, "EVENT: Target has been powered down\n");
  if (errInfo.error_events & MCD_ERR_EVT_HWFAILURE)
    fprintf(lf, "EVENT: There has been a target hardware failure\n");
  if (errInfo.error_events & ~7) { // Not MCD_ERR_EVT_RESET, _PWRDN, _HWFAILURE
    assert(false);
    fprintf(lf, "EVENT: There has been an unknown event\n");
  }

  if (ret == MCD_RET_ACT_HANDLE_EVENT) {
    return;  // Nothing to do
  }
  
  assert(ret == MCD_RET_ACT_HANDLE_ERROR);
  assert(errInfo.error_str[0] != 0);
  fprintf(lf, "ERROR: %s\n", errInfo.error_str);

  if (errInfo.error_code == MCD_ERR_CONNECTION) {  // E.g. miniWiggler was unplugged
    if ((core != NULL) && (*core != NULL)) {
      mcd->mcd_close_core_f(*core);
      *core = NULL;  // Will try to reconnect in main loop
    }
  }
}


//-------------------------------------------------------------------------------------------------
// The number of opened servers depends on how specific the config string is.
// E.g. in case of Real HW, whether it contains the name of the tool Access HW.
void mcdd_open_servers(const char *system_key, const char *config_string,
                           uint32_t *num_servers, mcd_server_st **server)
{
  mcd_return_et ret;
  uint32_t sv;

  printf("\nOpen Servers\n\n");

  for (sv = 0; sv < *num_servers; sv++) { 

    ret = mcd->mcd_open_server_f(system_key, config_string, &server[sv]);

    if (ret != MCD_RET_ACT_NONE) {
      assert(ret == MCD_RET_ACT_HANDLE_ERROR);
      mcd_error_info_st errInfo;
      mcd->mcd_qry_error_info_f(0, &errInfo);
      break;  // while
    }
    printf("%s\n", server[sv]->config_string);
  }

  *num_servers = sv;
}

//-------------------------------------------------------------------------------------------------
mcd_return_et mcdd_read_core_ip(mcd_core_st *core, const mcd_register_info_st *core_ip_reg, 
                                uint32_t *core_ip)
{
  mcd_return_et ret = MCD_RET_ACT_HANDLE_ERROR;

  switch (core_ip_reg->regsize) {
    case 32:
      ret = mcd->read32(core, &core_ip_reg->addr, core_ip);
      break;
    case 16:
      uint16_t core_ip16;
      ret = mcd->read16(core, &core_ip_reg->addr, &core_ip16);
      *core_ip = core_ip16;
    break;
    default:
      assert(false);
  }
  return ret;
}


//-------------------------------------------------------------------------------------------------
mcd_return_et mcdd_set_acc_hw_frequency(mcd_server_st *server, uint32_t frequ)
{
  mcd_return_et ret;

  uint32_t frequNew = frequ;
  ret = mcd->set_acc_hw_frequency(server, &frequNew);

  if (ret == MCD_RET_ACT_NONE)
    printf("Frequency set to %d kHz\n", frequNew/1000);
  else
    printf("Could not set frequency\n");
 
  return ret;
}


//-------------------------------------------------------------------------------------------------
// In mcdxdas.dll V1.4.0 the standard approach doesn't work due to an implementation bug
void mcdd_set_acc_hw_frequency_mcdxdas_v140_workaround(mcd_core_st *core, uint32_t frequ)
{
  // Using some magic...
  mcd_addr_st addr;
  memset(&addr, 0, sizeof(mcd_addr_st));
  addr.address      = 0x000C0100;
  addr.addr_space_id = 0xDADADA84;

  mcd_return_et ret;
  ret = mcd->write32(core, &addr, frequ);
  assert(ret == MCD_RET_ACT_NONE);

  uint32_t frequNew;
  ret = mcd->read32(core, &addr, &frequNew);
  assert(ret == MCD_RET_ACT_NONE);

  printf("Frequency set to %d kHz\n", frequNew/1000);
}


//-------------------------------------------------------------------------------------------------
void mcdd_select_running_server(const char *host, const char *system_key, uint32_t *num_servers, 
                                mcd_server_st **server)
{ 
  mcd_return_et ret;

  uint32_t i, sv, numRunningServers, notOnlyTheSelectedAccHw;
  const uint32_t maxNumServers = 16;
  mcd_server_info_st serverInfo[maxNumServers];

  numRunningServers = maxNumServers;
  ret = mcd->mcd_qry_servers_f(host, TRUE, 0, &numRunningServers, serverInfo);
  assert(ret == MCD_RET_ACT_NONE);

  printf("\nRunning Servers\n");
  mcdt_print_server_info(stdout, numRunningServers, serverInfo);
  printf("\n");

  if (numRunningServers > 1) {
    printf("\nEnter server index (0...%d):\n", numRunningServers - 1);
    scanf("%i", &sv);
    assert(sv < numRunningServers);

    // Check if there are different Access HWs for the same kind of server
    for (i = 0; i < numRunningServers; i++) {
      if (i == sv)
        continue;
      if (serverInfo[i].acc_hw[0] == 0)
        continue;
      if (strncmp(serverInfo[i].server, serverInfo[sv].server, MCD_UNIQUE_NAME_LEN) == 0)
        break;
    }
    if (i < numRunningServers) {
      printf("\nEnter 0 to open only the selected Access HW server:\n");
      scanf("%i", &notOnlyTheSelectedAccHw);
    }
  }
  else {
    assert(numRunningServers == 1);
    sv = 0;
  }

  char  configString[256];
  if (serverInfo[sv].acc_hw[0] != 0) {  // Real HW
    assert(serverInfo[sv].system_instance[0] == 0); 
    if (notOnlyTheSelectedAccHw)
      sprintf(configString, "McdHostName=\"%s\"\nMcdServerName=\"%s\" ", 
              host, serverInfo[sv].server);
    else
      sprintf(configString, "McdHostName=\"%s\"\nMcdServerName=\"%s\"\nMcdAccHw=\"%s\" ", 
              host, serverInfo[sv].server, serverInfo[sv].acc_hw);
  }
  else if (serverInfo[sv].system_instance[0] != 0) {  // Simulation model
    assert(serverInfo[sv].acc_hw[0] == 0); 
    sprintf(configString, "McdHostName=\"%s\"\nMcdServerName=\"%s\"\nMcdSystemInstance=\"%s\" ", 
            host, serverInfo[sv].server, serverInfo[sv].system_instance);
  } else {  // Not a good MCD API implementation
    assert(false);
    sprintf(configString, "McdHostName=\"%s\"\nMcdServerName=\"%s\" ", 
            host, serverInfo[sv].server);
  }

  mcdd_open_servers(system_key, configString, num_servers, server);
}


//-------------------------------------------------------------------------------------------------
void mcdd_start_servers(const char *host, const char *system_key, uint32_t *num_servers, 
                        mcd_server_st **server)
{
  mcd_return_et ret;
  uint32_t      sv, numInstalledServers;

  const uint32_t maxNumInstalledServers = 16;
  mcd_server_info_st serverInfo[maxNumInstalledServers];

  // Query installed servers
  numInstalledServers = maxNumInstalledServers;
  ret = mcd->mcd_qry_servers_f(host, FALSE, 0, &numInstalledServers, serverInfo);
  assert(ret == MCD_RET_ACT_NONE);

  printf("\nInstalled Servers:\n");
  mcdt_print_server_info(stdout, numInstalledServers, serverInfo);

  printf("\n\nEnter server index (0...%d) to start server\n", num_servers - 1);

  scanf("%i", &sv);

  assert(sv < numInstalledServers);

  char configString[128];
  sprintf(configString, "McdHostName=\"%s\"\nMcdServerName=\"%s\" ", host, serverInfo[sv].server);
  
  // In case of Real HW, servers for all different Access HWs will be openend
  // If several boards are connected, all devices will be available for the selection process
  mcdd_open_servers(system_key, configString, num_servers, server);
}


//-------------------------------------------------------------------------------------------------
mcd_return_et mcdd_read_block(mcd_core_st *core, const mcd_addr_st *addr, uint32_t num_bytes)
{
  mcd_return_et ret;

  uint32_t maxPayload;
  ret = mcd->mcd_qry_max_payload_size_f(core, &maxPayload);
  assert(ret == MCD_RET_ACT_NONE);
  if (ret != MCD_RET_ACT_NONE)
    return ret;

  // assert(maxPayload >= MCD_GUARANTEED_MIN_PAYLOAD); Not for mcdxdas.dll V4.0.5
  assert(num_bytes <= 204800); // 200 KB -> few seconds for 400 kHz JTAG/DAP clock

  uint8_t *data = new uint8_t[num_bytes];
    
  mcd_tx_st tx;

  mcd_txlist_st  txlist;
  txlist.tx     = &tx;
  txlist.num_tx = 1;

  // Prepare transactions
  memset(&tx, 0, sizeof(tx));  // Set all to default values
  memcpy(&tx.addr, addr, sizeof(mcd_addr_st));
  tx.access_type = MCD_TX_AT_R;
  tx.data        = data;

  int numBytesRemaining = num_bytes;
  while (numBytesRemaining > 0) {
    if (numBytesRemaining > (int)maxPayload)
      tx.num_bytes = maxPayload;
    else
      tx.num_bytes = numBytesRemaining;

    ret = mcd->mcd_execute_txlist_f(core, &txlist);
    
    numBytesRemaining -= tx.num_bytes_ok;

    if (ret != MCD_RET_ACT_NONE)
      break;

    tx.data           += tx.num_bytes;
    tx.addr.address   += tx.num_bytes;
  }

  printf("Read %d byte\n", num_bytes - numBytesRemaining);
 
  delete data;  // No usage for the data in this simple demo

  return ret;
}


