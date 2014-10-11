pub const RESTART_SYSCALL          : uint = 0;
pub const EXIT                     : uint = 1;
pub const FORK                     : uint = 2;
pub const READ                     : uint = 3;
pub const WRITE                    : uint = 4;
pub const OPEN                     : uint = 5;
pub const CLOSE                    : uint = 6;
pub const WAITPID                  : uint = 7;
pub const CREAT                    : uint = 8;
pub const LINK                     : uint = 9;
pub const UNLINK                   : uint = 10;
pub const EXECVE                   : uint = 11;
pub const CHDIR                    : uint = 12;
pub const TIME                     : uint = 13;
pub const MKNOD                    : uint = 14;
pub const CHMOD                    : uint = 15;
pub const LCHOWN                   : uint = 16;
pub const BREAK                    : uint = 17;
pub const OLDSTAT                  : uint = 18;
pub const LSEEK                    : uint = 19;
pub const GETPID                   : uint = 20;
pub const MOUNT                    : uint = 21;
pub const UMOUNT                   : uint = 22;
pub const SETUID                   : uint = 23;
pub const GETUID                   : uint = 24;
pub const STIME                    : uint = 25;
pub const PTRACE                   : uint = 26;
pub const ALARM                    : uint = 27;
pub const OLDFSTAT                 : uint = 28;
pub const PAUSE                    : uint = 29;
pub const UTIME                    : uint = 30;
pub const STTY                     : uint = 31;
pub const GTTY                     : uint = 32;
pub const ACCESS                   : uint = 33;
pub const NICE                     : uint = 34;
pub const FTIME                    : uint = 35;
pub const SYNC                     : uint = 36;
pub const KILL                     : uint = 37;
pub const RENAME                   : uint = 38;
pub const MKDIR                    : uint = 39;
pub const RMDIR                    : uint = 40;
pub const DUP                      : uint = 41;
pub const PIPE                     : uint = 42;
pub const TIMES                    : uint = 43;
pub const PROF                     : uint = 44;
pub const BRK                      : uint = 45;
pub const SETGID                   : uint = 46;
pub const GETGID                   : uint = 47;
pub const SIGNAL                   : uint = 48;
pub const GETEUID                  : uint = 49;
pub const GETEGID                  : uint = 50;
pub const ACCT                     : uint = 51;
pub const UMOUNT2                  : uint = 52;
pub const LOCK                     : uint = 53;
pub const IOCTL                    : uint = 54;
pub const FCNTL                    : uint = 55;
pub const MPX                      : uint = 56;
pub const SETPGID                  : uint = 57;
pub const ULIMIT                   : uint = 58;
pub const OLDOLDUNAME              : uint = 59;
pub const UMASK                    : uint = 60;
pub const CHROOT                   : uint = 61;
pub const USTAT                    : uint = 62;
pub const DUP2                     : uint = 63;
pub const GETPPID                  : uint = 64;
pub const GETPGRP                  : uint = 65;
pub const SETSID                   : uint = 66;
pub const SIGACTION                : uint = 67;
pub const SGETMASK                 : uint = 68;
pub const SSETMASK                 : uint = 69;
pub const SETREUID                 : uint = 70;
pub const SETREGID                 : uint = 71;
pub const SIGSUSPEND               : uint = 72;
pub const SIGPENDING               : uint = 73;
pub const SETHOSTNAME              : uint = 74;
pub const SETRLIMIT                : uint = 75;
pub const GETRLIMIT                : uint = 76;
pub const GETRUSAGE                : uint = 77;
pub const GETTIMEOFDAY             : uint = 78;
pub const SETTIMEOFDAY             : uint = 79;
pub const GETGROUPS                : uint = 80;
pub const SETGROUPS                : uint = 81;
pub const SELECT                   : uint = 82;
pub const SYMLINK                  : uint = 83;
pub const OLDLSTAT                 : uint = 84;
pub const READLINK                 : uint = 85;
pub const USELIB                   : uint = 86;
pub const SWAPON                   : uint = 87;
pub const REBOOT                   : uint = 88;
pub const READDIR                  : uint = 89;
pub const MMAP                     : uint = 90;
pub const MUNMAP                   : uint = 91;
pub const TRUNCATE                 : uint = 92;
pub const FTRUNCATE                : uint = 93;
pub const FCHMOD                   : uint = 94;
pub const FCHOWN                   : uint = 95;
pub const GETPRIORITY              : uint = 96;
pub const SETPRIORITY              : uint = 97;
pub const PROFIL                   : uint = 98;
pub const STATFS                   : uint = 99;
pub const FSTATFS                  : uint = 100;
pub const IOPERM                   : uint = 101;
pub const SOCKETCALL               : uint = 102;
pub const SYSLOG                   : uint = 103;
pub const SETITIMER                : uint = 104;
pub const GETITIMER                : uint = 105;
pub const STAT                     : uint = 106;
pub const LSTAT                    : uint = 107;
pub const FSTAT                    : uint = 108;
pub const OLDUNAME                 : uint = 109;
pub const IOPL                     : uint = 110;
pub const VHANGUP                  : uint = 111;
pub const IDLE                     : uint = 112;
pub const VM86OLD                  : uint = 113;
pub const WAIT4                    : uint = 114;
pub const SWAPOFF                  : uint = 115;
pub const SYSINFO                  : uint = 116;
pub const IPC                      : uint = 117;
pub const FSYNC                    : uint = 118;
pub const SIGRETURN                : uint = 119;
pub const CLONE                    : uint = 120;
pub const SETDOMAINNAME            : uint = 121;
pub const UNAME                    : uint = 122;
pub const MODIFY_LDT               : uint = 123;
pub const ADJTIMEX                 : uint = 124;
pub const MPROTECT                 : uint = 125;
pub const SIGPROCMASK              : uint = 126;
pub const CREATE_MODULE            : uint = 127;
pub const INIT_MODULE              : uint = 128;
pub const DELETE_MODULE            : uint = 129;
pub const GET_KERNEL_SYMS          : uint = 130;
pub const QUOTACTL                 : uint = 131;
pub const GETPGID                  : uint = 132;
pub const FCHDIR                   : uint = 133;
pub const BDFLUSH                  : uint = 134;
pub const SYSFS                    : uint = 135;
pub const PERSONALITY              : uint = 136;
pub const AFS_SYSCALL              : uint = 137;
pub const SETFSUID                 : uint = 138;
pub const SETFSGID                 : uint = 139;
pub const _LLSEEK                  : uint = 140;
pub const GETDENTS                 : uint = 141;
pub const _NEWSELECT               : uint = 142;
pub const FLOCK                    : uint = 143;
pub const MSYNC                    : uint = 144;
pub const READV                    : uint = 145;
pub const WRITEV                   : uint = 146;
pub const GETSID                   : uint = 147;
pub const FDATASYNC                : uint = 148;
pub const _SYSCTL                  : uint = 149;
pub const MLOCK                    : uint = 150;
pub const MUNLOCK                  : uint = 151;
pub const MLOCKALL                 : uint = 152;
pub const MUNLOCKALL               : uint = 153;
pub const SCHED_SETPARAM           : uint = 154;
pub const SCHED_GETPARAM           : uint = 155;
pub const SCHED_SETSCHEDULER       : uint = 156;
pub const SCHED_GETSCHEDULER       : uint = 157;
pub const SCHED_YIELD              : uint = 158;
pub const SCHED_GET_PRIORITY_MAX   : uint = 159;
pub const SCHED_GET_PRIORITY_MIN   : uint = 160;
pub const SCHED_RR_GET_INTERVAL    : uint = 161;
pub const NANOSLEEP                : uint = 162;
pub const MREMAP                   : uint = 163;
pub const SETRESUID                : uint = 164;
pub const GETRESUID                : uint = 165;
pub const VM86                     : uint = 166;
pub const QUERY_MODULE             : uint = 167;
pub const POLL                     : uint = 168;
pub const NFSSERVCTL               : uint = 169;
pub const SETRESGID                : uint = 170;
pub const GETRESGID                : uint = 171;
pub const PRCTL                    : uint = 172;
pub const RT_SIGRETURN             : uint = 173;
pub const RT_SIGACTION             : uint = 174;
pub const RT_SIGPROCMASK           : uint = 175;
pub const RT_SIGPENDING            : uint = 176;
pub const RT_SIGTIMEDWAIT          : uint = 177;
pub const RT_SIGQUEUEINFO          : uint = 178;
pub const RT_SIGSUSPEND            : uint = 179;
pub const PREAD64                  : uint = 180;
pub const PWRITE64                 : uint = 181;
pub const CHOWN                    : uint = 182;
pub const GETCWD                   : uint = 183;
pub const CAPGET                   : uint = 184;
pub const CAPSET                   : uint = 185;
pub const SIGALTSTACK              : uint = 186;
pub const SENDFILE                 : uint = 187;
pub const GETPMSG                  : uint = 188;
pub const PUTPMSG                  : uint = 189;
pub const VFORK                    : uint = 190;
pub const UGETRLIMIT               : uint = 191;
pub const MMAP2                    : uint = 192;
pub const TRUNCATE64               : uint = 193;
pub const FTRUNCATE64              : uint = 194;
pub const STAT64                   : uint = 195;
pub const LSTAT64                  : uint = 196;
pub const FSTAT64                  : uint = 197;
pub const LCHOWN32                 : uint = 198;
pub const GETUID32                 : uint = 199;
pub const GETGID32                 : uint = 200;
pub const GETEUID32                : uint = 201;
pub const GETEGID32                : uint = 202;
pub const SETREUID32               : uint = 203;
pub const SETREGID32               : uint = 204;
pub const GETGROUPS32              : uint = 205;
pub const SETGROUPS32              : uint = 206;
pub const FCHOWN32                 : uint = 207;
pub const SETRESUID32              : uint = 208;
pub const GETRESUID32              : uint = 209;
pub const SETRESGID32              : uint = 210;
pub const GETRESGID32              : uint = 211;
pub const CHOWN32                  : uint = 212;
pub const SETUID32                 : uint = 213;
pub const SETGID32                 : uint = 214;
pub const SETFSUID32               : uint = 215;
pub const SETFSGID32               : uint = 216;
pub const PIVOT_ROOT               : uint = 217;
pub const MINCORE                  : uint = 218;
pub const MADVISE                  : uint = 219;
pub const GETDENTS64               : uint = 220;
pub const FCNTL64                  : uint = 221;
pub const GETTID                   : uint = 224;
pub const READAHEAD                : uint = 225;
pub const SETXATTR                 : uint = 226;
pub const LSETXATTR                : uint = 227;
pub const FSETXATTR                : uint = 228;
pub const GETXATTR                 : uint = 229;
pub const LGETXATTR                : uint = 230;
pub const FGETXATTR                : uint = 231;
pub const LISTXATTR                : uint = 232;
pub const LLISTXATTR               : uint = 233;
pub const FLISTXATTR               : uint = 234;
pub const REMOVEXATTR              : uint = 235;
pub const LREMOVEXATTR             : uint = 236;
pub const FREMOVEXATTR             : uint = 237;
pub const TKILL                    : uint = 238;
pub const SENDFILE64               : uint = 239;
pub const FUTEX                    : uint = 240;
pub const SCHED_SETAFFINITY        : uint = 241;
pub const SCHED_GETAFFINITY        : uint = 242;
pub const SET_THREAD_AREA          : uint = 243;
pub const GET_THREAD_AREA          : uint = 244;
pub const IO_SETUP                 : uint = 245;
pub const IO_DESTROY               : uint = 246;
pub const IO_GETEVENTS             : uint = 247;
pub const IO_SUBMIT                : uint = 248;
pub const IO_CANCEL                : uint = 249;
pub const FADVISE64                : uint = 250;
pub const EXIT_GROUP               : uint = 252;
pub const LOOKUP_DCOOKIE           : uint = 253;
pub const EPOLL_CREATE             : uint = 254;
pub const EPOLL_CTL                : uint = 255;
pub const EPOLL_WAIT               : uint = 256;
pub const REMAP_FILE_PAGES         : uint = 257;
pub const SET_TID_ADDRESS          : uint = 258;
pub const TIMER_CREATE             : uint = 259;
pub const TIMER_SETTIME            : uint = 260;
pub const TIMER_GETTIME            : uint = 261;
pub const TIMER_GETOVERRUN         : uint = 262;
pub const TIMER_DELETE             : uint = 263;
pub const CLOCK_SETTIME            : uint = 264;
pub const CLOCK_GETTIME            : uint = 265;
pub const CLOCK_GETRES             : uint = 266;
pub const CLOCK_NANOSLEEP          : uint = 267;
pub const STATFS64                 : uint = 268;
pub const FSTATFS64                : uint = 269;
pub const TGKILL                   : uint = 270;
pub const UTIMES                   : uint = 271;
pub const FADVISE64_64             : uint = 272;
pub const VSERVER                  : uint = 273;
pub const MBIND                    : uint = 274;
pub const GET_MEMPOLICY            : uint = 275;
pub const SET_MEMPOLICY            : uint = 276;
pub const MQ_OPEN                  : uint = 277;
pub const MQ_UNLINK                : uint = 278;
pub const MQ_TIMEDSEND             : uint = 279;
pub const MQ_TIMEDRECEIVE          : uint = 280;
pub const MQ_NOTIFY                : uint = 281;
pub const MQ_GETSETATTR            : uint = 282;
pub const KEXEC_LOAD               : uint = 283;
pub const WAITID                   : uint = 284;
pub const ADD_KEY                  : uint = 286;
pub const REQUEST_KEY              : uint = 287;
pub const KEYCTL                   : uint = 288;
pub const IOPRIO_SET               : uint = 289;
pub const IOPRIO_GET               : uint = 290;
pub const INOTIFY_INIT             : uint = 291;
pub const INOTIFY_ADD_WATCH        : uint = 292;
pub const INOTIFY_RM_WATCH         : uint = 293;
pub const MIGRATE_PAGES            : uint = 294;
pub const OPENAT                   : uint = 295;
pub const MKDIRAT                  : uint = 296;
pub const MKNODAT                  : uint = 297;
pub const FCHOWNAT                 : uint = 298;
pub const FUTIMESAT                : uint = 299;
pub const FSTATAT64                : uint = 300;
pub const UNLINKAT                 : uint = 301;
pub const RENAMEAT                 : uint = 302;
pub const LINKAT                   : uint = 303;
pub const SYMLINKAT                : uint = 304;
pub const READLINKAT               : uint = 305;
pub const FCHMODAT                 : uint = 306;
pub const FACCESSAT                : uint = 307;
pub const PSELECT6                 : uint = 308;
pub const PPOLL                    : uint = 309;
pub const UNSHARE                  : uint = 310;
pub const SET_ROBUST_LIST          : uint = 311;
pub const GET_ROBUST_LIST          : uint = 312;
pub const SPLICE                   : uint = 313;
pub const SYNC_FILE_RANGE          : uint = 314;
pub const TEE                      : uint = 315;
pub const VMSPLICE                 : uint = 316;
pub const MOVE_PAGES               : uint = 317;
pub const GETCPU                   : uint = 318;
pub const EPOLL_PWAIT              : uint = 319;
pub const UTIMENSAT                : uint = 320;
pub const SIGNALFD                 : uint = 321;
pub const TIMERFD_CREATE           : uint = 322;
pub const EVENTFD                  : uint = 323;
pub const FALLOCATE                : uint = 324;
pub const TIMERFD_SETTIME          : uint = 325;
pub const TIMERFD_GETTIME          : uint = 326;
pub const SIGNALFD4                : uint = 327;
pub const EVENTFD2                 : uint = 328;
pub const EPOLL_CREATE1            : uint = 329;
pub const DUP3                     : uint = 330;
pub const PIPE2                    : uint = 331;
pub const INOTIFY_INIT1            : uint = 332;
pub const PREADV                   : uint = 333;
pub const PWRITEV                  : uint = 334;
pub const RT_TGSIGQUEUEINFO        : uint = 335;
pub const PERF_EVENT_OPEN          : uint = 336;
pub const RECVMMSG                 : uint = 337;
pub const FANOTIFY_INIT            : uint = 338;
pub const FANOTIFY_MARK            : uint = 339;
pub const PRLIMIT64                : uint = 340;
pub const NAME_TO_HANDLE_AT        : uint = 341;
pub const OPEN_BY_HANDLE_AT        : uint = 342;
pub const CLOCK_ADJTIME            : uint = 343;
pub const SYNCFS                   : uint = 344;
pub const SENDMMSG                 : uint = 345;
pub const SETNS                    : uint = 346;
pub const PROCESS_VM_READV         : uint = 347;
pub const PROCESS_VM_WRITEV        : uint = 348;
pub const KCMP                     : uint = 349;
pub const FINIT_MODULE             : uint = 350;
pub const SCHED_SETATTR            : uint = 351;
pub const SCHED_GETATTR            : uint = 352;
pub const RENAMEAT2                : uint = 353;
