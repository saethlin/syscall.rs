// Copyright 2015 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System call numbers for aarch64 MacOS.

pub const SYSCALL: usize = 0;
pub const EXIT: usize = 1;
pub const FORK: usize = 2;
pub const READ: usize = 3;
pub const WRITE: usize = 4;
pub const OPEN: usize = 5;
pub const CLOSE: usize = 6;
pub const WAIT4: usize = 7;
pub const LINK: usize = 9;
pub const UNLINK: usize = 10;
pub const CHDIR: usize = 12;
pub const FCHDIR: usize = 13;
pub const MKNOD: usize = 14;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 16;
pub const GETFSSTAT: usize = 18;
pub const GETPID: usize = 20;
pub const SETUID: usize = 23;
pub const GETUID: usize = 24;
pub const GETEUID: usize = 25;
pub const PTRACE: usize = 26;
pub const RECVMSG: usize = 27;
pub const SENDMSG: usize = 28;
pub const RECVFROM: usize = 29;
pub const ACCEPT: usize = 30;
pub const GETPEERNAME: usize = 31;
pub const GETSOCKNAME: usize = 32;
pub const ACCESS: usize = 33;
pub const CHFLAGS: usize = 34;
pub const FCHFLAGS: usize = 35;
pub const SYNC: usize = 36;
pub const KILL: usize = 37;
pub const GETPPID: usize = 39;
pub const DUP: usize = 41;
pub const PIPE: usize = 42;
pub const GETEGID: usize = 43;
pub const PROFIL: usize = 44;
pub const SIGACTION: usize = 46;
pub const GETGID: usize = 47;
pub const SIGPROCMASK: usize = 48;
pub const GETLOGIN: usize = 49;
pub const SETLOGIN: usize = 50;
pub const ACCT: usize = 51;
pub const SIGPENDING: usize = 52;
pub const SIGALTSTACK: usize = 53;
pub const IOCTL: usize = 54;
pub const REBOOT: usize = 55;
pub const REVOKE: usize = 56;
pub const SYMLINK: usize = 57;
pub const READLINK: usize = 58;
pub const EXECVE: usize = 59;
pub const UMASK: usize = 60;
pub const CHROOT: usize = 61;
pub const MSYNC: usize = 65;
pub const VFORK: usize = 66;
pub const MUNMAP: usize = 73;
pub const MPROTECT: usize = 74;
pub const MADVISE: usize = 75;
pub const MINCORE: usize = 78;
pub const GETGROUPS: usize = 79;
pub const SETGROUPS: usize = 80;
pub const GETPGRP: usize = 81;
pub const SETPGID: usize = 82;
pub const SETITIMER: usize = 83;
pub const SWAPON: usize = 85;
pub const GETITIMER: usize = 86;
pub const GETDTABLESIZE: usize = 89;
pub const DUP2: usize = 90;
pub const FCNTL: usize = 92;
pub const SELECT: usize = 93;
pub const FSYNC: usize = 95;
pub const SETPRIORITY: usize = 96;
pub const SOCKET: usize = 97;
pub const CONNECT: usize = 98;
pub const GETPRIORITY: usize = 100;
pub const BIND: usize = 104;
pub const SETSOCKOPT: usize = 105;
pub const LISTEN: usize = 106;
pub const SIGSUSPEND: usize = 111;
pub const GETTIMEOFDAY: usize = 116;
pub const GETRUSAGE: usize = 117;
pub const GETSOCKOPT: usize = 118;
pub const READV: usize = 120;
pub const WRITEV: usize = 121;
pub const SETTIMEOFDAY: usize = 122;
pub const FCHOWN: usize = 123;
pub const FCHMOD: usize = 124;
pub const SETREUID: usize = 126;
pub const SETREGID: usize = 127;
pub const RENAME: usize = 128;
pub const FLOCK: usize = 131;
pub const MKFIFO: usize = 132;
pub const SENDTO: usize = 133;
pub const SHUTDOWN: usize = 134;
pub const SOCKETPAIR: usize = 135;
pub const MKDIR: usize = 136;
pub const RMDIR: usize = 137;
pub const UTIMES: usize = 138;
pub const FUTIMES: usize = 139;
pub const ADJTIME: usize = 140;
pub const GETHOSTUUID: usize = 142;
pub const SETSID: usize = 147;
pub const GETPGID: usize = 151;
pub const SETPRIVEXEC: usize = 152;
pub const PREAD: usize = 153;
pub const PWRITE: usize = 154;
pub const NFSSVC: usize = 155;
pub const STATFS: usize = 157;
pub const FSTATFS: usize = 158;
pub const UNMOUNT: usize = 159;
pub const GETFH: usize = 161;
pub const QUOTACTL: usize = 165;
pub const MOUNT: usize = 167;
pub const CSOPS: usize = 169;
pub const WAITID: usize = 173;
pub const ADD_PROFIL: usize = 176;
pub const KDEBUG_TRACE: usize = 180;
pub const SETGID: usize = 181;
pub const SETEGID: usize = 182;
pub const SETEUID: usize = 183;
pub const SIGRETURN: usize = 184;
pub const CHUD: usize = 185;
pub const FDATASYNC: usize = 187;
pub const STAT: usize = 188;
pub const FSTAT: usize = 189;
pub const LSTAT: usize = 190;
pub const PATHCONF: usize = 191;
pub const FPATHCONF: usize = 192;
pub const GETRLIMIT: usize = 194;
pub const SETRLIMIT: usize = 195;
pub const GETDIRENTRIES: usize = 196;
pub const MMAP: usize = 197;
pub const LSEEK: usize = 199;
pub const TRUNCATE: usize = 200;
pub const FTRUNCATE: usize = 201;
pub const __SYSCTL: usize = 202;
pub const MLOCK: usize = 203;
pub const MUNLOCK: usize = 204;
pub const UNDELETE: usize = 205;
pub const ATSOCKET: usize = 206;
pub const ATGETMSG: usize = 207;
pub const ATPUTMSG: usize = 208;
pub const ATPSNDREQ: usize = 209;
pub const ATPSNDRSP: usize = 210;
pub const ATPGETREQ: usize = 211;
pub const ATPGETRSP: usize = 212;
pub const MKCOMPLEX: usize = 216;
pub const STATV: usize = 217;
pub const LSTATV: usize = 218;
pub const FSTATV: usize = 219;
pub const GETATTRLIST: usize = 220;
pub const SETATTRLIST: usize = 221;
pub const GETDIRENTRIESATTR: usize = 222;
pub const EXCHANGEDATA: usize = 223;
pub const SEARCHFS: usize = 225;
pub const DELETE: usize = 226;
pub const COPYFILE: usize = 227;
pub const FGETATTRLIST: usize = 228;
pub const FSETATTRLIST: usize = 229;
pub const POLL: usize = 230;
pub const WATCHEVENT: usize = 231;
pub const WAITEVENT: usize = 232;
pub const MODWATCH: usize = 233;
pub const GETXATTR: usize = 234;
pub const FGETXATTR: usize = 235;
pub const SETXATTR: usize = 236;
pub const FSETXATTR: usize = 237;
pub const REMOVEXATTR: usize = 238;
pub const FREMOVEXATTR: usize = 239;
pub const LISTXATTR: usize = 240;
pub const FLISTXATTR: usize = 241;
pub const FSCTL: usize = 242;
pub const INITGROUPS: usize = 243;
pub const POSIX_SPAWN: usize = 244;
pub const FFSCTL: usize = 245;
pub const NFSCLNT: usize = 247;
pub const FHOPEN: usize = 248;
pub const MINHERIT: usize = 250;
pub const SEMSYS: usize = 251;
pub const MSGSYS: usize = 252;
pub const SHMSYS: usize = 253;
pub const SEMCTL: usize = 254;
pub const SEMGET: usize = 255;
pub const SEMOP: usize = 256;
pub const MSGCTL: usize = 258;
pub const MSGGET: usize = 259;
pub const MSGSND: usize = 260;
pub const MSGRCV: usize = 261;
pub const SHMAT: usize = 262;
pub const SHMCTL: usize = 263;
pub const SHMDT: usize = 264;
pub const SHMGET: usize = 265;
pub const SHM_OPEN: usize = 266;
pub const SHM_UNLINK: usize = 267;
pub const SEM_OPEN: usize = 268;
pub const SEM_CLOSE: usize = 269;
pub const SEM_UNLINK: usize = 270;
pub const SEM_WAIT: usize = 271;
pub const SEM_TRYWAIT: usize = 272;
pub const SEM_POST: usize = 273;
pub const SEM_GETVALUE: usize = 274;
pub const SEM_INIT: usize = 275;
pub const SEM_DESTROY: usize = 276;
pub const OPEN_EXTENDED: usize = 277;
pub const UMASK_EXTENDED: usize = 278;
pub const STAT_EXTENDED: usize = 279;
pub const LSTAT_EXTENDED: usize = 280;
pub const FSTAT_EXTENDED: usize = 281;
pub const CHMOD_EXTENDED: usize = 282;
pub const FCHMOD_EXTENDED: usize = 283;
pub const ACCESS_EXTENDED: usize = 284;
pub const SETTID: usize = 285;
pub const GETTID: usize = 286;
pub const SETSGROUPS: usize = 287;
pub const GETSGROUPS: usize = 288;
pub const SETWGROUPS: usize = 289;
pub const GETWGROUPS: usize = 290;
pub const MKFIFO_EXTENDED: usize = 291;
pub const MKDIR_EXTENDED: usize = 292;
pub const IDENTITYSVC: usize = 293;
pub const SHARED_REGION_CHECK_NP: usize = 294;
pub const VM_PRESSURE_MONITOR: usize = 296;
pub const PSYNCH_RW_LONGRDLOCK: usize = 297;
pub const PSYNCH_RW_YIELDWRLOCK: usize = 298;
pub const PSYNCH_RW_DOWNGRADE: usize = 299;
pub const PSYNCH_RW_UPGRADE: usize = 300;
pub const PSYNCH_MUTEXWAIT: usize = 301;
pub const PSYNCH_MUTEXDROP: usize = 302;
pub const PSYNCH_CVBROAD: usize = 303;
pub const PSYNCH_CVSIGNAL: usize = 304;
pub const PSYNCH_CVWAIT: usize = 305;
pub const PSYNCH_RW_RDLOCK: usize = 306;
pub const PSYNCH_RW_WRLOCK: usize = 307;
pub const PSYNCH_RW_UNLOCK: usize = 308;
pub const PSYNCH_RW_UNLOCK2: usize = 309;
pub const GETSID: usize = 310;
pub const SETTID_WITH_PID: usize = 311;
pub const PSYNCH_CVCLRPREPOST: usize = 312;
pub const AIO_FSYNC: usize = 313;
pub const AIO_RETURN: usize = 314;
pub const AIO_SUSPEND: usize = 315;
pub const AIO_CANCEL: usize = 316;
pub const AIO_ERROR: usize = 317;
pub const AIO_READ: usize = 318;
pub const AIO_WRITE: usize = 319;
pub const LIO_LISTIO: usize = 320;
pub const IOPOLICYSYS: usize = 322;
pub const PROCESS_POLICY: usize = 323;
pub const MLOCKALL: usize = 324;
pub const MUNLOCKALL: usize = 325;
pub const ISSETUGID: usize = 327;
pub const __PTHREAD_KILL: usize = 328;
pub const __PTHREAD_SIGMASK: usize = 329;
pub const __SIGWAIT: usize = 330;
pub const __DISABLE_THREADSIGNAL: usize = 331;
pub const __PTHREAD_MARKCANCEL: usize = 332;
pub const __PTHREAD_CANCELED: usize = 333;
pub const __SEMWAIT_SIGNAL: usize = 334;
pub const PROC_INFO: usize = 336;
pub const SENDFILE: usize = 337;
pub const STAT64: usize = 338;
pub const FSTAT64: usize = 339;
pub const LSTAT64: usize = 340;
pub const STAT64_EXTENDED: usize = 341;
pub const LSTAT64_EXTENDED: usize = 342;
pub const FSTAT64_EXTENDED: usize = 343;
pub const GETDIRENTRIES64: usize = 344;
pub const STATFS64: usize = 345;
pub const FSTATFS64: usize = 346;
pub const GETFSSTAT64: usize = 347;
pub const __PTHREAD_CHDIR: usize = 348;
pub const __PTHREAD_FCHDIR: usize = 349;
pub const AUDIT: usize = 350;
pub const AUDITON: usize = 351;
pub const GETAUID: usize = 353;
pub const SETAUID: usize = 354;
pub const GETAUDIT: usize = 355;
pub const SETAUDIT: usize = 356;
pub const GETAUDIT_ADDR: usize = 357;
pub const SETAUDIT_ADDR: usize = 358;
pub const AUDITCTL: usize = 359;
pub const BSDTHREAD_CREATE: usize = 360;
pub const BSDTHREAD_TERMINATE: usize = 361;
pub const KQUEUE: usize = 362;
pub const KEVENT: usize = 363;
pub const LCHOWN: usize = 364;
pub const STACK_SNAPSHOT: usize = 365;
pub const BSDTHREAD_REGISTER: usize = 366;
pub const WORKQ_OPEN: usize = 367;
pub const WORKQ_KERNRETURN: usize = 368;
pub const KEVENT64: usize = 369;
pub const __OLD_SEMWAIT_SIGNAL: usize = 370;
pub const __OLD_SEMWAIT_SIGNAL_NOCANCEL: usize = 371;
pub const THREAD_SELFID: usize = 372;
pub const __MAC_EXECVE: usize = 380;
pub const __MAC_SYSCALL: usize = 381;
pub const __MAC_GET_FILE: usize = 382;
pub const __MAC_SET_FILE: usize = 383;
pub const __MAC_GET_LINK: usize = 384;
pub const __MAC_SET_LINK: usize = 385;
pub const __MAC_GET_PROC: usize = 386;
pub const __MAC_SET_PROC: usize = 387;
pub const __MAC_GET_FD: usize = 388;
pub const __MAC_SET_FD: usize = 389;
pub const __MAC_GET_PID: usize = 390;
pub const __MAC_GET_LCID: usize = 391;
pub const __MAC_GET_LCTX: usize = 392;
pub const __MAC_SET_LCTX: usize = 393;
pub const SETLCID: usize = 394;
pub const GETLCID: usize = 395;
pub const READ_NOCANCEL: usize = 396;
pub const WRITE_NOCANCEL: usize = 397;
pub const OPEN_NOCANCEL: usize = 398;
pub const CLOSE_NOCANCEL: usize = 399;
pub const WAIT4_NOCANCEL: usize = 400;
pub const RECVMSG_NOCANCEL: usize = 401;
pub const SENDMSG_NOCANCEL: usize = 402;
pub const RECVFROM_NOCANCEL: usize = 403;
pub const ACCEPT_NOCANCEL: usize = 404;
pub const MSYNC_NOCANCEL: usize = 405;
pub const FCNTL_NOCANCEL: usize = 406;
pub const SELECT_NOCANCEL: usize = 407;
pub const FSYNC_NOCANCEL: usize = 408;
pub const CONNECT_NOCANCEL: usize = 409;
pub const SIGSUSPEND_NOCANCEL: usize = 410;
pub const READV_NOCANCEL: usize = 411;
pub const WRITEV_NOCANCEL: usize = 412;
pub const SENDTO_NOCANCEL: usize = 413;
pub const PREAD_NOCANCEL: usize = 414;
pub const PWRITE_NOCANCEL: usize = 415;
pub const WAITID_NOCANCEL: usize = 416;
pub const POLL_NOCANCEL: usize = 417;
pub const MSGSND_NOCANCEL: usize = 418;
pub const MSGRCV_NOCANCEL: usize = 419;
pub const SEM_WAIT_NOCANCEL: usize = 420;
pub const AIO_SUSPEND_NOCANCEL: usize = 421;
pub const __SIGWAIT_NOCANCEL: usize = 422;
pub const __SEMWAIT_SIGNAL_NOCANCEL: usize = 423;
pub const __MAC_MOUNT: usize = 424;
pub const __MAC_GET_MOUNT: usize = 425;
pub const __MAC_GETFSSTAT: usize = 426;
pub const FSGETPATH: usize = 427;
pub const AUDIT_SESSION_SELF: usize = 428;
pub const AUDIT_SESSION_JOIN: usize = 429;
pub const FILEPORT_MAKEPORT: usize = 430;
pub const FILEPORT_MAKEFD: usize = 431;
pub const AUDIT_SESSION_PORT: usize = 432;
pub const PID_SUSPEND: usize = 433;
pub const PID_RESUME: usize = 434;
pub const PID_HIBERNATE: usize = 435;
pub const PID_SHUTDOWN_SOCKETS: usize = 436;
pub const SHARED_REGION_MAP_AND_SLIDE_NP: usize = 438;
pub const MAXSYSCALL: usize = 439;
