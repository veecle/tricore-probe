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

//#define TRACE(...) //TRACE(__VA_ARGS__)

//a list of FTD product ids to access (in hexidecimal)
#define FTDprodVar "FTDID"


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


// Proxy
FT_STATUS WINAPI FT_CreateDeviceInfoList(LPDWORD lpdwNumDevs)
{
    TRACE("lpdwNumDevs=%d\n", *lpdwNumDevs);
    FT_STATUS result = xFT_CreateDeviceInfoList(lpdwNumDevs);
    TRACE("returning %d, lpdwNumDevs=%d\n", result, *lpdwNumDevs);
    return result;
}


FT_STATUS WINAPI FT_GetDeviceInfoList(FT_DEVICE_LIST_INFO_NODE *pDest,
                                   LPDWORD lpdwNumDevs)
{
  TRACE("\n");
	return xFT_GetDeviceInfoList(pDest, lpdwNumDevs);
}


// Proxy
FT_STATUS WINAPI FT_GetDeviceInfoDetail (DWORD dwIndex, LPDWORD lpdwFlags,
                                    LPDWORD lpdwType,
                                    LPDWORD lpdwID, LPDWORD lpdwLocId,
                                    LPVOID pcSerialNumber, LPVOID pcDescription,
                                    FT_HANDLE* pftHandle)
{
  TRACE("dwIndex=%d, lpdwFlags=%d, lpdwType=%d, lpdwID=%d, lpdwLocId=%d, pcSerialNumber=%s, pcDescription=%s, pftHandle=%p, *pftHandle=%p\n", dwIndex, *lpdwFlags, *lpdwType, *lpdwID, *lpdwLocId, pcSerialNumber, pcDescription, pftHandle, *pftHandle);
  FT_STATUS result = xFT_GetDeviceInfoDetail(dwIndex, lpdwFlags,
                                                           lpdwType, lpdwID, lpdwLocId,
                                                           pcSerialNumber, pcDescription,
                                                           pftHandle);

  TRACE("returning %d, dwIndex=%d, lpdwFlags=%d, lpdwType=%d, lpdwID=%d, lpdwLocId=%d, pcSerialNumber=%s, pcDescription=%s, pftHandle=%p, *pftHandle=%p\n",result, dwIndex, *lpdwFlags, *lpdwType, *lpdwID, *lpdwLocId, pcSerialNumber, pcDescription, pftHandle, *pftHandle);
  return result;
}


FT_STATUS WINAPI FT_ListDevices(PVOID pArg1, PVOID pArg2, DWORD flags)
{
    TRACE("pArg1=%p, pArg2=%p, flags=0x%x\n", pArg1, pArg2, flags);
    FT_STATUS result = xFT_ListDevices( pArg1,  pArg2,  flags);
    TRACE("returning %d, nDevs=%d\n", result, *(DWORD *)pArg1);
  return result;
}


// Proxy
FT_STATUS WINAPI FT_Open(int deviceNumber, FT_HANDLE* pHandle)
{
    TRACE("deviceNumber=%d, pHandle=%p\n",deviceNumber, *pHandle);
    FT_STATUS result =xFT_Open(deviceNumber, pHandle);
    TRACE("returning %d, deviceNumber=%d, pHandle=%p\n", result,deviceNumber, *pHandle);
    return result;
}


FT_STATUS WINAPI FT_OpenEx(PVOID pvArg1, DWORD dwFlags, FT_HANDLE* pHandle)
{
  TRACE("\n");
  return xFT_OpenEx(pvArg1, dwFlags, pHandle);
}


FT_STATUS WINAPI FT_SetBaudRate(FT_HANDLE ftHandle, DWORD dwBaudRate)
{
  TRACE("\n");
  return xFT_SetBaudRate(ftHandle, dwBaudRate);
}


FT_STATUS WINAPI FT_SetDivisor(FT_HANDLE ftHandle, USHORT usDivisor)
{
  TRACE("\n");
  return xFT_SetDivisor(ftHandle, usDivisor);
}


FT_STATUS WINAPI FT_SetDataCharacteristics(FT_HANDLE ftHandle,
                          UCHAR uWordLength, UCHAR uStopBits, UCHAR uParity)
{
  TRACE("\n");
  return xFT_SetDataCharacteristics(ftHandle, uWordLength, uStopBits, uParity);
}

// Proxy
FT_STATUS WINAPI FT_SetFlowControl(FT_HANDLE ftHandle, USHORT usFlowControl,
                               UCHAR uXon, UCHAR uXoff)
{
  TRACE("ftHandle=%p, usFlowControl=%d, uXon=%d, uXoff=%d\n", ftHandle, usFlowControl, uXon, uXoff);
  FT_STATUS result = xFT_SetFlowControl(ftHandle, usFlowControl, uXon, uXoff);
  TRACE("returning %d, ftHandle=%p, usFlowControl=%d, uXon=%d, uXoff=%d\n", result, ftHandle, usFlowControl, uXon, uXoff);
  return result;
}


FT_STATUS WINAPI FT_SetDtr(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_SetDtr(ftHandle);
}


FT_STATUS WINAPI FT_ClrDtr(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_ClrDtr(ftHandle);
}


FT_STATUS WINAPI FT_SetRts(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_SetRts(ftHandle);
}


FT_STATUS WINAPI FT_ClrRts(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_ClrRts(ftHandle);
}


FT_STATUS WINAPI FT_SetBreakOn(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_SetBreakOn(ftHandle);
}


FT_STATUS WINAPI FT_SetBreakOff(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_SetBreakOff(ftHandle);
}


FT_STATUS WINAPI FT_GetModemStatus(FT_HANDLE ftHandle, LPDWORD lpdwModemStatus)
{
  TRACE("\n");
  return xFT_GetModemStatus(ftHandle, lpdwModemStatus);
}


BOOL WINAPI FT_W32_GetCommModemStatus(FT_HANDLE ftHandle, LPDWORD lpdwStat)
{
  TRACE("\n");
  return xFT_W32_GetCommModemStatus(ftHandle, lpdwStat);
}

// Proxy
FT_STATUS WINAPI FT_Close(FT_HANDLE ftHandle)
{
  TRACE("ftHandle=%p\n", ftHandle);
  FT_STATUS result =xFT_Close(ftHandle);
  TRACE("returning %d, ftHandle=%p\n", result, ftHandle);
  return result;
}


FT_STATUS WINAPI FT_StopInTask(FT_HANDLE ftHandle)
{
  TRACE("FT_StopInTask\n");
  return xFT_StopInTask(ftHandle);
}


FT_STATUS WINAPI FT_RestartInTask(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_RestartInTask(ftHandle);
}


DWORD WINAPI FT_W32_GetLastError(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_W32_GetLastError(ftHandle);
}


BOOL WINAPI FT_W32_ClearCommError(FT_HANDLE ftHandle,
                            LPDWORD lpdwErrors, LPFTCOMSTAT lpftComstat)
{
  TRACE("\n");
  return xFT_W32_ClearCommError(ftHandle, lpdwErrors, lpftComstat);
}


BOOL WINAPI FT_W32_PurgeComm(FT_HANDLE ftHandle, DWORD dwFlags)
{
  TRACE("\n");
  return xFT_W32_PurgeComm(ftHandle, dwFlags);
}


BOOL WINAPI FT_W32_EscapeCommFunction(FT_HANDLE ftHandle, DWORD dwFunc)
{
  TRACE("\n");
  return xFT_W32_EscapeCommFunction(ftHandle, dwFunc);
}


BOOL WINAPI FT_W32_SetupComm(FT_HANDLE ftHandle,
                              DWORD dwReadBufferSize, DWORD dwWriteBufferSize)
{
  TRACE("\n");
  return xFT_W32_SetupComm(ftHandle, dwReadBufferSize, dwWriteBufferSize);
}


BOOL WINAPI FT_W32_WaitCommEvent(FT_HANDLE ftHandle, LPDWORD lpdwEvent,
                                  LPOVERLAPPED lpOverlapped)
{
  TRACE("\n");
  return xFT_W32_WaitCommEvent(ftHandle, lpdwEvent, lpOverlapped);
}


BOOL WINAPI FT_W32_SetCommMask(FT_HANDLE ftHandle, DWORD dwMask)
{
  TRACE("\n");
  return xFT_W32_SetCommMask(ftHandle, dwMask);
}


BOOL WINAPI FT_W32_SetCommBreak(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_W32_SetCommBreak(ftHandle);
}


BOOL WINAPI FT_W32_ClearCommBreak(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_W32_ClearCommBreak(ftHandle);
}


FT_HANDLE FT_W32_CreateFile(PVOID pvArg1, DWORD dwAccess, DWORD dwShareMode,
                                LPSECURITY_ATTRIBUTES lpSecurityAttributes,
                                DWORD dwCreate, DWORD dwAttrsAndFlags,
                                HANDLE hTemplate)
{
TRACE("\n");
  return xFT_W32_CreateFile(pvArg1, dwAccess, dwShareMode,
                                lpSecurityAttributes,
                                dwCreate, dwAttrsAndFlags,
                                hTemplate);
}


BOOL WINAPI FT_W32_CloseHandle(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_W32_CloseHandle(ftHandle);
}


BOOL WINAPI FT_W32_SetCommState(FT_HANDLE ftHandle, LPFTDCB lpftDcb)
{
  TRACE("\n");
  return xFT_W32_SetCommState(ftHandle, lpftDcb);
}


BOOL WINAPI FT_W32_GetCommState(FT_HANDLE ftHandle, LPFTDCB lpftDcb)
{
  TRACE("\n");
  return xFT_W32_GetCommState(ftHandle, lpftDcb);
}


BOOL WINAPI FT_W32_SetCommTimeouts(FT_HANDLE ftHandle, LPFTTIMEOUTS lpftTimeouts)
{
  TRACE("\n");
  return xFT_W32_SetCommTimeouts(ftHandle, lpftTimeouts);
}


BOOL WINAPI FT_W32_GetCommTimeouts(FT_HANDLE ftHandle, LPFTTIMEOUTS lpftTimeouts)
{
  TRACE("\n");
  return xFT_W32_GetCommTimeouts(ftHandle, lpftTimeouts);
}


BOOL WINAPI FT_W32_GetOverlappedResult(FT_HANDLE ftHandle,
  LPOVERLAPPED lpOverlapped, LPDWORD lpdwBytesTransferred, BOOL bWait)
{
  TRACE("\n");
  return xFT_W32_GetOverlappedResult(ftHandle,
              lpOverlapped, lpdwBytesTransferred, bWait);
}


BOOL WINAPI FT_W32_WriteFile(FT_HANDLE ftHandle, LPVOID lpBuffer,
   DWORD dwBytesToWrite, LPDWORD lpdwBytesWritten, LPOVERLAPPED lpOverlapped)
{
  TRACE("\n");
  return xFT_W32_WriteFile(ftHandle,
                  lpBuffer, dwBytesToWrite, lpdwBytesWritten, lpOverlapped);
}


BOOL WINAPI FT_W32_ReadFile(FT_HANDLE ftHandle, LPVOID lpBuffer,
    DWORD dwBytesToRead, LPDWORD lpdwBytesReturned, LPOVERLAPPED lpOverlapped)
{
  TRACE("\n");
  return xFT_W32_ReadFile(ftHandle,
                  lpBuffer, dwBytesToRead, lpdwBytesReturned, lpOverlapped);
}


FT_STATUS WINAPI FT_EE_UASize(FT_HANDLE ftHandle, DWORD *lpdwSize)
{
  TRACE("\n");
  return xFT_EE_UASize(ftHandle, lpdwSize);
}


FT_STATUS WINAPI FT_EE_UAWrite(FT_HANDLE ftHandle,
                                PUCHAR pucData, DWORD dwDataLen)
{
  TRACE("\n");
  return xFT_EE_UAWrite(ftHandle, pucData, dwDataLen);
}


FT_STATUS WINAPI FT_EE_UARead(FT_HANDLE ftHandle, unsigned char *pucData,
                              DWORD dwDataLen, DWORD *lpdwBytesRead)
{
  TRACE("\n");
  return xFT_EE_UARead(ftHandle, pucData, dwDataLen, lpdwBytesRead);
}


FT_STATUS WINAPI FT_EE_Program(FT_HANDLE ftHandle, PFT_PROGRAM_DATA pData)
{
  TRACE("\n");
  return xFT_EE_Program(ftHandle, pData);
}


FT_STATUS WINAPI FT_EE_Read(FT_HANDLE ftHandle, PFT_PROGRAM_DATA pData)
{
  TRACE("\n");
  return xFT_EE_Read(ftHandle, pData);
}


FT_STATUS WINAPI FT_EraseEE(FT_HANDLE ftHandle)
{
  TRACE("\n");
  return xFT_EraseEE(ftHandle);
}


FT_STATUS WINAPI FT_WriteEE(FT_HANDLE ftHandle, DWORD dwWordOffset, WORD wValue)
{
  TRACE("\n");
  return xFT_WriteEE(ftHandle, dwWordOffset, wValue);
}


FT_STATUS WINAPI FT_ReadEE(FT_HANDLE ftHandle, DWORD dwWordOffset, LPWORD lpwValue)
{
  TRACE("\n");
  return xFT_ReadEE(ftHandle, dwWordOffset, lpwValue);
}


FT_STATUS WINAPI FT_EE_ProgramEx(FT_HANDLE ftHandle, PFT_PROGRAM_DATA pData,
                    char *Manufacturer, char *ManufacturerId,
                    char *Description, char *SerialNumber)
{
  TRACE("\n");
  return xFT_EE_ProgramEx(ftHandle, pData,
                    Manufacturer, ManufacturerId,
                    Description, SerialNumber);
}


FT_STATUS WINAPI FT_EE_ReadEx(FT_HANDLE ftHandle, PFT_PROGRAM_DATA pData,
                    char *Manufacturer, char *ManufacturerId,
                    char *Description, char *SerialNumber)
{
  TRACE("\n");
  return xFT_EE_ReadEx(ftHandle, pData,
                    Manufacturer, ManufacturerId,
                    Description, SerialNumber);
}


FT_STATUS WINAPI FT_GetDeviceInfo(FT_HANDLE ftHandle, FT_DEVICE *lpftDevice,
      LPDWORD lpdwID, PCHAR SerialNumber, PCHAR Description,	LPVOID Dummy)
{
  TRACE("\n");
  return xFT_GetDeviceInfo(ftHandle, lpftDevice,
                            lpdwID, SerialNumber, Description, Dummy);
}

// Proxy
FT_STATUS WINAPI FT_Read(FT_HANDLE ftHandle, LPVOID lpBuffer,
                          DWORD nBufferSize, LPDWORD lpBytesReturned)
{
  TRACE("ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesReturned=%d\n", ftHandle, lpBuffer, nBufferSize,  *lpBytesReturned);
  FT_STATUS result = xFT_Read(ftHandle, lpBuffer, nBufferSize, lpBytesReturned);
  TRACE("returning %d, ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesReturned=%d\n", result, ftHandle, lpBuffer, nBufferSize, *lpBytesReturned);
  return result;
}

// Proxy
FT_STATUS WINAPI FT_Write(FT_HANDLE ftHandle, LPVOID lpBuffer,
                          DWORD nBufferSize, LPDWORD lpBytesWritten)
{
  TRACE("ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesWritten=%d\n", ftHandle, lpBuffer, nBufferSize,  *lpBytesWritten);
  FT_STATUS result = xFT_Write(ftHandle, lpBuffer, nBufferSize, lpBytesWritten);
  TRACE("returning %d, ftHandle=%p, lpBuffer=%p, nBufferSize=%d, lpBytesWritten=%d\n", result, ftHandle, lpBuffer, nBufferSize, *lpBytesWritten);
  return result;
}

// Proxy
FT_STATUS WINAPI FT_ResetDevice(FT_HANDLE ftHandle)
{
       TRACE("ftHandle=%p,\n", ftHandle);
       FT_STATUS result = xFT_ResetDevice(ftHandle);
       TRACE("returning %d, ftHandle=%p\n", result, ftHandle);
       return result;
}

// Proxy
FT_STATUS WINAPI FT_SetTimeouts(FT_HANDLE ftHandle,
                                DWORD dwReadTimeout, DWORD dwWriteTimeout)
{
    TRACE("ftHandle=%p, dwReadTimeout=%d, dwWriteTimeout=%d\n", ftHandle, dwReadTimeout, dwWriteTimeout);
    FT_STATUS result =  xFT_SetTimeouts(ftHandle, dwReadTimeout, dwWriteTimeout);
    TRACE("returning %d, ftHandle=%p, dwReadTimeout=%d, dwWriteTimeout=%d\n", result, ftHandle, dwReadTimeout, dwWriteTimeout);
    return result;
}


BOOL WINAPI FT_Purge(FT_HANDLE ftHandle, ULONG Mask)
{
  TRACE("\n");
  return xFT_Purge(ftHandle, Mask);
}

// Proxy
FT_STATUS WINAPI FT_GetQueueStatus(FT_HANDLE ftHandle, DWORD *dwRxBytes)
{
    TRACE("ftHandle=%p, dwRxBytes=%d\n", ftHandle, *dwRxBytes);
    FT_STATUS result =  xFT_GetQueueStatus(ftHandle, dwRxBytes);
    TRACE("returning %d, ftHandle=%p, dwRxBytes=%d\n", result, ftHandle, *dwRxBytes);
    return result;
}


FT_STATUS WINAPI FT_GetStatus(FT_HANDLE ftHandle,
                      DWORD *dwRxBytes, DWORD *dwTxBytes, DWORD *dwEventDWord)
{
  TRACE("\n");
  return xFT_GetStatus(ftHandle, dwRxBytes, dwTxBytes, dwEventDWord);
}

// Proxy
FT_STATUS WINAPI FT_SetLatencyTimer(FT_HANDLE ftHandle, UCHAR ucLatency)
{
    TRACE("ftHandle=%p, ucLatency=%d\n", ftHandle, ucLatency);
    FT_STATUS result =  xFT_SetLatencyTimer(ftHandle, ucLatency);
    TRACE("returning %d, ftHandle=%p, ucLatency=%d\n", result, ftHandle, ucLatency);
    return result;
}


FT_STATUS WINAPI FT_GetLatencyTimer(FT_HANDLE ftHandle, PUCHAR pucLatency)
{
  TRACE("\n");
  return xFT_GetLatencyTimer(ftHandle, pucLatency);
}

// Proxy
FT_STATUS WINAPI FT_SetBitMode(FT_HANDLE ftHandle, UCHAR ucMask, UCHAR ucEnable)
{
    TRACE("ftHandle=%p, ucMask=%d, ucEnable=%d\n", ftHandle, ucMask, ucEnable);
    FT_STATUS result = xFT_SetBitMode(ftHandle, ucMask, ucEnable);
    TRACE("returning %d, ftHandle=%p, ucMask=%d, ucEnable=%d\n", result, ftHandle, ucMask, ucEnable);
    return result;
}


FT_STATUS WINAPI FT_GetBitMode(FT_HANDLE ftHandle, PUCHAR pucMode)
{
  TRACE("\n");
  return xFT_GetBitMode(ftHandle, pucMode);
}

// Proxy
FT_STATUS WINAPI FT_SetUSBParameters(FT_HANDLE ftHandle,
                          ULONG ulInTransferSize, ULONG ulOutTransferSize)
{
    TRACE("ftHandle=%p, ulInTransferSize=%d, ulOutTransferSize=%d\n", ftHandle, ulInTransferSize, ulOutTransferSize);
    FT_STATUS result = xFT_SetUSBParameters(ftHandle, ulInTransferSize, ulOutTransferSize);
    TRACE("returning %d, ftHandle=%p, ulInTransferSize=%d, ulOutTransferSize=%d\n", result, ftHandle, ulInTransferSize, ulOutTransferSize);
    return result;
}


FT_STATUS WINAPI FT_SetEventNotification(FT_HANDLE ftHandle,
                                         DWORD dwEventMask, PVOID pvArg)
{
  TRACE("\n");
  return xFT_SetEventNotification(ftHandle, dwEventMask, pvArg);
}

// Proxy
FT_STATUS WINAPI FT_SetChars (FT_HANDLE ftHandle,
   UCHAR uEventCh, UCHAR uEventChEn, UCHAR uErrorCh, UCHAR uErrorChEn)
{
  TRACE("ftHandle=%p, uEventCh=%d, uEventChEn=%d, uErrorCh=%d, uErrorChEn=%d\n", ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    FT_STATUS result = xFT_SetChars(ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    TRACE("returning %d, ftHandle=%p, uEventCh=%d, uEventChEn=%d, uErrorCh=%d, uErrorChEn=%d\n", result, ftHandle, uEventCh, uEventChEn, uErrorCh, uErrorChEn);
    return result;
}

// Proxy
FT_STATUS FT_GetDriverVersion (FT_HANDLE ftHandle, LPDWORD lpdwDriverVersion)
{
  TRACE("ftHandle=%p, lpdwDriverVersion=0x%x\n", ftHandle, *lpdwDriverVersion);
  FT_STATUS result = xFT_GetDriverVersion(ftHandle, lpdwDriverVersion);
  TRACE("returning %d, ftHandle=%p, lpdwDriverVersion=0x%x\n", result, ftHandle, *lpdwDriverVersion);
  return result;
}

// Proxy
FT_STATUS WINAPI FT_GetLibraryVersion (LPDWORD lpdwDLLVersion)
{
  TRACE("lpdwDLLVersion=0x%x\n", *lpdwDLLVersion);
  FT_STATUS result = xFT_GetLibraryVersion (lpdwDLLVersion);
  TRACE("returning %d, lpdwDLLVersion=0x%x\n", result, *lpdwDLLVersion);
  return result;
}
