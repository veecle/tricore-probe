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
 * MODULE:  mcd_tools.cpp
 * VERSION: $Revision: 1.26 $ $Date: 2012/04/10 17:45:49 $
 ******************************************************************************
 * DESCRIPTION:
 * MCD tool functions             
 ******************************************************************************/

#include "mcd_api.h"
#include "mcd_tools.h"

#include <stdio.h>
#include <string.h>
#include <assert.h>

#include <sys/types.h>
#include <sys/timeb.h>


//-------------------------------------------------------------------------------------------------
bool mcdt_check_if_real_hw(const mcd_core_con_info_st *core_con_info)
{
  if (strcmp(core_con_info->system, "Real HW") == 0) {
    return true;
  }
  return false;
}

//-------------------------------------------------------------------------------------------------
bool mcdt_check_if_server_used(const mcd_core_con_info_st *core_con_info, 
                               const char *server_config_string)
{
  char paramValue[MCD_UNIQUE_NAME_LEN];
  
  if (!mcdt_extract_param(server_config_string, "McdHostName", paramValue)) {
    assert(false);
    return false;
  }
  if (strncmp(paramValue, core_con_info->host, MCD_UNIQUE_NAME_LEN) != 0)
    return false;

  if (mcdt_check_if_real_hw(core_con_info)) {  // Real HW
    if (!mcdt_extract_param(server_config_string, "McdAccHw", paramValue))
      return false;
    if (strncmp(paramValue, core_con_info->acc_hw, MCD_UNIQUE_NAME_LEN) != 0)
      return false;
  }
  else {  // Simulation model
    if (!mcdt_extract_param(server_config_string, "McdSystemInstance", paramValue))
      return false;
    if (strncmp(paramValue, core_con_info->system_instance, MCD_UNIQUE_NAME_LEN) != 0)
      return false;
  }

  return true; 
}

//-------------------------------------------------------------------------------------------------
int mcdt_compare_mcd_impl_version(const mcd_impl_version_info_st *mivi, uint16_t v_imp_major, 
                                  uint16_t v_imp_minor, uint16_t v_imp_build)
{
  // Assuming major and minor version numbers < 128
  uint32_t v_mivi  = (mivi->v_imp_major<<24) | (mivi->v_imp_minor<<16) | mivi->v_imp_build;
  uint32_t v_param = (      v_imp_major<<24) | (      v_imp_minor<<16) |       v_imp_build;
  return v_mivi - v_param;
}

//-------------------------------------------------------------------------------------------------
bool mcdt_extract_param(const char *config_string, const char *param_name, char *value)
{
  const char *pn;   // name
  const char *pv;   // value

  unsigned whitespace_len, token_len;

  if (value != NULL)
    value[0] = 0;  // Default in case of errors

  if (config_string == NULL) {
    return false;
  }
    
  pn = strstr(config_string, param_name);

  if (pn == NULL)
    return false;

  token_len = (unsigned) strlen(param_name);

  pv = pn + token_len;

  // Skip white space
  char white_space[] = " \t\n";

  whitespace_len = (unsigned) strspn(pv, white_space);

  if (whitespace_len > 64)  // Some reasonable value
    return false;

  pv = pv + whitespace_len;

  if (*pv != '=')  // Syntax error
    return false;

  pv++;  // Skip '=' character

  whitespace_len = (unsigned) strspn(pv, white_space);

  if (whitespace_len > 64)  // Some reasonable value
    return false;

  pv = pv + whitespace_len;

  if (*pv == '"') {
    pv++;  // Remove \" character
    token_len = (unsigned) strcspn(pv, "\"");
  }
  else {
    token_len = (unsigned) strcspn(pv, white_space);
  }

  if (token_len >= MCD_UNIQUE_NAME_LEN) {
    token_len = (MCD_UNIQUE_NAME_LEN - 1);
  }

  if (token_len > 0) {
    if (value != NULL) {
      memcpy(value, pv, token_len);
      *(value + token_len) = 0;
    }
    return true;
  }
  else {
    return false;
  }
}


//-------------------------------------------------------------------------------------------------
bool mcdt_extract_param_int32(const char *config_string, const char *param_name, int32_t *value)
{
  *value = 0xEEEEEEEE; // Default in case of errors

  char valueStr[MCD_UNIQUE_NAME_LEN];
  bool success = mcdt_extract_param(config_string, param_name, valueStr);
  if (success) {
    int nItems = sscanf(valueStr, "%i", value);
    if (nItems == 1)
      return true;  
  }
  return false;
}

//-------------------------------------------------------------------------------------------------
const char* mcdt_get_core_state_string(mcd_core_state_et core_state)
{
  switch (core_state) {
    case MCD_CORE_STATE_UNKNOWN:  return "Unknown";
    case MCD_CORE_STATE_RUNNING:  return "Running";
    case MCD_CORE_STATE_HALTED:   return "Halted";
    case MCD_CORE_STATE_DEBUG:    return "Debug";
  }
  if (   (core_state >= MCD_CORE_STATE_CUSTOM_LO) 
      && (core_state >= MCD_CORE_STATE_CUSTOM_HI) ) {
    return "Custom";
  }
  assert(false);
  return "ERROR Unknown Core State";
}


//-------------------------------------------------------------------------------------------------
const char* mcdt_get_event_string(mcd_error_event_et event)
{
  assert(event <= (MCD_ERR_EVT_HWFAILURE|MCD_ERR_EVT_RESET|MCD_ERR_EVT_PWRDN));
  switch (event) {
    case MCD_ERR_EVT_NONE:                        return "No event";

    case MCD_ERR_EVT_RESET:                       return "Reset";
    case MCD_ERR_EVT_PWRDN:                       return "Power down";
    case MCD_ERR_EVT_RESET|MCD_ERR_EVT_PWRDN:     return "Reset and Power Down";

    case MCD_ERR_EVT_HWFAILURE:                   return "HW Failure";
    case MCD_ERR_EVT_HWFAILURE|MCD_ERR_EVT_PWRDN: return "HW Failure and Power Down";
    case MCD_ERR_EVT_HWFAILURE|MCD_ERR_EVT_RESET: return "HW Failure and Reset";
    case MCD_ERR_EVT_HWFAILURE|MCD_ERR_EVT_RESET|MCD_ERR_EVT_PWRDN: 
                                                  return "HW Failure, Reset and Power Down";
    default:
      assert(false);
      return "ERROR Unknown Event";
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_core_con_info(FILE *file, const mcd_core_con_info_st *core_con_info,
                              const char *server_name)
{
  fprintf(file, "MCD Core Connection Info\n");

  char txtBuf[MGT_PRINT_CORE_CON_INFO_TXT_BUF_SIZE];

  mcdt_print_core_con_info(txtBuf, core_con_info, server_name);

  fprintf(file, "%s", txtBuf);
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_core_con_info(char *txt_buf, const mcd_core_con_info_st *cci, 
                              const char *server_name)
{
  unsigned t = 0;

  if (server_name != 0) {
    t += sprintf(txt_buf+t, "server:      %s\n", server_name);
  }
  if (cci->host[0] != 0) {
    t += sprintf(txt_buf+t, "host:        %s\n", cci->host);
  }
  if (cci->server_port != 0) {
    t += sprintf(txt_buf+t, "server_port: %d\n", cci->server_port);
  }
  if (cci->server_key[0] != 0) {
    t += sprintf(txt_buf+t, "server_key:  %s\n", cci->server_key);
  }
  if (cci->system_key[0] != 0) {
    t += sprintf(txt_buf+t, "system_key:  %s\n", cci->system_key);
  }
  if (cci->device_key[0] != 0) {
    t += sprintf(txt_buf+t, "device_key:  %s\n", cci->device_key);
  }
  if (cci->system[0] != 0) {
    t += sprintf(txt_buf+t, "system:      %s\n", cci->system);
  }
  if (cci->system_instance[0] != 0) {
    t += sprintf(txt_buf+t, "system_instance: %s\n", cci->system_instance);
  }
  if (cci->acc_hw[0] != 0) {
    t += sprintf(txt_buf+t, "acc_hw:      %s\n", cci->acc_hw);
  }
  if (cci->device_type != 0) {
    t += sprintf(txt_buf+t, "device_type: 0x%8.8X\n", cci->device_type);
  }
  if (cci->device[0] != 0) {
    t += sprintf(txt_buf+t, "device:      %s\n", cci->device);
  }
  if (cci->device_id != 0) {
    t += sprintf(txt_buf+t, "device_id:   0x%8.8X\n", cci->device_id);
  }
  if (cci->core[0] != 0) {
    t += sprintf(txt_buf+t, "core:        %s\n", cci->core);
  }
  if (cci->core_type != 0) {
    t += sprintf(txt_buf+t, "core_type:   %d\n", cci->core_type);
    t += sprintf(txt_buf+t, "core_id:     0x%8.8X\n", cci->core_id);
  }

  assert((t + 32) < MGT_PRINT_CORE_CON_INFO_TXT_BUF_SIZE);
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_memspace(FILE *file, const mcd_memspace_st *memspace)
{
  fprintf(file, "MCD Memory Space Info\n");
  fprintf(file, "  mem_space_id:    %d\n",      memspace->mem_space_id);
  fprintf(file, "  mem_space_name:  %s\n",      memspace->mem_space_name);

  if (memspace->mem_type != 0) {
    fprintf(file, "  mem_type:        0x%8.8X\n", memspace->mem_type);
    fprintf(file, "  bits_per_mau:    %d\n",      memspace->bits_per_mau);
    fprintf(file, "  invariance:      %d\n",      memspace->invariance);

    switch (memspace->endian) {
    case MCD_ENDIAN_LITTLE:  fprintf(file, "  endian:          LITTLE\n");  break;
    case MCD_ENDIAN_BIG:     fprintf(file, "  endian:          BIG\n");     break;
    case MCD_ENDIAN_DEFAULT: fprintf(file, "  endian:          DEFAULT\n"); break;
    default:                 fprintf(file, "  endian:          UNKNOWN\n"); assert(false);
    }
  
    fprintf(file, "  min_addr:        0x%8.8X", memspace->min_addr >> 32);
    fprintf(file, "%8.8X\n", (uint32_t)(memspace->min_addr));
    fprintf(file, "  max_addr:        0x%8.8X", memspace->max_addr >> 32);
    fprintf(file, "%8.8X\n", (uint32_t)(memspace->max_addr));

    assert(memspace->min_addr < memspace->max_addr);

    fprintf(file, "  num_mem_blocks:  %d\n", memspace->num_mem_blocks);

    fprintf(file, "  supported_access_options: 0x%8.8X\n", memspace->supported_access_options);
    fprintf(file, "  core_mode_mask_read:      0x%8.8X\n", memspace->supported_access_options);
    fprintf(file, "  core_mode_mask_write:     0x%8.8X\n", memspace->supported_access_options);
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_reginfo(FILE *file, const mcd_register_info_st *reginfo)
{
  fprintf(file, "MCD Register Info\n");
  fprintf(file, "  regname:      %s\n",    reginfo->regname);
  fprintf(file, "  reg_group_id: %d\n",    reginfo->reg_group_id);

  fprintf(file, "  address:      0x%8.8X", reginfo->addr.address >> 32);
  fprintf(file, "%8.8X\n", (uint32_t)(reginfo->addr.address));

  fprintf(file, "  mem_space_id: %d\n", reginfo->addr.mem_space_id);

  if (reginfo->addr.addr_space_type != MCD_NOTUSED_ID)
    fprintf(file, "  addr_space_type: %d\n", reginfo->addr.addr_space_type);
 
  fprintf(file, "  reg_group_id: %d\n", reginfo->reg_group_id);
  fprintf(file, "  regsize:      %d\n",      reginfo->regsize);

  if (reginfo->has_side_effects_read && reginfo->has_side_effects_write)
    fprintf(file, "  Has read and write side effects\n");
  else if (reginfo->has_side_effects_read)
    fprintf(file, "  Has read side effects\n");
  else if (reginfo->has_side_effects_write)
    fprintf(file, "  Has write side effects\n");
  else 
    fprintf(file, "  Has no side effects\n");

  if (reginfo->reg_type != MCD_REG_TYPE_SIMPLE)
    fprintf(file, "  reg_type:     %d\n", reginfo->reg_type);

  if (reginfo->hw_thread_id != 0)
    fprintf(file, "  hw_thread_id: %d\n", reginfo->hw_thread_id);
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_mcd_impl_info(FILE *file, const mcd_impl_version_info_st *mivi)
{
  fprintf(file, "MCD Implementation Info\n");
  fprintf(file, "  Vendor:      %s\n", mivi->vendor);
  fprintf(file, "  Version:     %d.%d.%d\n", mivi->v_imp_major, mivi->v_imp_minor, mivi->v_imp_build);
  fprintf(file, "  API version: %d.%d\n", mivi->v_api.v_api_major, mivi->v_api.v_api_minor);
  fprintf(file, "  Date:        %s\n\n", mivi->date);
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_server_info(FILE *file, uint32_t num_servers, const mcd_server_info_st *si)
{
  uint32_t sv;
  for (sv = 0; sv < num_servers; sv++) {
    if (si[sv].acc_hw[0] != 0) {
      printf("%d: %s, Access HW: %s\n", sv, si[sv].server, si[sv].acc_hw);
      assert(si[sv].system_instance[0] == 0);
    }
    else if (si[sv].system_instance[0] != 0) {
      printf("%d: %s, Instance: %s\n", sv, si[sv].server, si[sv].system_instance);
    }
    else {  // Installed servers
      printf("%d: %s\n", sv, si[sv].server);
    }
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_print_trig_info(FILE *file, const mcd_trig_info_st *ti)
{
  fprintf(file, "mcd_trig_info_st\n");
  fprintf(file, "  type:           0x%8.8X\n", ti->type);
  fprintf(file, "  option:         0x%8.8X\n", ti->option);
  fprintf(file, "  action:         0x%8.8X\n", ti->action);
  fprintf(file, "  trig_number:    %d\n", ti->trig_number);
  fprintf(file, "  state_number:   %d\n", ti->state_number);
  fprintf(file, "  counter_number: %d\n", ti->counter_number);
  if (ti->sw_breakpoints)
    fprintf(file, "  sw_breakpoints: true\n\n");
  else
    fprintf(file, "  sw_breakpoints: false\n\n");
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint16_to_bytes(const uint16_t value, uint8_t * const bytes, const bool bigendian)
{
  if (bigendian) {
    mcdt_uint16_to_bytes_bigendian(value, bytes);
  }
  else {
    mcdt_uint16_to_bytes_littleendian(value, bytes);
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint16_to_bytes_littleendian(const uint16_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) value;
   bytes[1] = (uint8_t) (value >> 8);
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint16_to_bytes_bigendian(const uint16_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) (value >> 8);
   bytes[1] = (uint8_t) value;
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint32_to_bytes(const uint32_t value, uint8_t * const bytes, const bool bigendian)
{
  if (bigendian) {
    mcdt_uint32_to_bytes_bigendian(value, bytes);
  }
  else {
    mcdt_uint32_to_bytes_littleendian(value, bytes);
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint32_to_bytes_littleendian(const uint32_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) value;
   bytes[1] = (uint8_t) (value >> 8);
   bytes[2] = (uint8_t) (value >> 16);
   bytes[3] = (uint8_t) (value >> 24);
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint32_to_bytes_bigendian(const uint32_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) (value >> 24);
   bytes[1] = (uint8_t) (value >> 16);
   bytes[2] = (uint8_t) (value >> 8);
   bytes[3] = (uint8_t) value;
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint64_to_bytes(const uint64_t value, uint8_t * const bytes, const bool bigendian)
{
  if (bigendian) {
    mcdt_uint64_to_bytes_bigendian(value, bytes);
  }
  else {
    mcdt_uint64_to_bytes_littleendian(value, bytes);
  }
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint64_to_bytes_littleendian(const uint64_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) value;
   bytes[1] = (uint8_t) (value >> 8);
   bytes[2] = (uint8_t) (value >> 16);
   bytes[3] = (uint8_t) (value >> 24);
   bytes[4] = (uint8_t) (value >> 32);
   bytes[5] = (uint8_t) (value >> 40);
   bytes[6] = (uint8_t) (value >> 48);
   bytes[7] = (uint8_t) (value >> 56);
}


//-------------------------------------------------------------------------------------------------
void mcdt_uint64_to_bytes_bigendian(const uint64_t value, uint8_t * const bytes)
{
   bytes[0] = (uint8_t) (value >> 56);
   bytes[1] = (uint8_t) (value >> 48);
   bytes[2] = (uint8_t) (value >> 40);
   bytes[3] = (uint8_t) (value >> 32);
   bytes[4] = (uint8_t) (value >> 24);
   bytes[5] = (uint8_t) (value >> 16);
   bytes[6] = (uint8_t) (value >> 8);
   bytes[7] = (uint8_t) value;
}

//-------------------------------------------------------------------------------------------------
void mcdt_uint64_to_bytes_var(const uint64_t value, const uint32_t num_bytes, uint8_t * const bytes, const bool bigendian)
{
  uint32_t byte_index;
  uint8_t byte;

  // Loop from least to most significant byte.
  for (byte_index = 0; byte_index < num_bytes; byte_index++) {

    // Determine byte with weight byte_index * 8.
    if (byte_index < 8) {
      byte = (uint8_t) (value >> (byte_index * 8));
    }
    else
    {
      // Not in 'value' anymore. Zero then.
      byte = 0;
    }
    bytes[bigendian ? (num_bytes - 1 - byte_index) : byte_index] = byte;
  }
  return;
}


//-------------------------------------------------------------------------------------------------
uint16_t mcdt_bytes_to_uint16(const uint8_t * const bytes, const bool bigendian)
{
  return bigendian
         ? mcdt_bytes_to_uint16_bigendian(bytes)
         : mcdt_bytes_to_uint16_littleendian(bytes);
}


//-------------------------------------------------------------------------------------------------
uint16_t mcdt_bytes_to_uint16_littleendian(const uint8_t * const bytes)
{
  uint16_t value;

  value = bytes[0];
  value += ((uint16_t) bytes[1]) << 8;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint16_t mcdt_bytes_to_uint16_bigendian(const uint8_t * const bytes)
{
  uint16_t value;

  value = bytes[1];
  value += ((uint16_t) bytes[0]) << 8;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint32_t mcdt_bytes_to_uint32(const uint8_t * const bytes, const bool bigendian)
{
  return bigendian
         ? mcdt_bytes_to_uint32_bigendian(bytes)
         : mcdt_bytes_to_uint32_littleendian(bytes);
}


//-------------------------------------------------------------------------------------------------
uint32_t mcdt_bytes_to_uint32_littleendian(const uint8_t * const bytes)
{
  uint32_t value;

  value = bytes[0];
  value += ((uint32_t) bytes[1]) << 8;
  value += ((uint32_t) bytes[2]) << 16;
  value += ((uint32_t) bytes[3]) << 24;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint32_t mcdt_bytes_to_uint32_bigendian(const uint8_t * const bytes)
{
  uint32_t value;

  value = bytes[3];
  value += ((uint32_t) bytes[2]) << 8;
  value += ((uint32_t) bytes[1]) << 16;
  value += ((uint32_t) bytes[0]) << 24;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_bytes_to_uint64(const uint8_t * const bytes, const bool bigendian)
{
  return bigendian
         ? mcdt_bytes_to_uint64_bigendian(bytes)
         : mcdt_bytes_to_uint64_littleendian(bytes);
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_bytes_to_uint64_littleendian(const uint8_t * const bytes)
{
  uint64_t value;

  value = bytes[0];
  value += ((uint64_t) bytes[1]) << 8;
  value += ((uint64_t) bytes[2]) << 16;
  value += ((uint64_t) bytes[3]) << 24;
  value += ((uint64_t) bytes[4]) << 32;
  value += ((uint64_t) bytes[5]) << 40;
  value += ((uint64_t) bytes[6]) << 48;
  value += ((uint64_t) bytes[7]) << 56;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_bytes_to_uint64_bigendian(const uint8_t * const bytes)
{
  uint64_t value;

  value = bytes[7];
  value += ((uint64_t) bytes[6]) << 8;
  value += ((uint64_t) bytes[5]) << 16;
  value += ((uint64_t) bytes[4]) << 24;
  value += ((uint64_t) bytes[3]) << 32;
  value += ((uint64_t) bytes[2]) << 40;
  value += ((uint64_t) bytes[1]) << 48;
  value += ((uint64_t) bytes[0]) << 56;

  return value;
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_bytes_var_to_uint64(const uint32_t num_bytes, const uint8_t * const bytes, const bool bigendian)
{
  uint64_t value;
  uint32_t byte_index;
  uint64_t byte;

  value = 0;

  // Note that if num_bytes is larger than 8, we must return the 64 LSBs of the
  // "true mathematical" result. For example, for 'bigendian' true we will ignore
  // bytes[0] ... bytes[num_bytes - 8], which contain the bits 64 and higher.
  //
  // Loop from least significant to most significant byte of end value.
  for (byte_index = 0; byte_index < MCDT_MIN2(num_bytes, 8); byte_index++) {
    byte = bytes[bigendian ? (num_bytes - 1 - byte_index) : byte_index];
    value += ((uint64_t) byte) << (byte_index * 8);
  }
  return value;
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_get_millitime(void)
{
  struct _timeb    t;
  uint64_t         r;

  _ftime(&t);

  r = 1000 * (uint64_t) t.time + (uint64_t) t.millitm;

  return r;
}


//-------------------------------------------------------------------------------------------------
bool mcdt_all_zero(const size_t size, const uint8_t * data)
{
  size_t k;
  bool   allZero = true;

  for (k = 0; k < size; k++) {
    if (data[k] != 0) {
      allZero = false;
      break;
    }
  }
  return allZero;
}


//-------------------------------------------------------------------------------------------------
uint64_t mcdt_ranges_overlap(const uint64_t a_start, const uint64_t a_len,
                             const uint64_t b_start, const uint64_t b_len,
                             uint64_t * const overlap_start )
{
  uint64_t overlap_len = 0;

  if (a_start >= b_start)
  {
    if (a_start < b_start + b_len) {
      overlap_len = MCDT_MIN2(a_len, b_start + b_len - a_start);
      if (overlap_start) {
        *overlap_start = a_start;
      }
    }
  }
  else {
    if (b_start < a_start + a_len) {
      overlap_len = MCDT_MIN2(b_len, a_start + a_len - b_start);
      if (overlap_start) {
        *overlap_start = b_start;
      }
    }
  }
  return overlap_len;
}


//-------------------------------------------------------------------------------------------------
bool mcdt_txlist_is_ok(const mcd_txlist_st * const txlist)
{
  bool               isOk;
  uint32_t           txIndex;
  const mcd_tx_st *  tx;

  if (txlist->num_tx_ok == txlist->num_tx) {
    isOk = true;

    for (txIndex = 0; txIndex < txlist->num_tx; txIndex++) {
      tx = &txlist->tx[txIndex];

      if (tx->num_bytes_ok != tx->num_bytes) {
        isOk = false;
        break;
      }
    }
  }
  else {
    isOk = false;
  }
  return isOk;
}


//-------------------------------------------------------------------------------------------------
bool mcdt_addrs_are_same(const mcd_addr_st * const a, const mcd_addr_st * const b)
{
  return    (a->address == b->address)
         || (a->mem_space_id == b->mem_space_id)
         || (a->addr_space_id == b->addr_space_id)
         || (a->addr_space_type == b->addr_space_type);
}
