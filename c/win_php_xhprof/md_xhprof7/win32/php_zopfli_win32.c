/****************************************************************************
 *
 *  Copyright (c) 2013 - 2015 clickalicious GmbH (i.G.) - Benjamin Carl
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 ***************************************************************************/

int getrusage(int who, struct rusage * rusage)
{
    FILETIME starttime;
    FILETIME exittime;
    FILETIME kerneltime;
    FILETIME usertime;
    ULARGE_INTEGER li;

    if (who != RUSAGE_SELF) {
        // Only RUSAGE_SELF is supported in this implementation for now
        errno = EINVAL;
        return -1;
    }

    if (rusage == (struct rusage *) NULL)
    {
        errno = EFAULT;
        return -1;
    }

    memset(rusage, 0, sizeof(struct rusage));

    if (GetProcessTimes(GetCurrentProcess(),
                        &starttime, &exittime, &kerneltime, &usertime) == 0)
    {
        return -1;
    }

    /* Convert FILETIMEs (0.1 us) to struct timeval */
    memcpy(&li, &kerneltime, sizeof(FILETIME));
    li.QuadPart /= 10L;         /* Convert to microseconds */
    rusage->ru_stime.tv_sec = (long)li.QuadPart / 1000000L;
    rusage->ru_stime.tv_usec = (long)li.QuadPart % 1000000L;

    memcpy(&li, &usertime, sizeof(FILETIME));
    li.QuadPart /= 10L;         /* Convert to microseconds */
    rusage->ru_utime.tv_sec = (long)li.QuadPart / 1000000L;
    rusage->ru_utime.tv_usec = (long)li.QuadPart % 1000000L;


    // success
    return 0;
}

// Find 1st Jan 1970 as a FILETIME   
void get_base_time(LARGE_INTEGER *base_time)
{  
    SYSTEMTIME st;
    FILETIME ft;  
  
    memset(&st,0,sizeof(st));
    st.wYear=1970;  
    st.wMonth=1;  
    st.wDay=1;  
    SystemTimeToFileTime(&st, &ft);  
      
    base_time->LowPart = ft.dwLowDateTime;  
    base_time->HighPart = ft.dwHighDateTime;  
    base_time->QuadPart /= SECS_TO_FT_MULT;  
}

int gettimeofday(struct timeval *tv) {
    SYSTEMTIME st;
    FILETIME ft;
    LARGE_INTEGER li;
    static char get_base_time_flag = 0;
  
    if (get_base_time_flag == 0) {  
        get_base_time(&base_time);  
    }  
  
    /* Standard Win32 GetLocalTime */
    GetLocalTime(&st);
    SystemTimeToFileTime(&st, &ft);
  
    li.LowPart = ft.dwLowDateTime;
    li.HighPart = ft.dwHighDateTime;
    li.QuadPart /= SECS_TO_FT_MULT;
    li.QuadPart -= base_time.QuadPart;
  
    tv->tv_sec = li.LowPart;  
    tv->tv_usec = st.wMilliseconds;
    return 0;  
}


ULONGLONG win_cycle_timer()
{
    //SYSTEMTIME st;
    //GetLocalTime(&st);
    //uint64 t1;
    LARGE_INTEGER nTime;
    //QueryPerformanceFrequency(&tc);
    QueryPerformanceCounter(&nTime);
    //get_base_time(&base_time);
	//t1 = (LONGLONG)tc.QuadPart; //ms
    return (ULONGLONG)nTime.QuadPart;
}

void usleep(uint64 i)
{
    Sleep(i/1000);
}