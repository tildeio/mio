#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Re-export
pub use winapi::{
    c_int,
    GUID,
    DWORD,
    INVALID_HANDLE_VALUE,
    HANDLE,
    ULONG_PTR,
    WORD,
};
pub use libc::{
    atexit,
    uintptr_t,
    AF_INET,
    AF_INET6,
    SOCK_STREAM,
    SOCK_DGRAM,
};
pub use libc::consts::os::extra::{
    INVALID_SOCKET,
    WSAPROTOCOL_LEN,
};
pub use libc::types::os::arch::extra::{
    GROUP,
    WSAPROTOCOL_INFO,
    WSAPROTOCOLCHAIN,
};

/*
 *
 * ===== Types =====
 *
 */

pub type SOCKET = uintptr_t;
pub type LPWSAPROTOCOL_INFO = *mut WSAPROTOCOL_INFO;
pub type LPWSADATA = *mut WSADATA;
pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;

#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct WSADATA {
    pub wVersion: WORD,
    pub wHighVersion: WORD,
    pub szDescription: [u8; WSADESCRIPTION_LEN + 1],
    pub szSystemStatus: [u8; WSASYS_STATUS_LEN + 1],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut u8,
}

#[repr(C)]
#[cfg(target_arch = "x86_64")]
pub struct WSADATA {
    pub wVersion: WORD,
    pub wHighVersion: WORD,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut u8,
    pub szDescription: [u8; WSADESCRIPTION_LEN + 1],
    pub szSystemStatus: [u8; WSASYS_STATUS_LEN + 1],
}

/*
 *
 * ===== Constants =====
 *
 */

pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;

pub const WSA_FLAG_OVERLAPPED: DWORD = 0x01;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: DWORD = 0x02;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: DWORD = 0x04;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: DWORD = 0x08;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: DWORD = 0x10;
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: DWORD = 0x40;

/*
 *
 * ===== FFI =====
 *
 */
extern "system" {
    pub fn CreateIoCompletionPort(FileHandle: HANDLE,
                                  ExistingCompletionPort: HANDLE,
                                  CompletionKey: ULONG_PTR,
                                  NumberOfConcurrentThreads: DWORD) -> HANDLE;

    fn WSASocketW(af: c_int,
                  kind: c_int,
                  protocol: c_int,
                  lpProtocolInfo: LPWSAPROTOCOL_INFO,
                  g: GROUP,
                  dwFlags: DWORD) -> SOCKET;

    pub fn WSAStartup(wVersionRequested: WORD,
                      lpWSAData: LPWSADATA) -> c_int;

    pub fn WSACleanup() -> c_int;
}

pub fn WSASocket(af: c_int,
                 kind: c_int,
                 protocol: c_int,
                 lpProtocolInfo: LPWSAPROTOCOL_INFO,
                 g: GROUP,
                 dwFlags: DWORD) -> SOCKET {

    unsafe {
        WSASocketW(af, kind, protocol, lpProtocolInfo, g, dwFlags)
    }
}

/*
 *
 * ===== Initialize winsock =====
 *
 */

/// Checks whether the Windows socket interface has been started already, and
/// if not, starts it.
pub fn init() {
    use std::mem;
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    // Cleanup winsock resources
    extern "C" fn cleanup_winsock() {
        unsafe { WSACleanup(); }
    }

    START.call_once(|| unsafe {
        let mut data: WSADATA = mem::zeroed();
        let ret = WSAStartup(0x202, &mut data); // Request version 2.2
        assert_eq!(ret, 0);

        atexit(cleanup_winsock);
        // let _ = rt::at_exit(|| { WSACleanup(); });
    });
}
