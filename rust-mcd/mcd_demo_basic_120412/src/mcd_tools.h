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
 * MODULE:  mcd_tools.h
 * VERSION: $Revision: 1.19 $ $Date: 2012/04/10 17:45:12 $
 ******************************************************************************
 * DESCRIPTION:
 * MCD tool functions             
 ******************************************************************************/

#ifndef __mcd_tools_h_
#define __mcd_tools_h_

#include <stdio.h>

#include "mcd_api.h"

bool mcdt_check_if_real_hw(const mcd_core_con_info_st *core_con_info);

//-------------------------------------------------------------------------------------------------
#define MCDT_ARRAY_LEN(a) (sizeof(a) / sizeof((a)[0]))

#define MCDT_ZERO(a) {(void) memset(&(a), 0, sizeof(a));}
#define MCDT_ONE(a) {(void) memset(&(a), 0xff, sizeof(a));}

#define MCDT_ARRAY_ZERO(a) {(void) memset(&(a)[0], 0, sizeof(a));}
#define MCDT_ARRAY_ONE(a) {(void) memset(&(a)[0], 0xff, sizeof(a));}

#define MCDT_MAX2(a,b) ((a) > (b) ? (a) : (b))
#define MCDT_MIN2(a,b) ((a) < (b) ? (a) : (b))

//-------------------------------------------------------------------------------------------------
// MCD Config String utilities
// Allocated sizeof(value) has to be MCD_UNIQUE_NAME_LEN
bool mcdt_extract_param(const char *config_string, const char *param_name, char *value);

bool mcdt_extract_param_int32(const char *config_string, const char *param_name, int32_t *value);


//-------------------------------------------------------------------------------------------------
int mcdt_compare_mcd_impl_version(const mcd_impl_version_info_st *mivi, uint16_t v_imp_major,
                                  uint16_t v_imp_minor, uint16_t v_imp_build);
// Returns 0 if equal, positive if mivi is higher and negative if mivi is lower



const char* mcdt_get_core_state_string(mcd_core_state_et core_state);

const char* mcdt_get_event_string(mcd_error_event_et event);

// Check if a server connection is used for a device or core connection
bool mcdt_check_if_server_used(const mcd_core_con_info_st *core_con_info, 
                               const char *server_config_string);


//-------------------------------------------------------------------------------------------------
#define MGT_PRINT_CORE_CON_INFO_TXT_BUF_SIZE 1024
void mcdt_print_core_con_info(char *txt_buf, const mcd_core_con_info_st *core_con_info,
                              const char *server_name = 0);
void mcdt_print_core_con_info(FILE *file, const mcd_core_con_info_st *core_con_info,
                              const char *server_name = 0);

void mcdt_print_mcd_impl_info(FILE *file, const mcd_impl_version_info_st *mivi);

void mcdt_print_memspace(FILE *file, const mcd_memspace_st *memspace);

void mcdt_print_reginfo(FILE *file, const mcd_register_info_st *reginfo);

void mcdt_print_server_info(FILE *file, uint32_t num_servers, const mcd_server_info_st *server_info);

void mcdt_print_trig_info(FILE *file, const mcd_trig_info_st *trig_info);

void mcdt_uint16_to_bytes(const uint16_t value, uint8_t * const bytes, const bool bigendian);

void mcdt_uint16_to_bytes_littleendian(const uint16_t value, uint8_t * const bytes);

void mcdt_uint16_to_bytes_bigendian(const uint16_t value, uint8_t * const bytes);

void mcdt_uint32_to_bytes(const uint32_t value, uint8_t * const bytes, const bool bigendian);

void mcdt_uint32_to_bytes_littleendian(const uint32_t value, uint8_t * const bytes);

void mcdt_uint32_to_bytes_bigendian(const uint32_t value, uint8_t * const bytes);

void mcdt_uint64_to_bytes(const uint64_t value, uint8_t * const bytes, const bool bigendian);

void mcdt_uint64_to_bytes_littleendian(const uint64_t value, uint8_t * const bytes);

void mcdt_uint64_to_bytes_bigendian(const uint64_t value, uint8_t * const bytes);

void mcdt_uint64_to_bytes_var(const uint64_t value, const uint32_t num_bytes, uint8_t * const bytes, const bool bigendian);
// Converts the specified 64-bit value to an array of num_bytes bytes, bytes[0] ... bytes[num_bytes]
// in a big- or little-endian fashion. That is, for 'bigendian' true, bytes[0] will contain the
// MSBs. If num_bytes is larger than 8, then the beginning (for big-endian) or end (for little)
// of the array will contain zeros.

uint16_t mcdt_bytes_to_uint16(const uint8_t * const bytes, const bool bigendian);

uint16_t mcdt_bytes_to_uint16_littleendian(const uint8_t * const bytes);

uint16_t mcdt_bytes_to_uint16_bigendian(const uint8_t * const bytes);

uint32_t mcdt_bytes_to_uint32(const uint8_t * const bytes, const bool bigendian);

uint32_t mcdt_bytes_to_uint32_littleendian(const uint8_t * const bytes);

uint32_t mcdt_bytes_to_uint32_bigendian(const uint8_t * const bytes);

uint64_t mcdt_bytes_to_uint64(const uint8_t * const bytes, const bool bigendian);

uint64_t mcdt_bytes_to_uint64_littleendian(const uint8_t * const bytes);

uint64_t mcdt_bytes_to_uint64_bigendian(const uint8_t * const bytes);

uint64_t mcdt_bytes_var_to_uint64(const uint32_t num_bytes, const uint8_t * const bytes, const bool bigendian);
// Converts the array bytes[0] ... bytes[num_bytes - 1] to an unsigned 64-bit value in a big- or little-endian
// fashion. That is, for 'bigendian' true, bytes[0] is considered to contain the MSBs.
// num_bytes may be zero, in which case the return value is zero as well.
// If num_bytes is larger than 8, then only the 64 LSBs of the true mathematical result
// will be returned.

uint64_t mcdt_get_millitime(void);
// Returns the number of milliseconds elapsed since January 1st 1970.

bool mcdt_all_zero(const size_t size, const uint8_t * data);
// Returns true if all bytes data[0] ... data[size - 1] are zero, including when 'size' equals zero.

uint64_t mcdt_ranges_overlap(const uint64_t a_start, const uint64_t a_len,
                             const uint64_t b_start, const uint64_t b_len,
                             uint64_t * const overlap_start = 0 );
// Returns the overlap between ranges 'a' and 'b', both specified in
// terms of a start value and a length. If the return value and overlap_start
// are non-zero, then the starting point of the overlap is assigned to
// *overlap_start.
//
// For example, if a_start is 100, a_len is 5, b_start is 102 and b_len
// is 12, the input ranges are [100, 104] and [102, 113]. In this case,
// the overlap is [102, 104], so the return value will be 3 and 102
// will optionally be assigned to *overlap_start.

bool mcdt_txlist_is_ok(const mcd_txlist_st * const txlist);
// Returns true if and only if all the num_tx|bytes_ok values are equal to the corresponding
// num_tx|bytes values (including when txlist->num_tx is zero).

bool mcdt_addrs_are_same(const mcd_addr_st * const a, const mcd_addr_st * const b);
// Returns true if and only if the two mcd_addr_st instances are exactly the same,
// i.e. if all four members are.

#endif // __mcd_tools_h_
