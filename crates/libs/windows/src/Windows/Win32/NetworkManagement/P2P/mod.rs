#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_ADDRESS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_ADDRESS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_ADDRESS_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = 128i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_ADDRESS_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_ADDRESS_LIST {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_ADDRESS_LIST>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_ADDRESS_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct DRT_BOOTSTRAP_PROVIDER {
    pub pvContext: *mut ::core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub InitResolve: isize,
    pub IssueResolve: isize,
    pub EndResolve: isize,
    pub Register: isize,
    pub Unregister: isize,
}
impl ::core::marker::Copy for DRT_BOOTSTRAP_PROVIDER {}
impl ::core::clone::Clone for DRT_BOOTSTRAP_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRT_BOOTSTRAP_PROVIDER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRT_BOOTSTRAP_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_BOOTSTRAP_PROVIDER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRT_BOOTSTRAP_PROVIDER {}
impl ::core::default::Default for DRT_BOOTSTRAP_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hr: ::windows::core::HRESULT, pvcontext: *mut ::core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: super::super::Foundation::BOOL)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct DRT_DATA {
    pub cb: u32,
    pub pb: *mut u8,
}
impl ::core::marker::Copy for DRT_DATA {}
impl ::core::clone::Clone for DRT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRT_DATA {}
impl ::core::default::Default for DRT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: ::windows::core::HRESULT,
    pub pvContext: *mut ::core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA_0_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA_0_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA_0_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA_0_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for DRT_EVENT_DATA_0_2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_EVENT_DATA_0_2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052914i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052913i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_CAPABILITY_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052657i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_DUPLICATE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052919i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_FAULTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052662i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INSUFFICIENT_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052660i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052923i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052924i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_CERT_CHAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057020i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_INSTANCE_PREFIX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052659i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057015i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_KEY_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057022i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_MAX_ADDRESSES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057017i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_MAX_ENDPOINTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057007i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057019i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_PORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052928i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052922i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SEARCH_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052663i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SEARCH_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057006i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SECURITY_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052658i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SECURITY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052926i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052664i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052927i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_NO_ADDRESSES_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052920i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_NO_MORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057018i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_SEARCH_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057016i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_SECURITYPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052916i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052915i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_STILL_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052925i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057023i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052918i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052917i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_ALREADY_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052671i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052665i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052666i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052668i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052670i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052667i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052921i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_STILL_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052661i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_E_TRANSPORT_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052669i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_LEAFSET_KEY_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_MATCH_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct DRT_REGISTRATION {
    pub key: DRT_DATA,
    pub appData: DRT_DATA,
}
impl ::core::marker::Copy for DRT_REGISTRATION {}
impl ::core::clone::Clone for DRT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRT_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRT_REGISTRATION {}
impl ::core::default::Default for DRT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_REGISTRATION_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRT_SEARCH_INFO {
    pub dwSize: u32,
    pub fIterative: super::super::Foundation::BOOL,
    pub fAllowCurrentInstanceMatch: super::super::Foundation::BOOL,
    pub fAnyMatchInRange: super::super::Foundation::BOOL,
    pub cMaxEndpoints: u32,
    pub pMaximumKey: *mut DRT_DATA,
    pub pMinimumKey: *mut DRT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRT_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRT_SEARCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRT_SEARCH_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRT_SEARCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_SEARCH_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRT_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRT_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct DRT_SEARCH_RESULT {
    pub dwSize: u32,
    pub r#type: DRT_MATCH_TYPE,
    pub pvContext: *mut ::core::ffi::c_void,
    pub registration: DRT_REGISTRATION,
}
impl ::core::marker::Copy for DRT_SEARCH_RESULT {}
impl ::core::clone::Clone for DRT_SEARCH_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRT_SEARCH_RESULT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRT_SEARCH_RESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_SEARCH_RESULT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRT_SEARCH_RESULT {}
impl ::core::default::Default for DRT_SEARCH_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_SECURITY_MODE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct DRT_SECURITY_PROVIDER {
    pub pvContext: *mut ::core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub RegisterKey: isize,
    pub UnregisterKey: isize,
    pub ValidateAndUnpackPayload: isize,
    pub SecureAndPackPayload: isize,
    pub FreeData: isize,
    pub EncryptData: isize,
    pub DecryptData: isize,
    pub GetSerializedCredential: isize,
    pub ValidateRemoteCredential: isize,
    pub SignData: isize,
    pub VerifyData: isize,
}
impl ::core::marker::Copy for DRT_SECURITY_PROVIDER {}
impl ::core::clone::Clone for DRT_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRT_SECURITY_PROVIDER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRT_SECURITY_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_SECURITY_PROVIDER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRT_SECURITY_PROVIDER {}
impl ::core::default::Default for DRT_SECURITY_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: super::super::Foundation::PWSTR,
    pub hTransport: *mut ::core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRT_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRT_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRT_SETTINGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRT_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRT_SETTINGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRT_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRT_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type DRT_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ACTIVE: DRT_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_ALONE: DRT_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_NO_NETWORK: DRT_STATUS = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_FAULTED: DRT_STATUS = 20i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const DRT_S_RETRY: ::windows::core::HRESULT = ::windows::core::HRESULT(6426640i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtClose(hdrt: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtClose(hdrt: *const ::core::ffi::c_void);
        }
        DrtClose(::core::mem::transmute(hdrt))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtContinueSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtContinueSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtContinueSearch(::core::mem::transmute(hsearchcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<DRT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, pkey: *mut DRT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: DRT_DATA = ::core::mem::zeroed();
        DrtCreateDerivedKey(::core::mem::transmute(plocalcert), ::core::mem::transmute(&mut result__)).from_abi::<DRT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<*mut DRT_SECURITY_PROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut DRT_SECURITY_PROVIDER = ::core::mem::zeroed();
        DrtCreateDerivedKeySecurityProvider(::core::mem::transmute(prootcert), ::core::mem::transmute(plocalcert), ::core::mem::transmute(&mut result__)).from_abi::<*mut DRT_SECURITY_PROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtCreateDnsBootstrapResolver<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(port: u16, pwszaddress: Param1) -> ::windows::core::Result<*mut DRT_BOOTSTRAP_PROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreateDnsBootstrapResolver(port: u16, pwszaddress: super::super::Foundation::PWSTR, ppmodule: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut DRT_BOOTSTRAP_PROVIDER = ::core::mem::zeroed();
        DrtCreateDnsBootstrapResolver(::core::mem::transmute(port), pwszaddress.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut DRT_BOOTSTRAP_PROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtCreateIpv6UdpTransport(::core::mem::transmute(scope), ::core::mem::transmute(dwscopeid), ::core::mem::transmute(dwlocalitythreshold), ::core::mem::transmute(pwport), ::core::mem::transmute(phtransport)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtCreateNullSecurityProvider() -> ::windows::core::Result<*mut DRT_SECURITY_PROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreateNullSecurityProvider(ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut DRT_SECURITY_PROVIDER = ::core::mem::zeroed();
        DrtCreateNullSecurityProvider(::core::mem::transmute(&mut result__)).from_abi::<*mut DRT_SECURITY_PROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtCreatePnrpBootstrapResolver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(fpublish: Param0, pwzpeername: Param1, pwzcloudname: Param2, pwzpublishingidentity: Param3) -> ::windows::core::Result<*mut DRT_BOOTSTRAP_PROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtCreatePnrpBootstrapResolver(fpublish: super::super::Foundation::BOOL, pwzpeername: super::super::Foundation::PWSTR, pwzcloudname: super::super::Foundation::PWSTR, pwzpublishingidentity: super::super::Foundation::PWSTR, ppresolver: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut DRT_BOOTSTRAP_PROVIDER = ::core::mem::zeroed();
        DrtCreatePnrpBootstrapResolver(fpublish.into_param().abi(), pwzpeername.into_param().abi(), pwzcloudname.into_param().abi(), pwzpublishingidentity.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut DRT_BOOTSTRAP_PROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
        }
        DrtDeleteDerivedKeySecurityProvider(::core::mem::transmute(psecurityprovider))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
        }
        DrtDeleteDnsBootstrapResolver(::core::mem::transmute(presolver))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtDeleteIpv6UdpTransport(htransport: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtDeleteIpv6UdpTransport(htransport: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtDeleteIpv6UdpTransport(::core::mem::transmute(htransport)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
        }
        DrtDeleteNullSecurityProvider(::core::mem::transmute(psecurityprovider))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
        }
        DrtDeletePnrpBootstrapResolver(::core::mem::transmute(presolver))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtEndSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtEndSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtEndSearch(::core::mem::transmute(hsearchcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DrtGetEventData(hdrt: *const ::core::ffi::c_void, uleventdatalen: u32) -> ::windows::core::Result<DRT_EVENT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetEventData(hdrt: *const ::core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: DRT_EVENT_DATA = ::core::mem::zeroed();
        DrtGetEventData(::core::mem::transmute(hdrt), ::core::mem::transmute(uleventdatalen), ::core::mem::transmute(&mut result__)).from_abi::<DRT_EVENT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtGetEventDataSize(hdrt: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetEventDataSize(hdrt: *const ::core::ffi::c_void, puleventdatalen: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DrtGetEventDataSize(::core::mem::transmute(hdrt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtGetInstanceName(hdrt: *const ::core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetInstanceName(hdrt: *const ::core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        DrtGetInstanceName(::core::mem::transmute(hdrt), ::core::mem::transmute(ulcbinstancenamesize), ::core::mem::transmute(pwzdrtinstancename)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtGetInstanceNameSize(hdrt: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetInstanceNameSize(hdrt: *const ::core::ffi::c_void, pulcbinstancenamesize: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DrtGetInstanceNameSize(::core::mem::transmute(hdrt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DrtGetSearchPath(hsearchcontext: *const ::core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetSearchPath(hsearchcontext: *const ::core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> ::windows::core::HRESULT;
        }
        DrtGetSearchPath(::core::mem::transmute(hsearchcontext), ::core::mem::transmute(ulsearchpathsize), ::core::mem::transmute(psearchpath)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtGetSearchPathSize(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetSearchPathSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchpathsize: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DrtGetSearchPathSize(::core::mem::transmute(hsearchcontext), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtGetSearchResult(hsearchcontext: *const ::core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetSearchResult(hsearchcontext: *const ::core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> ::windows::core::HRESULT;
        }
        DrtGetSearchResult(::core::mem::transmute(hsearchcontext), ::core::mem::transmute(ulsearchresultsize), ::core::mem::transmute(psearchresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtGetSearchResultSize(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtGetSearchResultSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchresultsize: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DrtGetSearchResultSize(::core::mem::transmute(hsearchcontext), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtOpen<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(psettings: *const DRT_SETTINGS, hevent: Param1, pvcontext: *const ::core::ffi::c_void, phdrt: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtOpen(psettings: *const DRT_SETTINGS, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, phdrt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtOpen(::core::mem::transmute(psettings), hevent.into_param().abi(), ::core::mem::transmute(pvcontext), ::core::mem::transmute(phdrt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtRegisterKey(hdrt: *const ::core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: *const ::core::ffi::c_void, phkeyregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtRegisterKey(hdrt: *const ::core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: *const ::core::ffi::c_void, phkeyregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtRegisterKey(::core::mem::transmute(hdrt), ::core::mem::transmute(pregistration), ::core::mem::transmute(pvkeycontext), ::core::mem::transmute(phkeyregistration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtStartSearch<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdrt: *const ::core::ffi::c_void, pkey: *const DRT_DATA, pinfo: *const DRT_SEARCH_INFO, timeout: u32, hevent: Param4, pvcontext: *const ::core::ffi::c_void, hsearchcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtStartSearch(hdrt: *const ::core::ffi::c_void, pkey: *const DRT_DATA, pinfo: *const DRT_SEARCH_INFO, timeout: u32, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, hsearchcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DrtStartSearch(::core::mem::transmute(hdrt), ::core::mem::transmute(pkey), ::core::mem::transmute(pinfo), ::core::mem::transmute(timeout), hevent.into_param().abi(), ::core::mem::transmute(pvcontext), ::core::mem::transmute(hsearchcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtUnregisterKey(hkeyregistration: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtUnregisterKey(hkeyregistration: *const ::core::ffi::c_void);
        }
        DrtUnregisterKey(::core::mem::transmute(hkeyregistration))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn DrtUpdateKey(hkeyregistration: *const ::core::ffi::c_void, pappdata: *const DRT_DATA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrtUpdateKey(hkeyregistration: *const ::core::ffi::c_void, pappdata: *const DRT_DATA) -> ::windows::core::HRESULT;
        }
        DrtUpdateKey(::core::mem::transmute(hkeyregistration), ::core::mem::transmute(pappdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const FACILITY_DRT: u32 = 98u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const NS_PNRPCLOUD: u32 = 39u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const NS_PNRPNAME: u32 = 38u32;
pub const NS_PROVIDER_PNRPCLOUD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03fe89ce_766d_4976_b9c1_bb9bc42c7b4d);
pub const NS_PROVIDER_PNRPNAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03fe89cd_766d_4976_b9c1_bb9bc42c7b4d);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEERDIST_CLIENT_BASIC_INFO {
    pub fFlashCrowd: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEERDIST_CLIENT_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEERDIST_CLIENT_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEERDIST_CLIENT_BASIC_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEERDIST_CLIENT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEERDIST_CLIENT_BASIC_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEERDIST_CLIENT_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEERDIST_CLIENT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
impl ::core::marker::Copy for PEERDIST_CONTENT_TAG {}
impl ::core::clone::Clone for PEERDIST_CONTENT_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEERDIST_CONTENT_TAG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEERDIST_CONTENT_TAG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEERDIST_CONTENT_TAG>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEERDIST_CONTENT_TAG {}
impl ::core::default::Default for PEERDIST_CONTENT_TAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEERDIST_PUBLICATION_OPTIONS {
    pub dwVersion: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for PEERDIST_PUBLICATION_OPTIONS {}
impl ::core::clone::Clone for PEERDIST_PUBLICATION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEERDIST_PUBLICATION_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEERDIST_PUBLICATION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEERDIST_PUBLICATION_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEERDIST_PUBLICATION_OPTIONS {}
impl ::core::default::Default for PEERDIST_PUBLICATION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEERDIST_RETRIEVAL_OPTIONS {
    pub cbSize: u32,
    pub dwContentInfoMinVersion: u32,
    pub dwContentInfoMaxVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for PEERDIST_RETRIEVAL_OPTIONS {}
impl ::core::clone::Clone for PEERDIST_RETRIEVAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEERDIST_RETRIEVAL_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEERDIST_RETRIEVAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEERDIST_RETRIEVAL_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEERDIST_RETRIEVAL_OPTIONS {}
impl ::core::default::Default for PEERDIST_RETRIEVAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEERDIST_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEERDIST_STATUS_INFO {
    pub cbSize: u32,
    pub status: PEERDIST_STATUS,
    pub dwMinVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
    pub dwMaxVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
}
impl ::core::marker::Copy for PEERDIST_STATUS_INFO {}
impl ::core::clone::Clone for PEERDIST_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEERDIST_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEERDIST_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEERDIST_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEERDIST_STATUS_INFO {}
impl ::core::default::Default for PEERDIST_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Networking_WinSock'*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_ADDRESS {
    pub dwSize: u32,
    pub sin6: super::super::Networking::WinSock::SOCKADDR_IN6,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for PEER_ADDRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_ADDRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_APPLICATION {
    pub id: ::windows::core::GUID,
    pub data: PEER_DATA,
    pub pwzDescription: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_APPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_APPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_APPLICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_APPLICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_APPLICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_APPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_APPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_APPLICATION_REGISTRATION_INFO {
    pub application: PEER_APPLICATION,
    pub pwzApplicationToLaunch: super::super::Foundation::PWSTR,
    pub pwzApplicationArguments: super::super::Foundation::PWSTR,
    pub dwPublicationScope: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_APPLICATION_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_APPLICATION_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_APPLICATION_REGISTRATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_APPLICATION_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_APPLICATION_REGISTRATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_APPLICATION_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_APPLICATION_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_APPLICATION_REGISTRATION_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_APPLICATION_CURRENT_USER: PEER_APPLICATION_REGISTRATION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_APPLICATION_ALL_USERS: PEER_APPLICATION_REGISTRATION_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_APP_LAUNCH_INFO {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub pInvitation: *mut PEER_INVITATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_APP_LAUNCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_APP_LAUNCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_APP_LAUNCH_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_APP_LAUNCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_APP_LAUNCH_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_APP_LAUNCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_APP_LAUNCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CHANGE_ADDED: PEER_CHANGE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CHANGE_DELETED: PEER_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CHANGE_UPDATED: PEER_CHANGE_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_COLLAB_EVENT_DATA {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub Anonymous: PEER_COLLAB_EVENT_DATA_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_COLLAB_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_COLLAB_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_COLLAB_EVENT_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_COLLAB_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_COLLAB_EVENT_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_COLLAB_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_COLLAB_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union PEER_COLLAB_EVENT_DATA_0 {
    pub watchListChangedData: PEER_EVENT_WATCHLIST_CHANGED_DATA,
    pub presenceChangedData: PEER_EVENT_PRESENCE_CHANGED_DATA,
    pub applicationChangedData: PEER_EVENT_APPLICATION_CHANGED_DATA,
    pub objectChangedData: PEER_EVENT_OBJECT_CHANGED_DATA,
    pub endpointChangedData: PEER_EVENT_ENDPOINT_CHANGED_DATA,
    pub peopleNearMeChangedData: PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA,
    pub requestStatusChangedData: PEER_EVENT_REQUEST_STATUS_CHANGED_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_COLLAB_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_COLLAB_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_COLLAB_EVENT_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_COLLAB_EVENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_COLLAB_EVENT_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_COLLAB_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_COLLAB_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_COLLAB_EVENT_REGISTRATION {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub pInstance: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_COLLAB_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_COLLAB_EVENT_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_COLLAB_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_COLLAB_EVENT_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_COLLAB_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_COLLAB_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_WATCHLIST_CHANGED: PEER_COLLAB_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_ENDPOINT_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_ENDPOINT_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_ENDPOINT_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_MY_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_MY_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_MY_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_MY_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_PEOPLE_NEAR_ME_CHANGED: PEER_COLLAB_EVENT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_EVENT_REQUEST_STATUS_CHANGED: PEER_COLLAB_EVENT_TYPE = 11i32;
pub const PEER_COLLAB_OBJECTID_USER_PICTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd15f41f_fc4e_4922_b035_4c06a754d01d);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_CONNECTION_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CONNECTION_NEIGHBOR: PEER_CONNECTION_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CONNECTION_DIRECT: PEER_CONNECTION_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_CONNECTION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
    pub address: PEER_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_CONNECTION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_CONNECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_CONNECTION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_CONNECTION_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CONNECTED: PEER_CONNECTION_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_DISCONNECTED: PEER_CONNECTION_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_CONNECTION_FAILED: PEER_CONNECTION_STATUS = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_CONTACT {
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub pwzNickName: super::super::Foundation::PWSTR,
    pub pwzDisplayName: super::super::Foundation::PWSTR,
    pub pwzEmailAddress: super::super::Foundation::PWSTR,
    pub fWatch: super::super::Foundation::BOOL,
    pub WatcherPermissions: PEER_WATCH_PERMISSION,
    pub credentials: PEER_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_CONTACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_CONTACT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_CONTACT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_CONTACT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_CONTACT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_CONTACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_CONTACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_CREDENTIAL_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub pwzIssuerPeerName: super::super::Foundation::PWSTR,
    pub pwzIssuerFriendlyName: super::super::Foundation::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for PEER_CREDENTIAL_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_CREDENTIAL_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_DATA {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for PEER_DATA {}
impl ::core::clone::Clone for PEER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_DATA {}
impl ::core::default::Default for PEER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_ENDPOINT {
    pub address: PEER_ADDRESS,
    pub pwzEndpointName: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_ENDPOINT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_ENDPOINT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_ENDPOINT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_ENDPOINT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_APPLICATION_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pApplication: *mut PEER_APPLICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_APPLICATION_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_APPLICATION_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_APPLICATION_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_APPLICATION_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_EVENT_CONNECTION_CHANGE_DATA {
    pub dwSize: u32,
    pub status: PEER_CONNECTION_STATUS,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub ullNextConnectionId: u64,
    pub hrConnectionFailedReason: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_EVENT_CONNECTION_CHANGE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_CONNECTION_CHANGE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_ENDPOINT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_ENDPOINT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_ENDPOINT_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_ENDPOINT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_EVENT_INCOMING_DATA {
    pub dwSize: u32,
    pub ullConnectionId: u64,
    pub r#type: ::windows::core::GUID,
    pub data: PEER_DATA,
}
impl ::core::marker::Copy for PEER_EVENT_INCOMING_DATA {}
impl ::core::clone::Clone for PEER_EVENT_INCOMING_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_EVENT_INCOMING_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_EVENT_INCOMING_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_INCOMING_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_EVENT_INCOMING_DATA {}
impl ::core::default::Default for PEER_EVENT_INCOMING_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_MEMBER_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_MEMBER_CHANGE_TYPE,
    pub pwzIdentity: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_MEMBER_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_EVENT_MEMBER_CHANGE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_MEMBER_CHANGE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_EVENT_MEMBER_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_NODE_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_NODE_CHANGE_TYPE,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_NODE_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_NODE_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_EVENT_NODE_CHANGE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_EVENT_NODE_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_NODE_CHANGE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_EVENT_NODE_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_EVENT_NODE_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_OBJECT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pObject: *mut PEER_OBJECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_OBJECT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_OBJECT_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_OBJECT_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_OBJECT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    pub changeType: PEER_CHANGE_TYPE,
    pub pPeopleNearMe: *mut PEER_PEOPLE_NEAR_ME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_PRESENCE_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pPresenceInfo: *mut PEER_PRESENCE_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_PRESENCE_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_PRESENCE_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_PRESENCE_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_PRESENCE_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_EVENT_RECORD_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_RECORD_CHANGE_TYPE,
    pub recordId: ::windows::core::GUID,
    pub recordType: ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_RECORD_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_EVENT_RECORD_CHANGE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_EVENT_RECORD_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_RECORD_CHANGE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_RECORD_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub hrChange: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_REQUEST_STATUS_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_EVENT_SYNCHRONIZED_DATA {
    pub dwSize: u32,
    pub recordType: ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::clone::Clone for PEER_EVENT_SYNCHRONIZED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_EVENT_SYNCHRONIZED_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_EVENT_SYNCHRONIZED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_SYNCHRONIZED_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::default::Default for PEER_EVENT_SYNCHRONIZED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_WATCHLIST_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub changeType: PEER_CHANGE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_WATCHLIST_CHANGED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_EVENT_WATCHLIST_CHANGED_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_EVENT_WATCHLIST_CHANGED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024713i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_CLIENT_INVALID_COMPARTMENT_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013390i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_CLOUD_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013394i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_CLOUD_IS_DEAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013387i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_CLOUD_IS_SEARCH_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013391i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_CLOUD_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013395i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_DISK_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024784i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_DUPLICATE_PEER_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013388i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_INVALID_IDENTITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013393i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147023728i32);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_E_TOO_MUCH_LOAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013392i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GRAPH_EVENT_DATA {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub Anonymous: PEER_GRAPH_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GRAPH_EVENT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GRAPH_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GRAPH_EVENT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GRAPH_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GRAPH_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PEER_GRAPH_EVENT_DATA_0 {
    pub dwStatus: PEER_GRAPH_STATUS_FLAGS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub nodeChangeData: PEER_EVENT_NODE_CHANGE_DATA,
    pub synchronizedData: PEER_EVENT_SYNCHRONIZED_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GRAPH_EVENT_DATA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GRAPH_EVENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GRAPH_EVENT_DATA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GRAPH_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GRAPH_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_GRAPH_EVENT_REGISTRATION {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub pType: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_GRAPH_EVENT_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_GRAPH_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GRAPH_EVENT_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_GRAPH_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GRAPH_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_STATUS_CHANGED: PEER_GRAPH_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_PROPERTY_CHANGED: PEER_GRAPH_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_RECORD_CHANGED: PEER_GRAPH_EVENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_DIRECT_CONNECTION: PEER_GRAPH_EVENT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_NEIGHBOR_CONNECTION: PEER_GRAPH_EVENT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_INCOMING_DATA: PEER_GRAPH_EVENT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_CONNECTION_REQUIRED: PEER_GRAPH_EVENT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_NODE_CHANGED: PEER_GRAPH_EVENT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_EVENT_SYNCHRONIZED: PEER_GRAPH_EVENT_TYPE = 9i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GRAPH_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwScope: u32,
    pub dwMaxRecordSize: u32,
    pub pwzGraphId: super::super::Foundation::PWSTR,
    pub pwzCreatorId: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub ulPresenceLifetime: u32,
    pub cPresenceMax: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GRAPH_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GRAPH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GRAPH_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GRAPH_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GRAPH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GRAPH_PROPERTY_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_PROPERTY_HEARTBEATS: PEER_GRAPH_PROPERTY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_PROPERTY_DEFER_EXPIRATION: PEER_GRAPH_PROPERTY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GRAPH_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_SCOPE_ANY: PEER_GRAPH_SCOPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_SCOPE_GLOBAL: PEER_GRAPH_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_SCOPE_SITELOCAL: PEER_GRAPH_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_SCOPE_LINKLOCAL: PEER_GRAPH_SCOPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_SCOPE_LOOPBACK: PEER_GRAPH_SCOPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GRAPH_STATUS_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_STATUS_LISTENING: PEER_GRAPH_STATUS_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_STATUS_HAS_CONNECTIONS: PEER_GRAPH_STATUS_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GRAPH_STATUS_SYNCHRONIZED: PEER_GRAPH_STATUS_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GROUP_AUTHENTICATION_SCHEME = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_GMC_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_PASSWORD_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GROUP_EVENT_DATA {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub Anonymous: PEER_GROUP_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GROUP_EVENT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GROUP_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GROUP_EVENT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GROUP_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GROUP_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PEER_GROUP_EVENT_DATA_0 {
    pub dwStatus: PEER_GROUP_STATUS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub memberChangeData: PEER_EVENT_MEMBER_CHANGE_DATA,
    pub hrConnectionFailedReason: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GROUP_EVENT_DATA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GROUP_EVENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GROUP_EVENT_DATA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GROUP_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GROUP_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_GROUP_EVENT_REGISTRATION {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub pType: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_GROUP_EVENT_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_GROUP_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GROUP_EVENT_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_GROUP_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GROUP_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_STATUS_CHANGED: PEER_GROUP_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_PROPERTY_CHANGED: PEER_GROUP_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_RECORD_CHANGED: PEER_GROUP_EVENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_DIRECT_CONNECTION: PEER_GROUP_EVENT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_NEIGHBOR_CONNECTION: PEER_GROUP_EVENT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_INCOMING_DATA: PEER_GROUP_EVENT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_MEMBER_CHANGED: PEER_GROUP_EVENT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_CONNECTION_FAILED: PEER_GROUP_EVENT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_EVENT_AUTHENTICATION_FAILED: PEER_GROUP_EVENT_TYPE = 11i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_STORE_CREDENTIALS: PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GROUP_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloud: super::super::Foundation::PWSTR,
    pub pwzClassifier: super::super::Foundation::PWSTR,
    pub pwzGroupPeerName: super::super::Foundation::PWSTR,
    pub pwzCreatorPeerName: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub ulMemberDataLifetime: u32,
    pub ulPresenceLifetime: u32,
    pub dwAuthenticationSchemes: u32,
    pub pwzGroupPassword: super::super::Foundation::PWSTR,
    pub groupPasswordRole: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_GROUP_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_GROUP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_GROUP_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_GROUP_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_GROUP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GROUP_PROPERTY_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_DATA_OPTIONAL: PEER_GROUP_PROPERTY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_DISABLE_PRESENCE: PEER_GROUP_PROPERTY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_DEFER_EXPIRATION: PEER_GROUP_PROPERTY_FLAGS = 4i32;
pub const PEER_GROUP_ROLE_ADMIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04387127_aa56_450a_8ce5_4f565c6790f4);
pub const PEER_GROUP_ROLE_INVITING_MEMBER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4370fd89_dc18_4cfb_8dbf_9853a8a9f905);
pub const PEER_GROUP_ROLE_MEMBER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf12dc4c7_0857_4ca0_93fc_b1bb19a3d8c2);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_GROUP_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_STATUS_LISTENING: PEER_GROUP_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_GROUP_STATUS_HAS_CONNECTIONS: PEER_GROUP_STATUS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_INVITATION {
    pub applicationId: ::windows::core::GUID,
    pub applicationData: PEER_DATA,
    pub pwzMessage: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_INVITATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_INVITATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_INVITATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_INVITATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_INVITATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_INVITATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_INVITATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_INVITATION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub dwScope: u32,
    pub dwCloudFlags: u32,
    pub pwzGroupPeerName: super::super::Foundation::PWSTR,
    pub pwzIssuerPeerName: super::super::Foundation::PWSTR,
    pub pwzSubjectPeerName: super::super::Foundation::PWSTR,
    pub pwzGroupFriendlyName: super::super::Foundation::PWSTR,
    pub pwzIssuerFriendlyName: super::super::Foundation::PWSTR,
    pub pwzSubjectFriendlyName: super::super::Foundation::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows::core::GUID,
    pub cClassifiers: u32,
    pub ppwzClassifiers: *mut super::super::Foundation::PWSTR,
    pub pSubjectPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub authScheme: PEER_GROUP_AUTHENTICATION_SCHEME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_INVITATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_INVITATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for PEER_INVITATION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_INVITATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_INVITATION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_INVITATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_INVITATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_INVITATION_RESPONSE {
    pub action: PEER_INVITATION_RESPONSE_TYPE,
    pub pwzMessage: super::super::Foundation::PWSTR,
    pub hrExtendedInfo: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_INVITATION_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_INVITATION_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_INVITATION_RESPONSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_INVITATION_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_INVITATION_RESPONSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_INVITATION_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_INVITATION_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_INVITATION_RESPONSE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_INVITATION_RESPONSE_DECLINED: PEER_INVITATION_RESPONSE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_INVITATION_RESPONSE_ACCEPTED: PEER_INVITATION_RESPONSE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_INVITATION_RESPONSE_EXPIRED: PEER_INVITATION_RESPONSE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_INVITATION_RESPONSE_ERROR: PEER_INVITATION_RESPONSE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct PEER_MEMBER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzIdentity: super::super::Foundation::PWSTR,
    pub pwzAttributes: super::super::Foundation::PWSTR,
    pub ullNodeId: u64,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pCredentialInfo: *mut PEER_CREDENTIAL_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_MEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_MEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for PEER_MEMBER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_MEMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_MEMBER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_MEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_MEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_MEMBER_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_CONNECTED: PEER_MEMBER_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_DISCONNECTED: PEER_MEMBER_CHANGE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_UPDATED: PEER_MEMBER_CHANGE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_JOINED: PEER_MEMBER_CHANGE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_LEFT: PEER_MEMBER_CHANGE_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_MEMBER_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_MEMBER_PRESENT: PEER_MEMBER_FLAGS = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_NAME_PAIR {
    pub dwSize: u32,
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_NAME_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_NAME_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_NAME_PAIR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_NAME_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_NAME_PAIR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_NAME_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_NAME_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_NODE_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_NODE_CHANGE_CONNECTED: PEER_NODE_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_NODE_CHANGE_DISCONNECTED: PEER_NODE_CHANGE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_NODE_CHANGE_UPDATED: PEER_NODE_CHANGE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_NODE_INFO {
    pub dwSize: u32,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pwzAttributes: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_NODE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_NODE_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_NODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_NODE_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_NODE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_OBJECT {
    pub id: ::windows::core::GUID,
    pub data: PEER_DATA,
    pub dwPublicationScope: u32,
}
impl ::core::marker::Copy for PEER_OBJECT {}
impl ::core::clone::Clone for PEER_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_OBJECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_OBJECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_OBJECT {}
impl ::core::default::Default for PEER_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PEOPLE_NEAR_ME {
    pub pwzNickName: super::super::Foundation::PWSTR,
    pub endpoint: PEER_ENDPOINT,
    pub id: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PEOPLE_NEAR_ME {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PEOPLE_NEAR_ME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_PEOPLE_NEAR_ME {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_PEOPLE_NEAR_ME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_PEOPLE_NEAR_ME>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_PEOPLE_NEAR_ME {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_PEOPLE_NEAR_ME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_PNRP_CLOUD_INFO {
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub dwScope: PNRP_SCOPE,
    pub dwScopeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_PNRP_CLOUD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_PNRP_CLOUD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_PNRP_CLOUD_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_PNRP_CLOUD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_PNRP_CLOUD_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_PNRP_CLOUD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_PNRP_CLOUD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PNRP_ENDPOINT_INFO {
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PNRP_ENDPOINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_PNRP_ENDPOINT_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_PNRP_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_PNRP_ENDPOINT_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_PNRP_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PNRP_REGISTRATION_INFO {
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub pwzPublishingIdentity: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub wPort: u16,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PNRP_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PEER_PNRP_REGISTRATION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_PNRP_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_PNRP_REGISTRATION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_PNRP_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_PRESENCE_INFO {
    pub status: PEER_PRESENCE_STATUS,
    pub pwzDescriptiveText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_PRESENCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_PRESENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_PRESENCE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_PRESENCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_PRESENCE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_PRESENCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_PRESENCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_PRESENCE_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_OFFLINE: PEER_PRESENCE_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_OUT_TO_LUNCH: PEER_PRESENCE_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_AWAY: PEER_PRESENCE_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_BE_RIGHT_BACK: PEER_PRESENCE_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_IDLE: PEER_PRESENCE_STATUS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_BUSY: PEER_PRESENCE_STATUS = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_ON_THE_PHONE: PEER_PRESENCE_STATUS = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PRESENCE_ONLINE: PEER_PRESENCE_STATUS = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_PUBLICATION_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PUBLICATION_SCOPE_NONE: PEER_PUBLICATION_SCOPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PUBLICATION_SCOPE_NEAR_ME: PEER_PUBLICATION_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PUBLICATION_SCOPE_INTERNET: PEER_PUBLICATION_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_PUBLICATION_SCOPE_ALL: PEER_PUBLICATION_SCOPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_RECORD {
    pub dwSize: u32,
    pub r#type: ::windows::core::GUID,
    pub id: ::windows::core::GUID,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pwzCreatorId: super::super::Foundation::PWSTR,
    pub pwzModifiedById: super::super::Foundation::PWSTR,
    pub pwzAttributes: super::super::Foundation::PWSTR,
    pub ftCreation: super::super::Foundation::FILETIME,
    pub ftExpiration: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
    pub securityData: PEER_DATA,
    pub data: PEER_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_RECORD_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_ADDED: PEER_RECORD_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_UPDATED: PEER_RECORD_CHANGE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_DELETED: PEER_RECORD_CHANGE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_EXPIRED: PEER_RECORD_CHANGE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_RECORD_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_FLAG_AUTOREFRESH: PEER_RECORD_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_RECORD_FLAG_DELETED: PEER_RECORD_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_SECURITY_INTERFACE {
    pub dwSize: u32,
    pub pwzSspFilename: super::super::Foundation::PWSTR,
    pub pwzPackageName: super::super::Foundation::PWSTR,
    pub cbSecurityInfo: u32,
    pub pbSecurityInfo: *mut u8,
    pub pvContext: *mut ::core::ffi::c_void,
    pub pfnValidateRecord: PFNPEER_VALIDATE_RECORD,
    pub pfnSecureRecord: PFNPEER_SECURE_RECORD,
    pub pfnFreeSecurityData: PFNPEER_FREE_SECURITY_DATA,
    pub pfnAuthFailed: PFNPEER_ON_PASSWORD_AUTH_FAILED,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_SECURITY_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_SECURITY_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PEER_SECURITY_INTERFACE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_SECURITY_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_SECURITY_INTERFACE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_SECURITY_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_SECURITY_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_SIGNIN_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_SIGNIN_NONE: PEER_SIGNIN_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_SIGNIN_NEAR_ME: PEER_SIGNIN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_SIGNIN_INTERNET: PEER_SIGNIN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_SIGNIN_ALL: PEER_SIGNIN_FLAGS = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PEER_VERSION_DATA {
    pub wVersion: u16,
    pub wHighestVersion: u16,
}
impl ::core::marker::Copy for PEER_VERSION_DATA {}
impl ::core::clone::Clone for PEER_VERSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PEER_VERSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PEER_VERSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEER_VERSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PEER_VERSION_DATA {}
impl ::core::default::Default for PEER_VERSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PEER_WATCH_PERMISSION = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_WATCH_BLOCKED: PEER_WATCH_PERMISSION = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PEER_WATCH_ALLOWED: PEER_WATCH_PERMISSION = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PFNPEER_FREE_SECURITY_DATA = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, psecuritydata: *const PEER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PFNPEER_ON_PASSWORD_AUTH_FAILED = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_SECURE_RECORD = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE, ppsecuritydata: *mut *mut PEER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_VALIDATE_RECORD = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PNRPCLOUDINFO {
    pub dwSize: u32,
    pub Cloud: PNRP_CLOUD_ID,
    pub enCloudState: PNRP_CLOUD_STATE,
    pub enCloudFlags: PNRP_CLOUD_FLAGS,
}
impl ::core::marker::Copy for PNRPCLOUDINFO {}
impl ::core::clone::Clone for PNRPCLOUDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PNRPCLOUDINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PNRPCLOUDINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PNRPCLOUDINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PNRPCLOUDINFO {}
impl ::core::default::Default for PNRPCLOUDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRPINFO_HINT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: super::super::Foundation::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PNRPINFO_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PNRPINFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for PNRPINFO_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PNRPINFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PNRPINFO_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PNRPINFO_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PNRPINFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: super::super::Foundation::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
    pub enExtendedPayloadType: PNRP_EXTENDED_PAYLOAD_TYPE,
    pub Anonymous: PNRPINFO_V2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for PNRPINFO_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for PNRPINFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PNRPINFO_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for PNRPINFO_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::default::Default for PNRPINFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for PNRPINFO_V2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for PNRPINFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PNRPINFO_V2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for PNRPINFO_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::default::Default for PNRPINFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_CLOUD_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub struct PNRP_CLOUD_ID {
    pub AddressFamily: i32,
    pub Scope: PNRP_SCOPE,
    pub ScopeId: u32,
}
impl ::core::marker::Copy for PNRP_CLOUD_ID {}
impl ::core::clone::Clone for PNRP_CLOUD_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PNRP_CLOUD_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PNRP_CLOUD_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PNRP_CLOUD_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for PNRP_CLOUD_ID {}
impl ::core::default::Default for PNRP_CLOUD_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_CLOUD_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_EXTENDED_PAYLOAD_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_REGISTERED_ID_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_RESOLVE_CRITERIA = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub type PNRP_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabAddContact<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzcontactdata: Param0) -> ::windows::core::Result<*mut PEER_CONTACT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabAddContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_CONTACT = ::core::mem::zeroed();
        PeerCollabAddContact(pwzcontactdata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_CONTACT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabAsyncInviteContact<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: Param3) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabAsyncInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        PeerCollabAsyncInviteContact(::core::mem::transmute(pccontact), ::core::mem::transmute(pcendpoint), ::core::mem::transmute(pcinvitation), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabAsyncInviteEndpoint<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabAsyncInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        PeerCollabAsyncInviteEndpoint(::core::mem::transmute(pcendpoint), ::core::mem::transmute(pcinvitation), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabCancelInvitation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hinvitation: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabCancelInvitation(hinvitation: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        PeerCollabCancelInvitation(hinvitation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabCloseHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hinvitation: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabCloseHandle(hinvitation: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        PeerCollabCloseHandle(hinvitation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabDeleteContact<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzpeername: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabDeleteContact(pwzpeername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerCollabDeleteContact(pwzpeername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::HRESULT;
        }
        PeerCollabDeleteEndpointData(::core::mem::transmute(pcendpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabDeleteObject(pobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabDeleteObject(pobjectid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        PeerCollabDeleteObject(::core::mem::transmute(pobjectid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumApplicationRegistrationInfo(::core::mem::transmute(registrationtype), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabEnumApplications(pcendpoint: *const PEER_ENDPOINT, papplicationid: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumApplications(pcendpoint: *const PEER_ENDPOINT, papplicationid: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumApplications(::core::mem::transmute(pcendpoint), ::core::mem::transmute(papplicationid), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabEnumContacts(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumContacts(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumContacts(::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabEnumEndpoints(pccontact: *const PEER_CONTACT, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumEndpoints(pccontact: *const PEER_CONTACT, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumEndpoints(::core::mem::transmute(pccontact), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabEnumObjects(pcendpoint: *const PEER_ENDPOINT, pobjectid: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumObjects(pcendpoint: *const PEER_ENDPOINT, pobjectid: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumObjects(::core::mem::transmute(pcendpoint), ::core::mem::transmute(pobjectid), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabEnumPeopleNearMe(::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabExportContact<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzpeername: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabExportContact(pwzpeername: super::super::Foundation::PWSTR, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerCollabExportContact(pwzpeername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabGetAppLaunchInfo() -> ::windows::core::Result<*mut PEER_APP_LAUNCH_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetAppLaunchInfo(pplaunchinfo: *mut *mut PEER_APP_LAUNCH_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_APP_LAUNCH_INFO = ::core::mem::zeroed();
        PeerCollabGetAppLaunchInfo(::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_APP_LAUNCH_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<*mut PEER_APPLICATION_REGISTRATION_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, ppapplication: *mut *mut PEER_APPLICATION_REGISTRATION_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_APPLICATION_REGISTRATION_INFO = ::core::mem::zeroed();
        PeerCollabGetApplicationRegistrationInfo(::core::mem::transmute(papplicationid), ::core::mem::transmute(registrationtype), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_APPLICATION_REGISTRATION_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetContact<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzpeername: Param0) -> ::windows::core::Result<*mut PEER_CONTACT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetContact(pwzpeername: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_CONTACT = ::core::mem::zeroed();
        PeerCollabGetContact(pwzpeername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_CONTACT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetEndpointName() -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetEndpointName(ppwzendpointname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerCollabGetEndpointName(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_COLLAB_EVENT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_COLLAB_EVENT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_COLLAB_EVENT_DATA = ::core::mem::zeroed();
        PeerCollabGetEventData(::core::mem::transmute(hpeerevent), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_COLLAB_EVENT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetInvitationResponse<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hinvitation: Param0) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetInvitationResponse(hinvitation: super::super::Foundation::HANDLE, ppinvitationresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_INVITATION_RESPONSE = ::core::mem::zeroed();
        PeerCollabGetInvitationResponse(hinvitation.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_INVITATION_RESPONSE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabGetPresenceInfo(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<*mut PEER_PRESENCE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetPresenceInfo(pcendpoint: *const PEER_ENDPOINT, pppresenceinfo: *mut *mut PEER_PRESENCE_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_PRESENCE_INFO = ::core::mem::zeroed();
        PeerCollabGetPresenceInfo(::core::mem::transmute(pcendpoint), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_PRESENCE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabGetSigninOptions() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabGetSigninOptions(pdwsigninoptions: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        PeerCollabGetSigninOptions(::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_INVITATION_RESPONSE = ::core::mem::zeroed();
        PeerCollabInviteContact(::core::mem::transmute(pccontact), ::core::mem::transmute(pcendpoint), ::core::mem::transmute(pcinvitation), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_INVITATION_RESPONSE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_INVITATION_RESPONSE = ::core::mem::zeroed();
        PeerCollabInviteEndpoint(::core::mem::transmute(pcendpoint), ::core::mem::transmute(pcinvitation), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_INVITATION_RESPONSE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabParseContact<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzcontactdata: Param0) -> ::windows::core::Result<*mut PEER_CONTACT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabParseContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_CONTACT = ::core::mem::zeroed();
        PeerCollabParseContact(pwzcontactdata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_CONTACT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabQueryContactData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabQueryContactData(pcendpoint: *const PEER_ENDPOINT, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerCollabQueryContactData(::core::mem::transmute(pcendpoint), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::HRESULT;
        }
        PeerCollabRefreshEndpointData(::core::mem::transmute(pcendpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::HRESULT;
        }
        PeerCollabRegisterApplication(::core::mem::transmute(pcapplication), ::core::mem::transmute(registrationtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabRegisterEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0, ceventregistration: u32, peventregistrations: *const PEER_COLLAB_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabRegisterEvent(hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_COLLAB_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabRegisterEvent(hevent.into_param().abi(), ::core::mem::transmute(ceventregistration), ::core::mem::transmute(peventregistrations), ::core::mem::transmute(phpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabSetEndpointName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzendpointname: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSetEndpointName(pwzendpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerCollabSetEndpointName(pwzendpointname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> ::windows::core::HRESULT;
        }
        PeerCollabSetObject(::core::mem::transmute(pcobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> ::windows::core::HRESULT;
        }
        PeerCollabSetPresenceInfo(::core::mem::transmute(pcpresenceinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabShutdown() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabShutdown() -> ::windows::core::HRESULT;
        }
        PeerCollabShutdown().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabSignin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, dwsigninoptions: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSignin(hwndparent: super::super::Foundation::HWND, dwsigninoptions: u32) -> ::windows::core::HRESULT;
        }
        PeerCollabSignin(hwndparent.into_param().abi(), ::core::mem::transmute(dwsigninoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabSignout(dwsigninoptions: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSignout(dwsigninoptions: u32) -> ::windows::core::HRESULT;
        }
        PeerCollabSignout(::core::mem::transmute(dwsigninoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabStartup(wversionrequested: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabStartup(wversionrequested: u16) -> ::windows::core::HRESULT;
        }
        PeerCollabStartup(::core::mem::transmute(wversionrequested)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::HRESULT;
        }
        PeerCollabSubscribeEndpointData(::core::mem::transmute(pcendpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabUnregisterApplication(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabUnregisterApplication(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::HRESULT;
        }
        PeerCollabUnregisterApplication(::core::mem::transmute(papplicationid), ::core::mem::transmute(registrationtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerCollabUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerCollabUnregisterEvent(::core::mem::transmute(hpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::HRESULT;
        }
        PeerCollabUnsubscribeEndpointData(::core::mem::transmute(pcendpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> ::windows::core::HRESULT;
        }
        PeerCollabUpdateContact(::core::mem::transmute(pcontact)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCreatePeerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzclassifier: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerCreatePeerName(pwzidentity: super::super::Foundation::PWSTR, pwzclassifier: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerCreatePeerName(pwzidentity.into_param().abi(), pwzclassifier.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientAddContentInformation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(cbnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientAddData(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(cbnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientBlockRead(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(cbmaxnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwtimeoutinmilliseconds), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientCancelAsyncOperation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32;
        }
        ::core::mem::transmute(PeerDistClientCloseContent(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientCompleteContentInformation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientFlushContent<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: Param2, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientFlushContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientFlushContent(::core::mem::transmute(hpeerdist), ::core::mem::transmute(pcontenttag), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PeerDistClientGetInformationByHandle(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(peerdistclientinfoclass), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistClientOpenContent<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: Param2, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientOpenContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32;
        }
        ::core::mem::transmute(PeerDistClientOpenContent(::core::mem::transmute(hpeerdist), ::core::mem::transmute(pcontenttag), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(phcontenthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistClientStreamRead(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontenthandle), ::core::mem::transmute(cbmaxnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwtimeoutinmilliseconds), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistGetOverlappedResult<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistGetOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PeerDistGetOverlappedResult(::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
        }
        ::core::mem::transmute(PeerDistGetStatus(::core::mem::transmute(hpeerdist), ::core::mem::transmute(ppeerdiststatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
        }
        ::core::mem::transmute(PeerDistGetStatusEx(::core::mem::transmute(hpeerdist), ::core::mem::transmute(ppeerdiststatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, hcompletionport: Param1, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistRegisterForStatusChangeNotification(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
        }
        ::core::mem::transmute(PeerDistRegisterForStatusChangeNotification(::core::mem::transmute(hpeerdist), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(ppeerdiststatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotificationEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, hcompletionport: Param1, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
        }
        ::core::mem::transmute(PeerDistRegisterForStatusChangeNotificationEx(::core::mem::transmute(hpeerdist), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(ppeerdiststatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistServerCancelAsyncOperation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(cbcontentidentifier), ::core::mem::transmute(pcontentidentifier), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32;
        }
        ::core::mem::transmute(PeerDistServerCloseContentInformation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontentinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32;
        }
        ::core::mem::transmute(PeerDistServerCloseStreamHandle(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hstream)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerOpenContentInformation<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: Param5, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerOpenContentInformation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
        }
        ::core::mem::transmute(PeerDistServerOpenContentInformation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(cbcontentidentifier), ::core::mem::transmute(pcontentidentifier), ::core::mem::transmute(ullcontentoffset), ::core::mem::transmute(cbcontentlength), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(phcontentinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerOpenContentInformationEx<'a, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: Param6, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerOpenContentInformationEx(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
        }
        ::core::mem::transmute(PeerDistServerOpenContentInformationEx(::core::mem::transmute(hpeerdist), ::core::mem::transmute(cbcontentidentifier), ::core::mem::transmute(pcontentidentifier), ::core::mem::transmute(ullcontentoffset), ::core::mem::transmute(cbcontentlength), ::core::mem::transmute(pretrievaloptions), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(phcontentinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistServerPublishAddToStream(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hstream), ::core::mem::transmute(cbnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistServerPublishCompleteStream(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hstream), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerPublishStream<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, cbcontentlength: u64, ppublishoptions: *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport: Param5, ulcompletionkey: usize, phstream: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerPublishStream(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, cbcontentlength: u64, ppublishoptions: *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phstream: *mut isize) -> u32;
        }
        ::core::mem::transmute(PeerDistServerPublishStream(::core::mem::transmute(hpeerdist), ::core::mem::transmute(cbcontentidentifier), ::core::mem::transmute(pcontentidentifier), ::core::mem::transmute(cbcontentlength), ::core::mem::transmute(ppublishoptions), hcompletionport.into_param().abi(), ::core::mem::transmute(ulcompletionkey), ::core::mem::transmute(phstream)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(PeerDistServerRetrieveContentInformation(::core::mem::transmute(hpeerdist), ::core::mem::transmute(hcontentinfo), ::core::mem::transmute(cbmaxnumberofbytes), ::core::mem::transmute(pbuffer), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistServerUnpublish(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistServerUnpublish(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8) -> u32;
        }
        ::core::mem::transmute(PeerDistServerUnpublish(::core::mem::transmute(hpeerdist), ::core::mem::transmute(cbcontentidentifier), ::core::mem::transmute(pcontentidentifier)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistShutdown(hpeerdist: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistShutdown(hpeerdist: isize) -> u32;
        }
        ::core::mem::transmute(PeerDistShutdown(::core::mem::transmute(hpeerdist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: *mut u32) -> u32;
        }
        ::core::mem::transmute(PeerDistStartup(::core::mem::transmute(dwversionrequested), ::core::mem::transmute(phpeerdist), ::core::mem::transmute(pdwsupportedversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32;
        }
        ::core::mem::transmute(PeerDistUnregisterForStatusChangeNotification(::core::mem::transmute(hpeerdist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerEndEnumeration(::core::mem::transmute(hpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerEnumGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerEnumGroups(pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerEnumGroups(pwzidentity.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerEnumIdentities(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerEnumIdentities(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerEnumIdentities(::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerFreeData(pvdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerFreeData(pvdata: *const ::core::ffi::c_void);
        }
        PeerFreeData(::core::mem::transmute(pvdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGetItemCount(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        PeerGetItemCount(::core::mem::transmute(hpeerenum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGetNextItem(::core::mem::transmute(hpeerenum), ::core::mem::transmute(pcount), ::core::mem::transmute(pppvitems)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphAddRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphAddRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        PeerGraphAddRecord(::core::mem::transmute(hgraph), ::core::mem::transmute(precord), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphClose(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphClose(hgraph: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphClose(::core::mem::transmute(hgraph)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphCloseDirectConnection(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphCloseDirectConnection(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::HRESULT;
        }
        PeerGraphCloseDirectConnection(::core::mem::transmute(hgraph), ::core::mem::transmute(ullconnectionid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerGraphConnect<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzpeerid: Param1, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphConnect(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        PeerGraphConnect(::core::mem::transmute(hgraph), pwzpeerid.into_param().abi(), ::core::mem::transmute(paddress), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphCreate<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: Param1, psecurityinterface: *const PEER_SECURITY_INTERFACE, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphCreate(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const PEER_SECURITY_INTERFACE, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphCreate(::core::mem::transmute(pgraphproperties), pwzdatabasename.into_param().abi(), ::core::mem::transmute(psecurityinterface), ::core::mem::transmute(phgraph)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphDelete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzgraphid: Param0, pwzpeerid: Param1, pwzdatabasename: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphDelete(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGraphDelete(pwzgraphid.into_param().abi(), pwzpeerid.into_param().abi(), pwzdatabasename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphDeleteRecord<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID, flocal: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphDeleteRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID, flocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        PeerGraphDeleteRecord(::core::mem::transmute(hgraph), ::core::mem::transmute(precordid), flocal.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphEndEnumeration(::core::mem::transmute(hpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphEnumConnections(hgraph: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphEnumConnections(hgraph: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphEnumConnections(::core::mem::transmute(hgraph), ::core::mem::transmute(dwflags), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphEnumNodes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzpeerid: Param1, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphEnumNodes(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphEnumNodes(::core::mem::transmute(hgraph), pwzpeerid.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphEnumRecords<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, precordtype: *const ::windows::core::GUID, pwzpeerid: Param2, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphEnumRecords(hgraph: *const ::core::ffi::c_void, precordtype: *const ::windows::core::GUID, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphEnumRecords(::core::mem::transmute(hgraph), ::core::mem::transmute(precordtype), pwzpeerid.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphExportDatabase<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzfilepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphExportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGraphExportDatabase(::core::mem::transmute(hgraph), pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphFreeData(pvdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphFreeData(pvdata: *const ::core::ffi::c_void);
        }
        PeerGraphFreeData(::core::mem::transmute(pvdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GRAPH_EVENT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GRAPH_EVENT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_GRAPH_EVENT_DATA = ::core::mem::zeroed();
        PeerGraphGetEventData(::core::mem::transmute(hpeerevent), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_GRAPH_EVENT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphGetItemCount(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        PeerGraphGetItemCount(::core::mem::transmute(hpeerenum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphGetNextItem(::core::mem::transmute(hpeerenum), ::core::mem::transmute(pcount), ::core::mem::transmute(pppvitems)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerGraphGetNodeInfo(hgraph: *const ::core::ffi::c_void, ullnodeid: u64) -> ::windows::core::Result<*mut PEER_NODE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetNodeInfo(hgraph: *const ::core::ffi::c_void, ullnodeid: u64, ppnodeinfo: *mut *mut PEER_NODE_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_NODE_INFO = ::core::mem::zeroed();
        PeerGraphGetNodeInfo(::core::mem::transmute(hgraph), ::core::mem::transmute(ullnodeid), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_NODE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphGetProperties(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GRAPH_PROPERTIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetProperties(hgraph: *const ::core::ffi::c_void, ppgraphproperties: *mut *mut PEER_GRAPH_PROPERTIES) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_GRAPH_PROPERTIES = ::core::mem::zeroed();
        PeerGraphGetProperties(::core::mem::transmute(hgraph), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_GRAPH_PROPERTIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphGetRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut PEER_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_RECORD = ::core::mem::zeroed();
        PeerGraphGetRecord(::core::mem::transmute(hgraph), ::core::mem::transmute(precordid), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphGetStatus(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphGetStatus(hgraph: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        PeerGraphGetStatus(::core::mem::transmute(hgraph), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphImportDatabase<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzfilepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphImportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGraphImportDatabase(::core::mem::transmute(hgraph), pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphListen(hgraph: *const ::core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphListen(hgraph: *const ::core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> ::windows::core::HRESULT;
        }
        PeerGraphListen(::core::mem::transmute(hgraph), ::core::mem::transmute(dwscope), ::core::mem::transmute(dwscopeid), ::core::mem::transmute(wport)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphOpen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzgraphid: Param0, pwzpeerid: Param1, pwzdatabasename: Param2, psecurityinterface: *const PEER_SECURITY_INTERFACE, crecordtypesyncprecedence: u32, precordtypesyncprecedence: *const ::windows::core::GUID, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphOpen(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const PEER_SECURITY_INTERFACE, crecordtypesyncprecedence: u32, precordtypesyncprecedence: *const ::windows::core::GUID, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphOpen(pwzgraphid.into_param().abi(), pwzpeerid.into_param().abi(), pwzdatabasename.into_param().abi(), ::core::mem::transmute(psecurityinterface), ::core::mem::transmute(crecordtypesyncprecedence), ::core::mem::transmute(precordtypesyncprecedence), ::core::mem::transmute(phgraph)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerGraphOpenDirectConnection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzpeerid: Param1, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphOpenDirectConnection(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        PeerGraphOpenDirectConnection(::core::mem::transmute(hgraph), pwzpeerid.into_param().abi(), ::core::mem::transmute(paddress), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphPeerTimeToUniversalTime(hgraph: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphPeerTimeToUniversalTime(hgraph: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        PeerGraphPeerTimeToUniversalTime(::core::mem::transmute(hgraph), ::core::mem::transmute(pftpeertime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphRegisterEvent<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hgraph: *const ::core::ffi::c_void, hevent: Param1, ceventregistrations: u32, peventregistrations: *const PEER_GRAPH_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphRegisterEvent(hgraph: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistrations: u32, peventregistrations: *const PEER_GRAPH_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphRegisterEvent(::core::mem::transmute(hgraph), hevent.into_param().abi(), ::core::mem::transmute(ceventregistrations), ::core::mem::transmute(peventregistrations), ::core::mem::transmute(phpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphSearchRecords<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzcriteria: Param1, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphSearchRecords(hgraph: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphSearchRecords(::core::mem::transmute(hgraph), pwzcriteria.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphSendData(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphSendData(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphSendData(::core::mem::transmute(hgraph), ::core::mem::transmute(ullconnectionid), ::core::mem::transmute(ptype), ::core::mem::transmute(cbdata), ::core::mem::transmute(pvdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphSetNodeAttributes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgraph: *const ::core::ffi::c_void, pwzattributes: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphSetNodeAttributes(hgraph: *const ::core::ffi::c_void, pwzattributes: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGraphSetNodeAttributes(::core::mem::transmute(hgraph), pwzattributes.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphSetPresence<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hgraph: *const ::core::ffi::c_void, fpresent: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphSetPresence(hgraph: *const ::core::ffi::c_void, fpresent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        PeerGraphSetPresence(::core::mem::transmute(hgraph), fpresent.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphSetProperties(hgraph: *const ::core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphSetProperties(hgraph: *const ::core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> ::windows::core::HRESULT;
        }
        PeerGraphSetProperties(::core::mem::transmute(hgraph), ::core::mem::transmute(pgraphproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphShutdown() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphShutdown() -> ::windows::core::HRESULT;
        }
        PeerGraphShutdown().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphStartup(wversionrequested: u16) -> ::windows::core::Result<PEER_VERSION_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: PEER_VERSION_DATA = ::core::mem::zeroed();
        PeerGraphStartup(::core::mem::transmute(wversionrequested), ::core::mem::transmute(&mut result__)).from_abi::<PEER_VERSION_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphUniversalTimeToPeerTime(hgraph: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphUniversalTimeToPeerTime(hgraph: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        PeerGraphUniversalTimeToPeerTime(::core::mem::transmute(hgraph), ::core::mem::transmute(pftuniversaltime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGraphUnregisterEvent(::core::mem::transmute(hpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphUpdateRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphUpdateRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::HRESULT;
        }
        PeerGraphUpdateRecord(::core::mem::transmute(hgraph), ::core::mem::transmute(precord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGraphValidateDeferredRecords(hgraph: *const ::core::ffi::c_void, crecordids: u32, precordids: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGraphValidateDeferredRecords(hgraph: *const ::core::ffi::c_void, crecordids: u32, precordids: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        PeerGraphValidateDeferredRecords(::core::mem::transmute(hgraph), ::core::mem::transmute(crecordids), ::core::mem::transmute(precordids)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupAddRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupAddRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        PeerGroupAddRecord(::core::mem::transmute(hgroup), ::core::mem::transmute(precord), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupClose(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupClose(hgroup: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupClose(::core::mem::transmute(hgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupCloseDirectConnection(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupCloseDirectConnection(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::HRESULT;
        }
        PeerGroupCloseDirectConnection(::core::mem::transmute(hgroup), ::core::mem::transmute(ullconnectionid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupConnect(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupConnect(hgroup: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupConnect(::core::mem::transmute(hgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Networking_WinSock'*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGroupConnectByAddress(hgroup: *const ::core::ffi::c_void, caddresses: u32, paddresses: *const PEER_ADDRESS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupConnectByAddress(hgroup: *const ::core::ffi::c_void, caddresses: u32, paddresses: *const PEER_ADDRESS) -> ::windows::core::HRESULT;
        }
        PeerGroupConnectByAddress(::core::mem::transmute(hgroup), ::core::mem::transmute(caddresses), ::core::mem::transmute(paddresses)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupCreate(::core::mem::transmute(pproperties), ::core::mem::transmute(phgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupCreateInvitation<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzidentityinfo: Param1, pftexpiration: *const super::super::Foundation::FILETIME, croles: u32, proles: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupCreateInvitation(hgroup: *const ::core::ffi::c_void, pwzidentityinfo: super::super::Foundation::PWSTR, pftexpiration: *const super::super::Foundation::FILETIME, croles: u32, proles: *const ::windows::core::GUID, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerGroupCreateInvitation(::core::mem::transmute(hgroup), pwzidentityinfo.into_param().abi(), ::core::mem::transmute(pftexpiration), ::core::mem::transmute(croles), ::core::mem::transmute(proles), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupCreatePasswordInvitation(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupCreatePasswordInvitation(hgroup: *const ::core::ffi::c_void, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerGroupCreatePasswordInvitation(::core::mem::transmute(hgroup), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupDelete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzgrouppeername: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupDelete(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGroupDelete(pwzidentity.into_param().abi(), pwzgrouppeername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupDeleteRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupDeleteRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        PeerGroupDeleteRecord(::core::mem::transmute(hgroup), ::core::mem::transmute(precordid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupEnumConnections(hgroup: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupEnumConnections(hgroup: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupEnumConnections(::core::mem::transmute(hgroup), ::core::mem::transmute(dwflags), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupEnumMembers<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, dwflags: u32, pwzidentity: Param2, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupEnumMembers(hgroup: *const ::core::ffi::c_void, dwflags: u32, pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupEnumMembers(::core::mem::transmute(hgroup), ::core::mem::transmute(dwflags), pwzidentity.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupEnumRecords(hgroup: *const ::core::ffi::c_void, precordtype: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupEnumRecords(hgroup: *const ::core::ffi::c_void, precordtype: *const ::windows::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupEnumRecords(::core::mem::transmute(hgroup), ::core::mem::transmute(precordtype), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupExportConfig<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzpassword: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupExportConfig(hgroup: *const ::core::ffi::c_void, pwzpassword: super::super::Foundation::PWSTR, ppwzxml: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerGroupExportConfig(::core::mem::transmute(hgroup), pwzpassword.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupExportDatabase<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzfilepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupExportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGroupExportDatabase(::core::mem::transmute(hgroup), pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GROUP_EVENT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GROUP_EVENT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_GROUP_EVENT_DATA = ::core::mem::zeroed();
        PeerGroupGetEventData(::core::mem::transmute(hpeerevent), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_GROUP_EVENT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupGetProperties(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GROUP_PROPERTIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupGetProperties(hgroup: *const ::core::ffi::c_void, ppproperties: *mut *mut PEER_GROUP_PROPERTIES) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_GROUP_PROPERTIES = ::core::mem::zeroed();
        PeerGroupGetProperties(::core::mem::transmute(hgroup), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_GROUP_PROPERTIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupGetRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut PEER_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupGetRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_RECORD = ::core::mem::zeroed();
        PeerGroupGetRecord(::core::mem::transmute(hgroup), ::core::mem::transmute(precordid), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupGetStatus(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupGetStatus(hgroup: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        PeerGroupGetStatus(::core::mem::transmute(hgroup), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupImportConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pwzxml: Param0, pwzpassword: Param1, foverwrite: Param2, ppwzidentity: *mut super::super::Foundation::PWSTR, ppwzgroup: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupImportConfig(pwzxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, foverwrite: super::super::Foundation::BOOL, ppwzidentity: *mut super::super::Foundation::PWSTR, ppwzgroup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGroupImportConfig(pwzxml.into_param().abi(), pwzpassword.into_param().abi(), foverwrite.into_param().abi(), ::core::mem::transmute(ppwzidentity), ::core::mem::transmute(ppwzgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupImportDatabase<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzfilepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupImportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerGroupImportDatabase(::core::mem::transmute(hgroup), pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn PeerGroupIssueCredentials<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzsubjectidentity: Param1, pcredentialinfo: *const PEER_CREDENTIAL_INFO, dwflags: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupIssueCredentials(hgroup: *const ::core::ffi::c_void, pwzsubjectidentity: super::super::Foundation::PWSTR, pcredentialinfo: *const PEER_CREDENTIAL_INFO, dwflags: u32, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerGroupIssueCredentials(::core::mem::transmute(hgroup), pwzsubjectidentity.into_param().abi(), ::core::mem::transmute(pcredentialinfo), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupJoin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzinvitation: Param1, pwzcloud: Param2, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupJoin(pwzidentity.into_param().abi(), pwzinvitation.into_param().abi(), pwzcloud.into_param().abi(), ::core::mem::transmute(phgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupOpen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzgrouppeername: Param1, pwzcloud: Param2, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupOpen(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupOpen(pwzidentity.into_param().abi(), pwzgrouppeername.into_param().abi(), pwzcloud.into_param().abi(), ::core::mem::transmute(phgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerGroupOpenDirectConnection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzidentity: Param1, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupOpenDirectConnection(hgroup: *const ::core::ffi::c_void, pwzidentity: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        PeerGroupOpenDirectConnection(::core::mem::transmute(hgroup), pwzidentity.into_param().abi(), ::core::mem::transmute(paddress), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn PeerGroupParseInvitation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzinvitation: Param0) -> ::windows::core::Result<*mut PEER_INVITATION_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupParseInvitation(pwzinvitation: super::super::Foundation::PWSTR, ppinvitationinfo: *mut *mut PEER_INVITATION_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_INVITATION_INFO = ::core::mem::zeroed();
        PeerGroupParseInvitation(pwzinvitation.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_INVITATION_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupPasswordJoin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzinvitation: Param1, pwzpassword: Param2, pwzcloud: Param3, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupPasswordJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupPasswordJoin(pwzidentity.into_param().abi(), pwzinvitation.into_param().abi(), pwzpassword.into_param().abi(), pwzcloud.into_param().abi(), ::core::mem::transmute(phgroup)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupPeerTimeToUniversalTime(hgroup: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupPeerTimeToUniversalTime(hgroup: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        PeerGroupPeerTimeToUniversalTime(::core::mem::transmute(hgroup), ::core::mem::transmute(pftpeertime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupRegisterEvent<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hgroup: *const ::core::ffi::c_void, hevent: Param1, ceventregistration: u32, peventregistrations: *const PEER_GROUP_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupRegisterEvent(hgroup: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_GROUP_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupRegisterEvent(::core::mem::transmute(hgroup), hevent.into_param().abi(), ::core::mem::transmute(ceventregistration), ::core::mem::transmute(peventregistrations), ::core::mem::transmute(phpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupResumePasswordAuthentication(hgroup: *const ::core::ffi::c_void, hpeereventhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupResumePasswordAuthentication(hgroup: *const ::core::ffi::c_void, hpeereventhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupResumePasswordAuthentication(::core::mem::transmute(hgroup), ::core::mem::transmute(hpeereventhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupSearchRecords<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hgroup: *const ::core::ffi::c_void, pwzcriteria: Param1, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupSearchRecords(hgroup: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupSearchRecords(::core::mem::transmute(hgroup), pwzcriteria.into_param().abi(), ::core::mem::transmute(phpeerenum)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupSendData(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupSendData(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupSendData(::core::mem::transmute(hgroup), ::core::mem::transmute(ullconnectionid), ::core::mem::transmute(ptype), ::core::mem::transmute(cbdata), ::core::mem::transmute(pvdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupSetProperties(hgroup: *const ::core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupSetProperties(hgroup: *const ::core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> ::windows::core::HRESULT;
        }
        PeerGroupSetProperties(::core::mem::transmute(hgroup), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupShutdown() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupShutdown() -> ::windows::core::HRESULT;
        }
        PeerGroupShutdown().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupStartup(wversionrequested: u16) -> ::windows::core::Result<PEER_VERSION_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: PEER_VERSION_DATA = ::core::mem::zeroed();
        PeerGroupStartup(::core::mem::transmute(wversionrequested), ::core::mem::transmute(&mut result__)).from_abi::<PEER_VERSION_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupUniversalTimeToPeerTime(hgroup: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupUniversalTimeToPeerTime(hgroup: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        PeerGroupUniversalTimeToPeerTime(::core::mem::transmute(hgroup), ::core::mem::transmute(pftuniversaltime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerGroupUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerGroupUnregisterEvent(::core::mem::transmute(hpeerevent)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupUpdateRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerGroupUpdateRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::HRESULT;
        }
        PeerGroupUpdateRecord(::core::mem::transmute(hgroup), ::core::mem::transmute(precord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerHostNameToPeerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzhostname: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerHostNameToPeerName(pwzhostname: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerHostNameToPeerName(pwzhostname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityCreate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzclassifier: Param0, pwzfriendlyname: Param1, hcryptprov: usize) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityCreate(pwzclassifier: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, hcryptprov: usize, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityCreate(pwzclassifier.into_param().abi(), pwzfriendlyname.into_param().abi(), ::core::mem::transmute(hcryptprov), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityDelete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityDelete(pwzidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerIdentityDelete(pwzidentity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityExport<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzpassword: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityExport(pwzidentity: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzexportxml: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityExport(pwzidentity.into_param().abi(), pwzpassword.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityGetCryptKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0) -> ::windows::core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityGetCryptKey(pwzidentity: super::super::Foundation::PWSTR, phcryptprov: *mut usize) -> ::windows::core::HRESULT;
        }
        let mut result__: usize = ::core::mem::zeroed();
        PeerIdentityGetCryptKey(pwzidentity.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityGetDefault() -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityGetDefault(ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityGetDefault(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityGetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityGetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, ppwzfriendlyname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityGetFriendlyName(pwzidentity.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityGetXML<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityGetXML(pwzidentity: super::super::Foundation::PWSTR, ppwzidentityxml: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityGetXML(pwzidentity.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentityImport<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzimportxml: Param0, pwzpassword: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentityImport(pwzimportxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerIdentityImport(pwzimportxml.into_param().abi(), pwzpassword.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerIdentitySetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzidentity: Param0, pwzfriendlyname: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerIdentitySetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        PeerIdentitySetFriendlyName(pwzidentity.into_param().abi(), pwzfriendlyname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerNameToPeerHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzpeername: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerNameToPeerHostName(pwzpeername: super::super::Foundation::PWSTR, ppwzhostname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        PeerNameToPeerHostName(pwzpeername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerPnrpEndResolve(hresolve: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpEndResolve(hresolve: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerPnrpEndResolve(::core::mem::transmute(hresolve)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> ::windows::core::HRESULT;
        }
        PeerPnrpGetCloudInfo(::core::mem::transmute(pcnumclouds), ::core::mem::transmute(ppcloudinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerPnrpGetEndpoint(hresolve: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_PNRP_ENDPOINT_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpGetEndpoint(hresolve: *const ::core::ffi::c_void, ppendpoint: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut PEER_PNRP_ENDPOINT_INFO = ::core::mem::zeroed();
        PeerPnrpGetEndpoint(::core::mem::transmute(hresolve), ::core::mem::transmute(&mut result__)).from_abi::<*mut PEER_PNRP_ENDPOINT_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerPnrpRegister<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcwzpeername: Param0, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO, phregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpRegister(pcwzpeername: super::super::Foundation::PWSTR, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO, phregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerPnrpRegister(pcwzpeername.into_param().abi(), ::core::mem::transmute(pregistrationinfo), ::core::mem::transmute(phregistration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerPnrpResolve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcwzpeername: Param0, pcwzcloudname: Param1, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::core::HRESULT;
        }
        PeerPnrpResolve(pcwzpeername.into_param().abi(), pcwzcloudname.into_param().abi(), ::core::mem::transmute(pcendpoints), ::core::mem::transmute(ppendpoints)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerPnrpShutdown() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpShutdown() -> ::windows::core::HRESULT;
        }
        PeerPnrpShutdown().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerPnrpStartResolve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pcwzpeername: Param0, pcwzcloudname: Param1, cmaxendpoints: u32, hevent: Param3, phresolve: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpStartResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, cmaxendpoints: u32, hevent: super::super::Foundation::HANDLE, phresolve: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerPnrpStartResolve(pcwzpeername.into_param().abi(), pcwzcloudname.into_param().abi(), ::core::mem::transmute(cmaxendpoints), hevent.into_param().abi(), ::core::mem::transmute(phresolve)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerPnrpStartup(wversionrequested: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpStartup(wversionrequested: u16) -> ::windows::core::HRESULT;
        }
        PeerPnrpStartup(::core::mem::transmute(wversionrequested)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
#[inline]
pub unsafe fn PeerPnrpUnregister(hregistration: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpUnregister(hregistration: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PeerPnrpUnregister(::core::mem::transmute(hregistration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_P2P', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerPnrpUpdateRegistration(hregistration: *const ::core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeerPnrpUpdateRegistration(hregistration: *const ::core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> ::windows::core::HRESULT;
        }
        PeerPnrpUpdateRegistration(::core::mem::transmute(hregistration), ::core::mem::transmute(pregistrationinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SVCID_PNRPCLOUD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce6_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce5_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce7_00c0_4fbf_bad6_18139385a49a);
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_CLIENT_INVALID_COMPARTMENT_ID: u32 = 11506u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_CLOUD_DISABLED: u32 = 11502u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_CLOUD_IS_DEAD: u32 = 11509u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_CLOUD_IS_SEARCH_ONLY: u32 = 11505u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_CLOUD_NOT_FOUND: u32 = 11501u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_DUPLICATE_PEER_NAME: u32 = 11508u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_ERROR_BASE: u32 = 11500u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_INVALID_IDENTITY: u32 = 11503u32;
#[doc = "*Required features: 'Win32_NetworkManagement_P2P'*"]
pub const WSA_PNRP_TOO_MUCH_LOAD: u32 = 11504u32;
