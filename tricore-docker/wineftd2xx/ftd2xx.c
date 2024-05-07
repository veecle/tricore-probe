/*
  Quick hack to wrap FTD2xx lib for linux
  such that it may be called from the Wine Windows Emulator

  revised:  brent@mbari.org  -- 5/25/23
*/

#include <stdlib.h>

#undef _WIN32

#include "xftd2xx.h"
#include "wine/debug.h"
WINE_DEFAULT_DEBUG_CHANNEL(ftd2xx);


BOOL WINAPI DllMain(
    HINSTANCE hinstDLL,  // handle to DLL module
    DWORD fdwReason,     // reason for calling function
    LPVOID lpReserved )  // reserved
{
    TRACE("reason=%d\n", fdwReason);
    switch(fdwReason) {
        case DLL_PROCESS_ATTACH: {
            TRACE("DLL_PROCESS_ATTACH\n");
            unsigned vendor = 0x058b;
            unsigned product = 0x0043;
            FT_STATUS result = FT_SetVIDPID(vendor, product);
            if (result){
                TRACE("FT_SetVIDPID failed: %d\n", result);
                return FALSE;
            }
            TRACE("Seeking vendor:product 0x%04x:0x%04x\n", vendor, product);
        }
        case DLL_PROCESS_DETACH: {
            TRACE("DLL_PROCESS_DETACH\n");
        }
        case DLL_THREAD_ATTACH: {
            TRACE("DLL_THREAD_ATTACH\n");
            unsigned vendor = 0x058b;
            unsigned product = 0x0043;
            FT_STATUS result = FT_SetVIDPID(vendor, product);
            if (result){
                TRACE("FT_SetVIDPID failed: %d\n", result);
                return FALSE;
            }
            TRACE("Seeking vendor:product 0x%04x:0x%04x\n", vendor, product);
        }
        case DLL_THREAD_DETACH: {
            TRACE("DLL_THREAD_DETACH\n");
        }
    }
    return TRUE;
}

FT_STATUS WINAPI FT_CreateDeviceInfoList(LPDWORD lpdwNumDevs)
{
    TRACE("lpdwNumDevs=%d\n", *lpdwNumDevs);
    FT_STATUS result = xFT_CreateDeviceInfoList(lpdwNumDevs);
    TRACE("returning %d, lpdwNumDevs=%d\n", result, *lpdwNumDevs);
    return result;
}

FT_STATUS WINAPI FT_GetDeviceInfoDetail (DWORD dwIndex, LPDWORD lpdwFlags,
                                    LPDWORD lpdwType,
                                    LPDWORD lpdwID, LPDWORD lpdwLocId,
                                    LPVOID pcSerialNumber, LPVOID pcDescription,
                                    FT_HANDLE* pftHandle)
{
    TRACE("dwIndex=%d, lpdwFlags=%d, lpdwType=%d, lpdwID=%d, lpdwLocId=%d, pcSerialNumber=%s, pcDescription=%s, pftHandle=%p, *pftHandle=%p\n", dwIndex, *lpdwFlags, *lpdwType, *lpdwID, *lpdwLocId, pcSerialNumber, pcDescription, pftHandle, *pftHandle);
    FT_STATUS result = xFT_GetDeviceInfoDetail(dwIndex, lpdwFlags, lpdwType, lpdwID, lpdwLocId, pcSerialNumber, pcDescription, pftHandle);
    TRACE("returning %d, dwIndex=%d, lpdwFlags=%d, lpdwType=%d, lpdwID=%d, lpdwLocId=%d, pcSerialNumber=%s, pcDescription=%s, pftHandle=%p, *pftHandle=%p\n",result, dwIndex, *lpdwFlags, *lpdwType, *lpdwID, *lpdwLocId, pcSerialNumber, pcDescription, pftHandle, *pftHandle);
    return result;
}

FT_STATUS WINAPI FT_Open(int deviceNumber, FT_HANDLE* pHandle)
{
    TRACE("deviceNumber=%d, pHandle=%p\n",deviceNumber, *pHandle);
    FT_STATUS result =xFT_Open(deviceNumber, pHandle);
    TRACE("returning %d, deviceNumber=%d, pHandle=%p\n", result,deviceNumber, *pHandle);
    return result;
}

FT_STATUS WINAPI FT_SetFlowControl(FT_HANDLE ftHandle, USHORT usFlowControl,
                               UCHAR uXon, UCHAR uXoff)
{
    TRACE("ftHandle=%p, usFlowControl=%d, uXon=%d, uXoff=%d\n", ftHandle, usFlowControl, uXon, uXoff);
    FT_STATUS result = xFT_SetFlowControl(ftHandle, usFlowControl, uXon, uXoff);
    TRACE("returning %d, ftHandle=%p, usFlowControl=%d, uXon=%d, uXoff=%d\n", result, ftHandle, usFlowControl, uXon, uXoff);
    return result;
}

FT_STATUS WINAPI FT_Close(FT_HANDLE ftHandle)
{
    TRACE("ftHandle=%p\n", ftHandle);
    FT_STATUS result =xFT_Close(ftHandle);
    TRACE("returning %d, ftHandle=%p\n", result, ftHandle);
    return result;
}

FT_STATUS WINAPI FT_Read(FT_HANDLE ftHandle, LPVOID lpBuffer,
                          DWORD nBufferSize, LPDWORD lpBytesReturned)
{
    TRACE("ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesReturned=%d\n", ftHandle, lpBuffer, nBufferSize,  *lpBytesReturned);
    FT_STATUS result = xFT_Read(ftHandle, lpBuffer, nBufferSize, lpBytesReturned);
    TRACE("returning %d, ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesReturned=%d\n", result, ftHandle, lpBuffer, nBufferSize, *lpBytesReturned);
    return result;
}


FT_STATUS WINAPI FT_Write(FT_HANDLE ftHandle, LPVOID lpBuffer,
                          DWORD nBufferSize, LPDWORD lpBytesWritten)
{
    TRACE("ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesWritten=%d\n", ftHandle, lpBuffer, nBufferSize,  *lpBytesWritten);
    FT_STATUS result = xFT_Write(ftHandle, lpBuffer, nBufferSize, lpBytesWritten);
    TRACE("returning %d, ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesWritten=%d\n", result, ftHandle, lpBuffer, nBufferSize, *lpBytesWritten);
    return result;
}


FT_STATUS WINAPI FT_ResetDevice(FT_HANDLE ftHandle)
{
    TRACE("ftHandle=%p,\n", ftHandle);
    FT_STATUS result = xFT_ResetDevice(ftHandle);
    TRACE("returning %d, ftHandle=%p\n", result, ftHandle);
    return result;
}


FT_STATUS WINAPI FT_SetTimeouts(FT_HANDLE ftHandle,
                                DWORD dwReadTimeout, DWORD dwWriteTimeout)
{
    TRACE("ftHandle=%p, dwReadTimeout=%d, dwWriteTimeout=%d\n", ftHandle, dwReadTimeout, dwWriteTimeout);
    FT_STATUS result =  xFT_SetTimeouts(ftHandle, dwReadTimeout, dwWriteTimeout);
    TRACE("returning %d, ftHandle=%p, dwReadTimeout=%d, dwWriteTimeout=%d\n", result, ftHandle, dwReadTimeout, dwWriteTimeout);
    return result;
}


FT_STATUS WINAPI FT_GetQueueStatus(FT_HANDLE ftHandle, DWORD *dwRxBytes)
{
    TRACE("ftHandle=%p, dwRxBytes=%d\n", ftHandle, *dwRxBytes);
    FT_STATUS result =  xFT_GetQueueStatus(ftHandle, dwRxBytes);
    TRACE("returning %d, ftHandle=%p, dwRxBytes=%d\n", result, ftHandle, *dwRxBytes);
    return result;
}


FT_STATUS WINAPI FT_SetLatencyTimer(FT_HANDLE ftHandle, UCHAR ucLatency)
{
    TRACE("ftHandle=%p, ucLatency=%d\n", ftHandle, ucLatency);
    FT_STATUS result =  xFT_SetLatencyTimer(ftHandle, ucLatency);
    TRACE("returning %d, ftHandle=%p, ucLatency=%d\n", result, ftHandle, ucLatency);
    return result;
}


FT_STATUS WINAPI FT_SetBitMode(FT_HANDLE ftHandle, UCHAR ucMask, UCHAR ucEnable)
{
    TRACE("ftHandle=%p, ucMask=%d, ucEnable=%d\n", ftHandle, ucMask, ucEnable);
    FT_STATUS result = xFT_SetBitMode(ftHandle, ucMask, ucEnable);
    TRACE("returning %d, ftHandle=%p, ucMask=%d, ucEnable=%d\n", result, ftHandle, ucMask, ucEnable);
    return result;
}


FT_STATUS WINAPI FT_SetUSBParameters(FT_HANDLE ftHandle,
                          ULONG ulInTransferSize, ULONG ulOutTransferSize)
{
    TRACE("ftHandle=%p, ulInTransferSize=%d, ulOutTransferSize=%d\n", ftHandle, ulInTransferSize, ulOutTransferSize);
    FT_STATUS result = xFT_SetUSBParameters(ftHandle, ulInTransferSize, ulOutTransferSize);
    TRACE("returning %d, ftHandle=%p, ulInTransferSize=%d, ulOutTransferSize=%d\n", result, ftHandle, ulInTransferSize, ulOutTransferSize);
    return result;
}


FT_STATUS WINAPI FT_SetChars (FT_HANDLE ftHandle,
   UCHAR uEventCh, UCHAR uEventChEn, UCHAR uErrorCh, UCHAR uErrorChEn)
{
    TRACE("ftHandle=%p, uEventCh=%d, uEventChEn=%d, uErrorCh=%d, uErrorChEn=%d\n", ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    FT_STATUS result = xFT_SetChars(ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    TRACE("returning %d, ftHandle=%p, uEventCh=%d, uEventChEn=%d, uErrorCh=%d, uErrorChEn=%d\n", result, ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    return result;
}

FT_STATUS FT_GetDriverVersion (FT_HANDLE ftHandle, LPDWORD lpdwDriverVersion)
{
    TRACE("ftHandle=%p, lpdwDriverVersion=0x%x\n", ftHandle, *lpdwDriverVersion);
    FT_STATUS result = xFT_GetDriverVersion(ftHandle, lpdwDriverVersion);
    TRACE("returning %d, ftHandle=%p, lpdwDriverVersion=0x%x\n", result, ftHandle, *lpdwDriverVersion);
    return result;
}

FT_STATUS WINAPI FT_GetLibraryVersion (LPDWORD lpdwDLLVersion)
{
    TRACE("lpdwDLLVersion=0x%x\n", *lpdwDLLVersion);
    FT_STATUS result = xFT_GetLibraryVersion (lpdwDLLVersion);
    TRACE("returning %d, lpdwDLLVersion=0x%x\n", result, *lpdwDLLVersion);
    return result;
}
