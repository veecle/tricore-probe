 1 stdcall FT_Open(long ptr)
 2 stdcall FT_Close(ptr)
 3 stdcall FT_Read(ptr ptr long ptr)
 4 stdcall FT_Write(ptr ptr long ptr)
 5 stub FT_IoCtl
 6 stdcall FT_ResetDevice(ptr)
 7 stub FT_SetBaudRate(ptr long)
 8 stub FT_SetDataCharacteristics(ptr long long long)
 9 stdcall FT_SetFlowControl(ptr long long long)
10 stub FT_SetDtr(ptr)
11 stub FT_ClrDtr(ptr)
12 stub FT_SetRts(ptr)
13 stub FT_ClrRts(ptr)
14 stub FT_GetModemStatus(ptr ptr)
15 stdcall FT_SetChars(ptr long long long long)
16 stub FT_Purge(ptr long)
17 stdcall FT_SetTimeouts(ptr long long)
18 stdcall FT_GetQueueStatus(ptr ptr)
19 stub FT_SetEventNotification(ptr long ptr)
20 stub FT_GetEventStatus
21 stub FT_GetStatus(ptr ptr ptr ptr)
22 stub FT_SetBreakOn(ptr)
23 stub FT_SetBreakOff(ptr)
24 stub FT_SetWaitMask
25 stub FT_WaitOnMask
26 stub FT_SetDivisor(ptr long)
27 stub FT_OpenEx(ptr long ptr)
28 stub FT_ListDevices(ptr ptr long)
29 stdcall FT_SetLatencyTimer(ptr long)
30 stub FT_GetLatencyTimer(ptr ptr)
31 stdcall FT_SetBitMode(ptr long long)
32 stub FT_GetBitMode(ptr ptr)
33 stdcall FT_SetUSBParameters(ptr long long)
34 stub FT_EraseEE(ptr)
35 stub FT_ReadEE(ptr long ptr)
36 stub FT_WriteEE(ptr long long)
37 stub FT_EE_Program(ptr ptr)
38 stub FT_EE_Read(ptr ptr)
39 stub FT_EE_UARead(ptr ptr long ptr)
40 stub FT_EE_UASize(ptr ptr)
41 stub FT_EE_UAWrite(ptr ptr long)
42 stub FT_W32_CreateFile(ptr long long ptr long long ptr)
43 stub FT_W32_CloseHandle(ptr)
44 stub FT_W32_ReadFile(ptr ptr long ptr ptr)
45 stub FT_W32_WriteFile(ptr ptr long ptr ptr)
46 stub FT_W32_GetOverlappedResult(ptr ptr ptr long)
47 stub FT_W32_ClearCommBreak(ptr)
48 stub FT_W32_ClearCommError(ptr ptr ptr)
49 stub FT_W32_EscapeCommFunction(ptr long)
50 stub FT_W32_GetCommModemStatus(ptr ptr)
51 stub FT_W32_GetCommState(ptr ptr)
52 stub FT_W32_GetCommTimeouts(ptr ptr)
53 stub FT_W32_GetLastError(ptr)
54 stub FT_W32_PurgeComm(ptr long)
55 stub FT_W32_SetCommBreak(ptr)
56 stub FT_W32_SetCommMask(ptr long)
57 stub FT_W32_SetCommState(ptr ptr)
58 stub FT_W32_SetCommTimeouts(ptr ptr)
59 stub FT_W32_SetupComm(ptr long long)
60 stub FT_W32_WaitCommEvent(ptr ptr ptr)
61 stub FT_GetDeviceInfo(ptr ptr ptr ptr ptr ptr)
62 stub FT_W32_CancelIo
63 stub FT_StopInTask(ptr)
64 stub FT_RestartInTask(ptr)
65 stub FT_SetResetPipeRetryCount
66 stub FT_ResetPort
67 stub FT_EE_ProgramEx(ptr ptr ptr ptr ptr ptr)
68 stub FT_EE_ReadEx(ptr ptr ptr ptr ptr ptr)
69 stub FT_CyclePort
70 stdcall FT_CreateDeviceInfoList(ptr)
71 stub FT_GetDeviceInfoList(ptr ptr)
72 stdcall FT_GetDeviceInfoDetail(long ptr ptr ptr ptr ptr ptr ptr)
73 stub FT_SetDeadmanTimeout
74 stub FT_Finalise
75 stdcall FT_GetDriverVersion(long ptr)
76 stdcall FT_GetLibraryVersion(ptr)
77 stub FT_W32_GetCommMask
78 stub FT_Rescan
79 stub FT_Reload
80 stub FT_GetComPortNumber
81 stub FT_EE_ReadConfig
82 stub FT_EE_WriteConfig
83 stub FT_EE_ReadECC
84 stub FT_GetQueueStatusEx
85 stub FT_EEPROM_Read
86 stub FT_EEPROM_Program
87 stub FT_ComPortIdle
88 stub FT_ComPortCancelIdel
89 stub FT_VendorCmdGet
90 stub FT_VendorCmdSet
91 stub FT_VendorCmdGetEx
92 stub FT_VendorCmdSetEx
93 stub FT_Initialise
