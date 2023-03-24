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
 * MODULE:  mcd_lc_basic.h
 * VERSION: $Revision: 1.3 $ $Date: 2012/04/09 17:18:57 $
 ******************************************************************************
 * DESCRIPTION:
 * MCD Loader Class with all the MCD API functions             
 ******************************************************************************/

#ifndef __mcd_lc_basic_h_
#define __mcd_lc_basic_h_


#include "mcd_api.h"
#include "mcd_lc_templates.h"

#define CB(x) x##(path, #x )

class McdLoaderClassBasic
{
public:
  McdLoaderClassBasic(const char* path)
    :CB(mcd_activate_trig_set_f)
    ,CB(mcd_chl_close_f)
    ,CB(mcd_chl_open_f)
    ,CB(mcd_chl_reset_f)
    ,CB(mcd_close_core_f)
    ,CB(mcd_close_server_f)
    ,CB(mcd_create_trig_f)
    ,CB(mcd_execute_command_f)
    ,CB(mcd_execute_txlist_f)
    ,CB(mcd_exit_f)
    ,CB(mcd_initialize_f)
    ,CB(mcd_open_core_f)
    ,CB(mcd_open_server_f)
    ,CB(mcd_qry_active_overlays_f)
    ,CB(mcd_qry_core_modes_f)
    ,CB(mcd_qry_cores_f)
    ,CB(mcd_qry_ctrigs_f)
    ,CB(mcd_qry_current_time_f)
    ,CB(mcd_qry_device_description_f)
    ,CB(mcd_qry_devices_f)
    ,CB(mcd_qry_error_info_f)
    ,CB(mcd_qry_max_payload_size_f)
    ,CB(mcd_qry_mem_blocks_f)
    ,CB(mcd_qry_mem_spaces_f)
    ,CB(mcd_qry_reg_compound_f)
    ,CB(mcd_qry_reg_groups_f)
    ,CB(mcd_qry_reg_map_f)
    ,CB(mcd_qry_rst_class_info_f)
    ,CB(mcd_qry_rst_classes_f)
    ,CB(mcd_qry_servers_f)
    ,CB(mcd_qry_state_f)
    ,CB(mcd_qry_systems_f)
    ,CB(mcd_qry_trace_state_f)
    ,CB(mcd_qry_traces_f)
    ,CB(mcd_qry_trig_info_f)
    ,CB(mcd_qry_trig_f)
    ,CB(mcd_qry_trig_set_f)
    ,CB(mcd_qry_trig_set_state_f)
    ,CB(mcd_qry_trig_state_f)
    ,CB(mcd_qry_server_config_f)
    ,CB(mcd_remove_trig_f)
    ,CB(mcd_remove_trig_set_f)
    ,CB(mcd_run_f)
    ,CB(mcd_run_until_f)
    ,CB(mcd_set_global_f)
    ,CB(mcd_set_server_config_f)
    ,CB(mcd_step_f)
    ,CB(mcd_stop_f)
    ,CB(mcd_read_trace_f)
    ,CB(mcd_receive_msg_f)
    ,CB(mcd_rst_f)
    ,CB(mcd_send_msg_f)
    ,CB(mcd_set_trace_state_f)
  {}

  bool lib_loaded() const
  {
    return mcd_initialize_f.good() && mcd_exit_f.good() && mcd_qry_servers_f.good();
  }

  LibCallback1<mcd_return_et, const mcd_core_st *>
    mcd_activate_trig_set_f;

  LibCallback2<mcd_return_et, const mcd_core_st *, const mcd_chl_st *>
    mcd_chl_close_f;

  LibCallback2<mcd_return_et, const mcd_core_st *, mcd_chl_st *>
    mcd_chl_open_f;

  LibCallback2<mcd_return_et, const mcd_core_st *, const mcd_chl_st *>
    mcd_chl_reset_f;

  LibCallback1<mcd_return_et, const mcd_core_st *>
    mcd_close_core_f;

  LibCallback1<mcd_return_et, const mcd_server_st *> 
    mcd_close_server_f;

  LibCallback3<mcd_return_et, const mcd_core_st *, void *, uint32_t *>
    mcd_create_trig_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, const char * , uint32_t , char * >
    mcd_execute_command_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, mcd_txlist_st *>
    mcd_execute_txlist_f;

  LibCallback0<void> 
    mcd_exit_f;

  LibCallback2<mcd_return_et, const mcd_api_version_st *, mcd_impl_version_info_st *>
    mcd_initialize_f;

  LibCallback2<mcd_return_et ,const mcd_core_con_info_st *, mcd_core_st **>
    mcd_open_core_f;

  LibCallback3<mcd_return_et, const char *, const char *, mcd_server_st **> 
    mcd_open_server_f;

  LibCallback2<mcd_return_et, const mcd_server_st *, const char *> 
    mcd_set_server_config_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, uint32_t *>
    mcd_qry_active_overlays_f;

  LibCallback4<mcd_return_et, const mcd_core_st *, uint32_t, uint32_t *, mcd_core_mode_info_st *>
    mcd_qry_core_modes_f;

  LibCallback4<mcd_return_et, const mcd_core_con_info_st *, uint32_t, uint32_t *, mcd_core_con_info_st *>
    mcd_qry_cores_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, mcd_ctrig_info_st *>
    mcd_qry_ctrigs_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, uint64_t *>
    mcd_qry_current_time_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, char *, uint32_t *>
    mcd_qry_device_description_f;

  LibCallback4<mcd_return_et, const mcd_core_con_info_st *, uint32_t, uint32_t *, mcd_core_con_info_st *>
    mcd_qry_devices_f;

  LibCallback2<void ,const mcd_core_st *, mcd_error_info_st *>
    mcd_qry_error_info_f;

  LibCallback3<mcd_return_et, const mcd_server_st *, uint32_t *, char *> 
    mcd_qry_server_config_f;

  LibCallback5<mcd_return_et, const char *, bool, uint32_t, uint32_t *, mcd_server_info_st *>
    mcd_qry_servers_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, mcd_core_state_st *>
    mcd_qry_state_f;

  LibCallback3<mcd_return_et, uint32_t , uint32_t*, mcd_core_con_info_st *>
    mcd_qry_systems_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, uint32_t *>
    mcd_qry_max_payload_size_f;

  LibCallback5<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t , uint32_t *, mcd_memblock_st *>
    mcd_qry_mem_blocks_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, mcd_memspace_st *>
    mcd_qry_mem_spaces_f;

  LibCallback5<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t , uint32_t *, uint32_t *>
    mcd_qry_reg_compound_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, mcd_register_group_st *>
    mcd_qry_reg_groups_f;

  LibCallback5<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t , uint32_t *, mcd_register_info_st *>
    mcd_qry_reg_map_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint8_t , mcd_rst_info_st *>
    mcd_qry_rst_class_info_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, uint32_t *>
    mcd_qry_rst_classes_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint32_t , mcd_trace_state_st *>
    mcd_qry_trace_state_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, mcd_trace_info_st *>
    mcd_qry_traces_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, mcd_trig_info_st *>
    mcd_qry_trig_info_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t , void *>
    mcd_qry_trig_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, uint32_t , uint32_t *, uint32_t * >
    mcd_qry_trig_set_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, mcd_trig_set_state_st *>
    mcd_qry_trig_set_state_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint32_t , mcd_trig_state_st *>
    mcd_qry_trig_state_f;

  LibCallback5<mcd_return_et ,const mcd_core_st *, const mcd_chl_st *, uint32_t , uint32_t *, uint8_t *>
    mcd_receive_msg_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint32_t , uint64_t >
    mcd_read_trace_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, uint32_t >
    mcd_remove_trig_f;

  LibCallback1<mcd_return_et ,const mcd_core_st *>
    mcd_remove_trig_set_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, bool >
    mcd_run_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, bool , bool , uint64_t >
    mcd_run_until_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint32_t , bool >
    mcd_rst_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, const mcd_chl_st *, uint32_t , const uint8_t *>
    mcd_send_msg_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, bool >
    mcd_set_global_f;

  LibCallback3<mcd_return_et ,const mcd_core_st *, uint32_t , mcd_trace_state_st *>
    mcd_set_trace_state_f;

  LibCallback4<mcd_return_et ,const mcd_core_st *, bool , mcd_core_step_type_et , uint32_t >
    mcd_step_f;

  LibCallback2<mcd_return_et ,const mcd_core_st *, bool >
    mcd_stop_f;

};


#endif //__mcd_lc_basic_h_
