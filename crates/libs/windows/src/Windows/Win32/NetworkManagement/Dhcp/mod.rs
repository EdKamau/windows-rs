#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ADDRESS_TYPE_IANA: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ADDRESS_TYPE_IATA: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CHANGESTATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CLIENT_TYPE_BOOTP: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CLIENT_TYPE_DHCP: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CLIENT_TYPE_NONE: u32 = 100u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CLIENT_TYPE_RESERVATION_FLAG: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CLIENT_TYPE_UNSPECIFIED: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DATE_TIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl ::core::marker::Copy for DATE_TIME {}
impl ::core::clone::Clone for DATE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATE_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DATE_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DATE_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DATE_TIME {}
impl ::core::default::Default for DATE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPAPI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPAPI_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPAPI_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPCAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl ::core::marker::Copy for DHCPCAPI_CLASSID {}
impl ::core::clone::Clone for DHCPCAPI_CLASSID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPCAPI_CLASSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPCAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPCAPI_CLASSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPCAPI_CLASSID {}
impl ::core::default::Default for DHCPCAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_DEREGISTER_HANDLE_EVENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPCAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPAPI_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPCAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPCAPI_PARAMS_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPCAPI_PARAMS_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPCAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPCAPI_PARAMS_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPCAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPCAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REGISTER_HANDLE_EVENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REQUEST_ASYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REQUEST_CANCEL: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REQUEST_MASK: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REQUEST_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPCAPI_REQUEST_SYNCHRONOUS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPDS_SERVER {
    pub Version: u32,
    pub ServerName: super::super::Foundation::PWSTR,
    pub ServerAddress: u32,
    pub Flags: u32,
    pub State: u32,
    pub DsLocation: super::super::Foundation::PWSTR,
    pub DsLocType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPDS_SERVER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPDS_SERVER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPDS_SERVER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPDS_SERVER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPDS_SERVER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPDS_SERVER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPDS_SERVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPDS_SERVERS {
    pub Flags: u32,
    pub NumElements: u32,
    pub Servers: *mut DHCPDS_SERVER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPDS_SERVERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPDS_SERVERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPDS_SERVERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPDS_SERVERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPDS_SERVERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPDS_SERVERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPDS_SERVERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: super::super::Foundation::PWSTR,
    pub Flags: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV4_FAILOVER_CLIENT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV4_FAILOVER_CLIENT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV4_FAILOVER_CLIENT_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: super::super::Foundation::PWSTR,
    pub Flags: u8,
    pub AddressStateEx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV4_FAILOVER_CLIENT_INFO_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6CAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl ::core::marker::Copy for DHCPV6CAPI_CLASSID {}
impl ::core::clone::Clone for DHCPV6CAPI_CLASSID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6CAPI_CLASSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6CAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6CAPI_CLASSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6CAPI_CLASSID {}
impl ::core::default::Default for DHCPV6CAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6CAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6CAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6CAPI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV6CAPI_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6CAPI_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6CAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPV6CAPI_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6CAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6CAPI_PARAMS_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV6CAPI_PARAMS_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6CAPI_PARAMS_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6Prefix {
    pub prefix: [u8; 16],
    pub prefixLength: u32,
    pub preferredLifeTime: u32,
    pub validLifeTime: u32,
    pub status: StatusCode,
}
impl ::core::marker::Copy for DHCPV6Prefix {}
impl ::core::clone::Clone for DHCPV6Prefix {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6Prefix {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6Prefix {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6Prefix>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6Prefix {}
impl ::core::default::Default for DHCPV6Prefix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6PrefixLeaseInformation {
    pub nPrefixes: u32,
    pub prefixArray: *mut DHCPV6Prefix,
    pub iaid: u32,
    pub T1: i64,
    pub T2: i64,
    pub MaxLeaseExpirationTime: i64,
    pub LastRenewalTime: i64,
    pub status: StatusCode,
    pub ServerId: *mut u8,
    pub ServerIdLen: u32,
}
impl ::core::marker::Copy for DHCPV6PrefixLeaseInformation {}
impl ::core::clone::Clone for DHCPV6PrefixLeaseInformation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6PrefixLeaseInformation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6PrefixLeaseInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6PrefixLeaseInformation>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6PrefixLeaseInformation {}
impl ::core::default::Default for DHCPV6PrefixLeaseInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: super::super::Foundation::PWSTR,
    pub IpV6IfIndex: u32,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_BIND_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV6_BIND_ELEMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_BIND_ELEMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCPV6_BIND_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_BIND_ELEMENT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV6_BIND_ELEMENT_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_BIND_ELEMENT_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCPV6_IP_ARRAY {}
impl ::core::clone::Clone for DHCPV6_IP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6_IP_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_IP_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6_IP_ARRAY {}
impl ::core::default::Default for DHCPV6_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_CLIENTID: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_DNS_SERVERS: u32 = 23u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_DOMAIN_LIST: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_IA_NA: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_IA_PD: u32 = 25u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_IA_TA: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_NISP_DOMAIN_NAME: u32 = 30u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_NISP_SERVERS: u32 = 28u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_NIS_DOMAIN_NAME: u32 = 29u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_NIS_SERVERS: u32 = 27u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_ORO: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_PREFERENCE: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_RAPID_COMMIT: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_RECONF_MSG: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_SERVERID: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_SIP_SERVERS_ADDRS: u32 = 22u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_SIP_SERVERS_NAMES: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_UNICAST: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_USER_CLASS: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_VENDOR_CLASS: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCPV6_OPTION_VENDOR_OPTS: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: super::super::Foundation::BOOL,
    pub PurgeInterval: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_STATELESS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_STATELESS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCPV6_STATELESS_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_STATELESS_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_STATELESS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_STATELESS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCPV6_STATELESS_PARAM_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpStatelessPurgeInterval: DHCPV6_STATELESS_PARAM_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpStatelessStatus: DHCPV6_STATELESS_PARAM_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: u64,
    pub NumStatelessClientsRemoved: u64,
}
impl ::core::marker::Copy for DHCPV6_STATELESS_SCOPE_STATS {}
impl ::core::clone::Clone for DHCPV6_STATELESS_SCOPE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6_STATELESS_SCOPE_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_SCOPE_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_STATELESS_SCOPE_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_SCOPE_STATS {}
impl ::core::default::Default for DHCPV6_STATELESS_SCOPE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: u32,
    pub ScopeStats: *mut DHCPV6_STATELESS_SCOPE_STATS,
}
impl ::core::marker::Copy for DHCPV6_STATELESS_STATS {}
impl ::core::clone::Clone for DHCPV6_STATELESS_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCPV6_STATELESS_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCPV6_STATELESS_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_STATS {}
impl ::core::default::Default for DHCPV6_STATELESS_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: super::super::Foundation::BOOL,
    pub HWType: u8,
    pub IsWildcard: super::super::Foundation::BOOL,
    pub Length: u8,
    pub Pattern: [u8; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ADDR_PATTERN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ADDR_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ADDR_PATTERN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ADDR_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ADDR_PATTERN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ADDR_PATTERN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: u32,
    pub NonVendorOptions: *mut DHCP_OPTION_ARRAY,
    pub NumVendorOptions: u32,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: super::super::Foundation::PWSTR,
    pub ClassName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTIONS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTIONS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTIONS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTIONS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTIONS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTION_VALUES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTION_VALUES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: super::super::Foundation::PWSTR,
    pub VendorName: super::super::Foundation::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTION_VALUES_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTION_VALUES_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_PB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTION_VALUES_PB {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTION_VALUES_PB>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: super::super::Foundation::PWSTR,
    pub VendorName: super::super::Foundation::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_PB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ALL_OPTION_VALUES_PB_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ALL_OPTION_VALUES_PB_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ATTRIB {
    pub DhcpAttribId: u32,
    pub DhcpAttribType: u32,
    pub Anonymous: DHCP_ATTRIB_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ATTRIB {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ATTRIB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ATTRIB>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ATTRIB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: super::super::Foundation::BOOL,
    pub DhcpAttribUlong: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ATTRIB_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ATTRIB_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ATTRIB_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ATTRIB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: u32,
    pub DhcpAttribs: *mut DHCP_ATTRIB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_ATTRIB_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ATTRIB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_ATTRIB_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ATTRIB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_BINARY_DATA {
    pub DataLength: u32,
    pub Data: *mut u8,
}
impl ::core::marker::Copy for DHCP_BINARY_DATA {}
impl ::core::clone::Clone for DHCP_BINARY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_BINARY_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_BINARY_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_BINARY_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_BINARY_DATA {}
impl ::core::default::Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: u32,
    pub AdapterSubnetAddress: u32,
    pub IfDescription: super::super::Foundation::PWSTR,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_BIND_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_BIND_ELEMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_BIND_ELEMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_BIND_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_BIND_ELEMENT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_BIND_ELEMENT_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_BIND_ELEMENT_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
    pub BootpAllocated: u32,
    pub MaxBootpAllowed: u32,
}
impl ::core::marker::Copy for DHCP_BOOTP_IP_RANGE {}
impl ::core::clone::Clone for DHCP_BOOTP_IP_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_BOOTP_IP_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_BOOTP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_BOOTP_IP_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_BOOTP_IP_RANGE {}
impl ::core::default::Default for DHCP_BOOTP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CALLOUT_TABLE {
    pub DhcpControlHook: LPDHCP_CONTROL,
    pub DhcpNewPktHook: LPDHCP_NEWPKT,
    pub DhcpPktDropHook: LPDHCP_DROP_SEND,
    pub DhcpPktSendHook: LPDHCP_DROP_SEND,
    pub DhcpAddressDelHook: LPDHCP_PROB,
    pub DhcpAddressOfferHook: LPDHCP_GIVE_ADDRESS,
    pub DhcpHandleOptionsHook: LPDHCP_HANDLE_OPTIONS,
    pub DhcpDeleteClientHook: LPDHCP_DELETE_CLIENT,
    pub DhcpExtensionHook: *mut ::core::ffi::c_void,
    pub DhcpReservedHook: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CALLOUT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CALLOUT_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CALLOUT_TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CALLOUT_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CALLOUT_TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CALLOUT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CALLOUT_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO {
    pub ClassName: super::super::Foundation::PWSTR,
    pub ClassComment: super::super::Foundation::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLASS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLASS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLASS_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLASS_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO_V6,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLASS_INFO_ARRAY_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLASS_INFO_ARRAY_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: super::super::Foundation::PWSTR,
    pub ClassComment: super::super::Foundation::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub EnterpriseNumber: u32,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLASS_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLASS_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CLIENT_BOOTP: u32 = 805306371u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CLIENT_DHCP: u32 = 805306372u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_FILTER_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_FILTER_STATUS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_FILTER_STATUS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V4,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_ARRAY_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_ARRAY_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V5,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_ARRAY_V5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_ARRAY_V5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V6,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_ARRAY_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_ARRAY_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_VQ,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_ARRAY_VQ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_ARRAY_VQ>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: super::super::Foundation::PWSTR,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_EX_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_EX_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_EX_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_PB {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_PB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_PB {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_PB>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_PB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_PB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_PB_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_PB_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_PB_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_V4 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_V5 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_V5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_V5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_V6 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub AddressType: u32,
    pub IAID: u32,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientValidLeaseExpires: DATE_TIME,
    pub ClientPrefLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO_V6,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_VQ {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
    pub ClientComment: super::super::Foundation::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_CLIENT_INFO_VQ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_CLIENT_INFO_VQ>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DHCP_CLIENT_SEARCH_UNION(pub u8);
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CONTROL_CONTINUE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CONTROL_PAUSE: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CONTROL_START: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_CONTROL_STOP: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_DUPLICATE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_GEN_FAILURE: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_INTERNAL_ERROR: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_INVALID: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_NOADDRESS: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_NOMEM: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_NO_SUBNETS: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_PAUSED: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_PROCESSED: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_TIMEOUT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_UNAUTH: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_DROP_WRONG_SERVER: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FAILOVER_DELETE_SCOPES: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: u32 = 400u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FAILOVER_MAX_NUM_REL: u32 = 31u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_FAILOVER_MODE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const LoadBalance: DHCP_FAILOVER_MODE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const HotStandby: DHCP_FAILOVER_MODE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FAILOVER_RELATIONSHIP {
    pub PrimaryServer: u32,
    pub SecondaryServer: u32,
    pub Mode: DHCP_FAILOVER_MODE,
    pub ServerType: DHCP_FAILOVER_SERVER,
    pub State: FSM_STATE,
    pub PrevState: FSM_STATE,
    pub Mclt: u32,
    pub SafePeriod: u32,
    pub RelationshipName: super::super::Foundation::PWSTR,
    pub PrimaryServerName: super::super::Foundation::PWSTR,
    pub SecondaryServerName: super::super::Foundation::PWSTR,
    pub pScopes: *mut DHCP_IP_ARRAY,
    pub Percentage: u8,
    pub SharedSecret: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FAILOVER_RELATIONSHIP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FAILOVER_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FAILOVER_RELATIONSHIP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FAILOVER_RELATIONSHIP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: u32,
    pub pRelationships: *mut DHCP_FAILOVER_RELATIONSHIP,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FAILOVER_RELATIONSHIP_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FAILOVER_RELATIONSHIP_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_FAILOVER_SERVER = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PrimaryServer: DHCP_FAILOVER_SERVER = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const SecondaryServer: DHCP_FAILOVER_SERVER = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: u32,
    pub AddrFree: u32,
    pub AddrInUse: u32,
    pub PartnerAddrFree: u32,
    pub ThisAddrFree: u32,
    pub PartnerAddrInUse: u32,
    pub ThisAddrInUse: u32,
}
impl ::core::marker::Copy for DHCP_FAILOVER_STATISTICS {}
impl ::core::clone::Clone for DHCP_FAILOVER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_FAILOVER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FAILOVER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_STATISTICS {}
impl ::core::default::Default for DHCP_FAILOVER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: super::super::Foundation::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_ADD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_ADD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FILTER_ADD_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ADD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FILTER_ADD_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ADD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ADD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: u32,
    pub pEnumRecords: *mut DHCP_FILTER_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_ENUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_ENUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FILTER_ENUM_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ENUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FILTER_ENUM_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ENUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ENUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: super::super::Foundation::BOOL,
    pub EnforceDenyList: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_GLOBAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_GLOBAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FILTER_GLOBAL_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_GLOBAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FILTER_GLOBAL_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_GLOBAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_GLOBAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_FILTER_LIST_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Deny: DHCP_FILTER_LIST_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Allow: DHCP_FILTER_LIST_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_FILTER_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_FILTER_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_FORCE_FLAG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpFullForce: DHCP_FORCE_FLAG = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpNoForce: DHCP_FORCE_FLAG = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpFailoverForce: DHCP_FORCE_FLAG = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_GIVE_ADDRESS_NEW: u32 = 805306369u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_GIVE_ADDRESS_OLD: u32 = 805306370u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_HOST_INFO {
    pub IpAddress: u32,
    pub NetBiosName: super::super::Foundation::PWSTR,
    pub HostName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_HOST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_HOST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_HOST_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_HOST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_HOST_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_HOST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_HOST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: super::super::Foundation::PWSTR,
    pub HostName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_HOST_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_HOST_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_HOST_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_HOST_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_HOST_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_HOST_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_HOST_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: u64,
    pub LowOrderBits: u64,
}
impl ::core::marker::Copy for DHCP_IPV6_ADDRESS {}
impl ::core::clone::Clone for DHCP_IPV6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IPV6_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IPV6_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IPV6_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IPV6_ADDRESS {}
impl ::core::default::Default for DHCP_IPV6_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut u32,
}
impl ::core::marker::Copy for DHCP_IP_ARRAY {}
impl ::core::clone::Clone for DHCP_IP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_ARRAY {}
impl ::core::default::Default for DHCP_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: u32,
    pub ClusterMask: u32,
}
impl ::core::marker::Copy for DHCP_IP_CLUSTER {}
impl ::core::clone::Clone for DHCP_IP_CLUSTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_CLUSTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_CLUSTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_CLUSTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_CLUSTER {}
impl ::core::default::Default for DHCP_IP_CLUSTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
}
impl ::core::marker::Copy for DHCP_IP_RANGE {}
impl ::core::clone::Clone for DHCP_IP_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE {}
impl ::core::default::Default for DHCP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IP_RANGE,
}
impl ::core::marker::Copy for DHCP_IP_RANGE_ARRAY {}
impl ::core::clone::Clone for DHCP_IP_RANGE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RANGE_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RANGE_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_ARRAY {}
impl ::core::default::Default for DHCP_IP_RANGE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCP_IP_RANGE_V6 {}
impl ::core::clone::Clone for DHCP_IP_RANGE_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RANGE_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RANGE_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_V6 {}
impl ::core::default::Default for DHCP_IP_RANGE_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RESERVATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RESERVATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION {}
impl ::core::default::Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: DHCP_BINARY_DATA,
    pub ReservedClientName: super::super::Foundation::PWSTR,
    pub ReservedClientDesc: super::super::Foundation::PWSTR,
    pub bAllowedClientTypes: u8,
    pub fOptionsPresent: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_IP_RESERVATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_IP_RESERVATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_IP_RESERVATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RESERVATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_IP_RESERVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub bAllowedClientTypes: u8,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION_V4 {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RESERVATION_V4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RESERVATION_V4>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V4 {}
impl ::core::default::Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub InterfaceId: u32,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION_V6 {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_IP_RESERVATION_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_IP_RESERVATION_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V6 {}
impl ::core::default::Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_MAX_DELAY: u32 = 1000u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_MIB_INFO {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO,
}
impl ::core::marker::Copy for DHCP_MIB_INFO {}
impl ::core::clone::Clone for DHCP_MIB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_MIB_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_MIB_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO {}
impl ::core::default::Default for DHCP_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_MIB_INFO_V5 {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub DelayedOffers: u32,
    pub ScopesWithDelayedOffers: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V5,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_V5 {}
impl ::core::clone::Clone for DHCP_MIB_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_MIB_INFO_V5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_MIB_INFO_V5>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V5 {}
impl ::core::default::Default for DHCP_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_MIB_INFO_V6 {
    pub Solicits: u32,
    pub Advertises: u32,
    pub Requests: u32,
    pub Renews: u32,
    pub Rebinds: u32,
    pub Replies: u32,
    pub Confirms: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub Informs: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V6,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_V6 {}
impl ::core::clone::Clone for DHCP_MIB_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_MIB_INFO_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_MIB_INFO_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V6 {}
impl ::core::default::Default for DHCP_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_MIB_INFO_VQ {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_VQ,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_VQ {}
impl ::core::clone::Clone for DHCP_MIB_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_MIB_INFO_VQ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_MIB_INFO_VQ>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_VQ {}
impl ::core::default::Default for DHCP_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_MIN_DELAY: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION {
    pub OptionID: u32,
    pub OptionName: super::super::Foundation::PWSTR,
    pub OptionComment: super::super::Foundation::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: u32,
    pub Options: *mut DHCP_OPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_DATA {
    pub NumElements: u32,
    pub Elements: *mut DHCP_OPTION_DATA_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_DATA_ELEMENT {
    pub OptionType: DHCP_OPTION_DATA_TYPE,
    pub Element: DHCP_OPTION_DATA_ELEMENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_DATA_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_DATA_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_DATA_ELEMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_DATA_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_DATA_ELEMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_DATA_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_OPTION_DATA_ELEMENT_0 {
    pub ByteOption: u8,
    pub WordOption: u16,
    pub DWordOption: u32,
    pub DWordDWordOption: DWORD_DWORD,
    pub IpAddressOption: u32,
    pub StringDataOption: super::super::Foundation::PWSTR,
    pub BinaryDataOption: DHCP_BINARY_DATA,
    pub EncapsulatedDataOption: DHCP_BINARY_DATA,
    pub Ipv6AddressDataOption: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_DATA_ELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_DATA_ELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_DATA_ELEMENT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_DATA_ELEMENT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_DATA_ELEMENT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_DATA_ELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_OPTION_DATA_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpByteOption: DHCP_OPTION_DATA_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpWordOption: DHCP_OPTION_DATA_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpDWordOption: DHCP_OPTION_DATA_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpDWordDWordOption: DHCP_OPTION_DATA_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpAddressOption: DHCP_OPTION_DATA_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpStringDataOption: DHCP_OPTION_DATA_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpBinaryDataOption: DHCP_OPTION_DATA_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpEncapsulatedDataOption: DHCP_OPTION_DATA_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpv6AddressOption: DHCP_OPTION_DATA_TYPE = 8i32;
#[repr(C)]
pub struct DHCP_OPTION_ELEMENT_UNION(pub u8);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: u32,
    pub Options: *mut DHCP_OPTION_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_SCOPE_INFO {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_SCOPE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_SCOPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_SCOPE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_SCOPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_OPTION_SCOPE_INFO_0 {
    pub DefaultScopeInfo: *mut ::core::ffi::c_void,
    pub GlobalScopeInfo: *mut ::core::ffi::c_void,
    pub SubnetScopeInfo: u32,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE,
    pub MScopeInfo: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_SCOPE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_SCOPE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_SCOPE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_SCOPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_OPTION_SCOPE_INFO6 {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE6,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO6_0,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO6 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_OPTION_SCOPE_INFO6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_SCOPE_INFO6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_SCOPE_INFO6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_SCOPE_INFO6 {}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub union DHCP_OPTION_SCOPE_INFO6_0 {
    pub DefaultScopeInfo: *mut ::core::ffi::c_void,
    pub SubnetScopeInfo: DHCP_IPV6_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE6,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO6_0 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_OPTION_SCOPE_INFO6_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_SCOPE_INFO6_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_SCOPE_INFO6_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_SCOPE_INFO6_0 {}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_OPTION_SCOPE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpDefaultOptions: DHCP_OPTION_SCOPE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpGlobalOptions: DHCP_OPTION_SCOPE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetOptions: DHCP_OPTION_SCOPE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpReservedOptions: DHCP_OPTION_SCOPE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpMScopeOptions: DHCP_OPTION_SCOPE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_OPTION_SCOPE_TYPE6 = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpDefaultOptions6: DHCP_OPTION_SCOPE_TYPE6 = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpScopeOptions6: DHCP_OPTION_SCOPE_TYPE6 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpReservedOptions6: DHCP_OPTION_SCOPE_TYPE6 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpGlobalOptions6: DHCP_OPTION_SCOPE_TYPE6 = 3i32;
#[repr(C)]
pub struct DHCP_OPTION_SCOPE_UNION6(pub u8);
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_OPTION_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUnaryElementTypeOption: DHCP_OPTION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpArrayTypeOption: DHCP_OPTION_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: u32,
    pub Value: DHCP_OPTION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: u32,
    pub Values: *mut DHCP_OPTION_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_OPTION_VALUE_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_OPTION_VALUE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_OPTION_VALUE_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_OPTION_VALUE_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_OPTION_VALUE_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_OPTION_VALUE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_PERF_STATS {
    pub dwNumPacketsReceived: u32,
    pub dwNumPacketsDuplicate: u32,
    pub dwNumPacketsExpired: u32,
    pub dwNumMilliSecondsProcessed: u32,
    pub dwNumPacketsInActiveQueue: u32,
    pub dwNumPacketsInPingQueue: u32,
    pub dwNumDiscoversReceived: u32,
    pub dwNumOffersSent: u32,
    pub dwNumRequestsReceived: u32,
    pub dwNumInformsReceived: u32,
    pub dwNumAcksSent: u32,
    pub dwNumNacksSent: u32,
    pub dwNumDeclinesReceived: u32,
    pub dwNumReleasesReceived: u32,
    pub dwNumDelayedOfferInQueue: u32,
    pub dwNumPacketsProcessed: u32,
    pub dwNumPacketsInQuarWaitingQueue: u32,
    pub dwNumPacketsInQuarReadyQueue: u32,
    pub dwNumPacketsInQuarDecisionQueue: u32,
}
impl ::core::marker::Copy for DHCP_PERF_STATS {}
impl ::core::clone::Clone for DHCP_PERF_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_PERF_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_PERF_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_PERF_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_PERF_STATS {}
impl ::core::default::Default for DHCP_PERF_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY {
    pub PolicyName: super::super::Foundation::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: super::super::Foundation::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POLICY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POLICY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POLICY_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POLICY_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_EX {
    pub PolicyName: super::super::Foundation::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: super::super::Foundation::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POLICY_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POLICY_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_EX_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POLICY_EX_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POLICY_EX_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_POLICY_FIELDS_TO_UPDATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyName: DHCP_POLICY_FIELDS_TO_UPDATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyOrder: DHCP_POLICY_FIELDS_TO_UPDATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyExpr: DHCP_POLICY_FIELDS_TO_UPDATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyRanges: DHCP_POLICY_FIELDS_TO_UPDATE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyDescr: DHCP_POLICY_FIELDS_TO_UPDATE = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyStatus: DHCP_POLICY_FIELDS_TO_UPDATE = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpUpdatePolicyDnsSuffix: DHCP_POLICY_FIELDS_TO_UPDATE = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_POL_ATTR_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpAttrHWAddr: DHCP_POL_ATTR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpAttrOption: DHCP_POL_ATTR_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpAttrSubOption: DHCP_POL_ATTR_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpAttrFqdn: DHCP_POL_ATTR_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpAttrFqdnSingleLabel: DHCP_POL_ATTR_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_POL_COMPARATOR = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompEqual: DHCP_POL_COMPARATOR = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompNotEqual: DHCP_POL_COMPARATOR = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompBeginsWith: DHCP_POL_COMPARATOR = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompNotBeginWith: DHCP_POL_COMPARATOR = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompEndsWith: DHCP_POL_COMPARATOR = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpCompNotEndWith: DHCP_POL_COMPARATOR = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POL_COND {
    pub ParentExpr: u32,
    pub Type: DHCP_POL_ATTR_TYPE,
    pub OptionID: u32,
    pub SubOptionID: u32,
    pub VendorName: super::super::Foundation::PWSTR,
    pub Operator: DHCP_POL_COMPARATOR,
    pub Value: *mut u8,
    pub ValueLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POL_COND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POL_COND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POL_COND {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POL_COND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POL_COND>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POL_COND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POL_COND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_COND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POL_COND_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POL_COND_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_POL_COND_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POL_COND_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POL_COND_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POL_COND_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POL_COND_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: u32,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
impl ::core::marker::Copy for DHCP_POL_EXPR {}
impl ::core::clone::Clone for DHCP_POL_EXPR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_POL_EXPR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POL_EXPR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR {}
impl ::core::default::Default for DHCP_POL_EXPR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_EXPR,
}
impl ::core::marker::Copy for DHCP_POL_EXPR_ARRAY {}
impl ::core::clone::Clone for DHCP_POL_EXPR_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_POL_EXPR_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_POL_EXPR_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR_ARRAY {}
impl ::core::default::Default for DHCP_POL_EXPR_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_POL_LOGIC_OPER = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpLogicalOr: DHCP_POL_LOGIC_OPER = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpLogicalAnd: DHCP_POL_LOGIC_OPER = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_PROB_CONFLICT: u32 = 536870913u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_PROB_DECLINE: u32 = 536870914u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_PROB_NACKED: u32 = 536870916u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_PROB_RELEASE: u32 = 536870915u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_PROPERTY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_PROPERTY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_PROPERTY_0 {
    pub ByteValue: u8,
    pub WordValue: u16,
    pub DWordValue: u32,
    pub StringValue: super::super::Foundation::PWSTR,
    pub BinaryValue: DHCP_BINARY_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_PROPERTY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_PROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_PROPERTY_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_PROPERTY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_PROPERTY_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_PROPERTY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_PROPERTY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_PROPERTY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_PROPERTY_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_PROPERTY_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_PROPERTY_ID = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropIdPolicyDnsSuffix: DHCP_PROPERTY_ID = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropIdClientAddressStateEx: DHCP_PROPERTY_ID = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_PROPERTY_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropTypeByte: DHCP_PROPERTY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropTypeWord: DHCP_PROPERTY_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropTypeDword: DHCP_PROPERTY_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropTypeString: DHCP_PROPERTY_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpPropTypeBinary: DHCP_PROPERTY_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut *mut DHCP_IP_RESERVATION_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_RESERVATION_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_RESERVATION_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_RESERVATION_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_RESERVATION_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_RESERVATION_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_RESERVATION_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: u32,
    pub ReservedIpSubnetAddress: u32,
}
impl ::core::marker::Copy for DHCP_RESERVED_SCOPE {}
impl ::core::clone::Clone for DHCP_RESERVED_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_RESERVED_SCOPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_RESERVED_SCOPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE {}
impl ::core::default::Default for DHCP_RESERVED_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCP_RESERVED_SCOPE6 {}
impl ::core::clone::Clone for DHCP_RESERVED_SCOPE6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_RESERVED_SCOPE6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_RESERVED_SCOPE6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE6 {}
impl ::core::default::Default for DHCP_RESERVED_SCOPE6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SCAN_FLAG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpRegistryFix: DHCP_SCAN_FLAG = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpDatabaseFix: DHCP_SCAN_FLAG = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: u32,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
impl ::core::marker::Copy for DHCP_SCAN_ITEM {}
impl ::core::clone::Clone for DHCP_SCAN_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_SCAN_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_SCAN_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SCAN_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_ITEM {}
impl ::core::default::Default for DHCP_SCAN_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: u32,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
impl ::core::marker::Copy for DHCP_SCAN_LIST {}
impl ::core::clone::Clone for DHCP_SCAN_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_SCAN_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_SCAN_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SCAN_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_LIST {}
impl ::core::default::Default for DHCP_SCAN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SEARCH_INFO {
    pub SearchType: DHCP_SEARCH_INFO_TYPE,
    pub SearchInfo: DHCP_SEARCH_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SEARCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SEARCH_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SEARCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SEARCH_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_SEARCH_INFO_0 {
    pub ClientIpAddress: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SEARCH_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SEARCH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SEARCH_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SEARCH_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SEARCH_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SEARCH_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SEARCH_INFO_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpClientIpAddress: DHCP_SEARCH_INFO_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpClientHardwareAddress: DHCP_SEARCH_INFO_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpClientName: DHCP_SEARCH_INFO_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SEARCH_INFO_TYPE_V6 = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6ClientIpAddress: DHCP_SEARCH_INFO_TYPE_V6 = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6ClientDUID: DHCP_SEARCH_INFO_TYPE_V6 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6ClientName: DHCP_SEARCH_INFO_TYPE_V6 = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SEARCH_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SEARCH_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SEARCH_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SEARCH_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SEARCH_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SEARCH_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_SEARCH_INFO_V6_0 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub ClientName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SEARCH_INFO_V6_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SEARCH_INFO_V6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SEARCH_INFO_V6_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SEARCH_INFO_V6_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SEARCH_INFO_V6_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SEARCH_INFO_V6_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_SEND_PACKET: u32 = 268435456u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO {
    pub APIProtocolSupport: u32,
    pub DatabaseName: super::super::Foundation::PWSTR,
    pub DatabasePath: super::super::Foundation::PWSTR,
    pub BackupPath: super::super::Foundation::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_CONFIG_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_CONFIG_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_V4 {
    pub APIProtocolSupport: u32,
    pub DatabaseName: super::super::Foundation::PWSTR,
    pub DatabasePath: super::super::Foundation::PWSTR,
    pub BackupPath: super::super::Foundation::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: super::super::Foundation::PWSTR,
    pub fAuditLog: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_CONFIG_INFO_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_CONFIG_INFO_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: super::super::Foundation::BOOL,
    pub RapidCommitFlag: super::super::Foundation::BOOL,
    pub PreferredLifetime: u32,
    pub ValidLifetime: u32,
    pub T1: u32,
    pub T2: u32,
    pub PreferredLifetimeIATA: u32,
    pub ValidLifetimeIATA: u32,
    pub fAuditLog: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_CONFIG_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_CONFIG_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_VQ {
    pub APIProtocolSupport: u32,
    pub DatabaseName: super::super::Foundation::PWSTR,
    pub DatabasePath: super::super::Foundation::PWSTR,
    pub BackupPath: super::super::Foundation::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: super::super::Foundation::PWSTR,
    pub fAuditLog: super::super::Foundation::BOOL,
    pub QuarantineOn: super::super::Foundation::BOOL,
    pub QuarDefFail: u32,
    pub QuarRuntimeStatus: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_CONFIG_INFO_VQ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_CONFIG_INFO_VQ>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_OPTIONS {
    pub MessageType: *mut u8,
    pub SubnetMask: *mut u32,
    pub RequestedAddress: *mut u32,
    pub RequestLeaseTime: *mut u32,
    pub OverlayFields: *mut u8,
    pub RouterAddress: *mut u32,
    pub Server: *mut u32,
    pub ParameterRequestList: *mut u8,
    pub ParameterRequestListLength: u32,
    pub MachineName: super::super::Foundation::PSTR,
    pub MachineNameLength: u32,
    pub ClientHardwareAddressType: u8,
    pub ClientHardwareAddressLength: u8,
    pub ClientHardwareAddress: *mut u8,
    pub ClassIdentifier: super::super::Foundation::PSTR,
    pub ClassIdentifierLength: u32,
    pub VendorClass: *mut u8,
    pub VendorClassLength: u32,
    pub DNSFlags: u32,
    pub DNSNameLength: u32,
    pub DNSName: *mut u8,
    pub DSDomainNameRequested: super::super::Foundation::BOOLEAN,
    pub DSDomainName: super::super::Foundation::PSTR,
    pub DSDomainNameLen: u32,
    pub ScopeId: *mut u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_OPTIONS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_OPTIONS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_OPTIONS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_OPTIONS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_OPTIONS {
    pub MessageType: *mut u8,
    pub SubnetMask: *mut u32,
    pub RequestedAddress: *mut u32,
    pub RequestLeaseTime: *mut u32,
    pub OverlayFields: *mut u8,
    pub RouterAddress: *mut u32,
    pub Server: *mut u32,
    pub ParameterRequestList: *mut u8,
    pub ParameterRequestListLength: u32,
    pub MachineName: super::super::Foundation::PSTR,
    pub MachineNameLength: u32,
    pub ClientHardwareAddressType: u8,
    pub ClientHardwareAddressLength: u8,
    pub ClientHardwareAddress: *mut u8,
    pub ClassIdentifier: super::super::Foundation::PSTR,
    pub ClassIdentifierLength: u32,
    pub VendorClass: *mut u8,
    pub VendorClassLength: u32,
    pub DNSFlags: u32,
    pub DNSNameLength: u32,
    pub DNSName: *mut u8,
    pub DSDomainNameRequested: super::super::Foundation::BOOLEAN,
    pub DSDomainName: super::super::Foundation::PSTR,
    pub DSDomainNameLen: u32,
    pub ScopeId: *mut u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_OPTIONS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_OPTIONS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_OPTIONS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_OPTIONS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: super::super::Foundation::PWSTR,
    pub DefaultUserClassName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_SPECIFIC_STRINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_SPECIFIC_STRINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SERVER_SPECIFIC_STRINGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_SPECIFIC_STRINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SERVER_SPECIFIC_STRINGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_SPECIFIC_STRINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_SPECIFIC_STRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_SUBNET_ELEMENT_DATA_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V4_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V4_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V4_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V4_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V5_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    pub IpRange: *mut DHCP_BOOTP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V5_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V5_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V5_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE_V6,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V6_0,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V6 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub union DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    pub IpRange: *mut DHCP_IP_RANGE_V6,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V6,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE_V6,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V6_0 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_DATA_V6_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_DATA_V6_0 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_INFO_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V4,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V5,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V6,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SUBNET_ELEMENT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpRanges: DHCP_SUBNET_ELEMENT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSecondaryHosts: DHCP_SUBNET_ELEMENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpReservedIps: DHCP_SUBNET_ELEMENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpUsedClusters: DHCP_SUBNET_ELEMENT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpRangesDhcpOnly: DHCP_SUBNET_ELEMENT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpRangesDhcpBootp: DHCP_SUBNET_ELEMENT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpIpRangesBootpOnly: DHCP_SUBNET_ELEMENT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SUBNET_ELEMENT_TYPE_V6 = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6IpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6ReservedIps: DHCP_SUBNET_ELEMENT_TYPE_V6 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Dhcpv6ExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = 2i32;
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION(pub u8);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V4(pub u8);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V6(pub u8);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: super::super::Foundation::PWSTR,
    pub SubnetComment: super::super::Foundation::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: u32,
    pub Preference: u16,
    pub SubnetName: super::super::Foundation::PWSTR,
    pub SubnetComment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub ScopeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_INFO_V6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_INFO_V6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUBNET_INFO_VQ {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: super::super::Foundation::PWSTR,
    pub SubnetComment: super::super::Foundation::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
    pub QuarantineOn: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub Reserved4: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUBNET_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUBNET_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUBNET_INFO_VQ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUBNET_INFO_VQ>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUBNET_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type DHCP_SUBNET_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetEnabled: DHCP_SUBNET_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetDisabled: DHCP_SUBNET_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetEnabledSwitched: DHCP_SUBNET_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetDisabledSwitched: DHCP_SUBNET_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DhcpSubnetInvalidState: DHCP_SUBNET_STATE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: u32,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUPER_SCOPE_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUPER_SCOPE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUPER_SCOPE_TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUPER_SCOPE_TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: u32,
    pub SuperScopeNumber: u32,
    pub NextInSuperScope: u32,
    pub SuperScopeName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SUPER_SCOPE_TABLE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DHCP_SUPER_SCOPE_TABLE_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_ENABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct DWORD_DWORD {
    pub DWord1: u32,
    pub DWord2: u32,
}
impl ::core::marker::Copy for DWORD_DWORD {}
impl ::core::clone::Clone for DWORD_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DWORD_DWORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_DWORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWORD_DWORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for DWORD_DWORD {}
impl ::core::default::Default for DWORD_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddFilterV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddFilterV4(serveripaddress: super::super::Foundation::PWSTR, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DhcpAddFilterV4(serveripaddress.into_param().abi(), ::core::mem::transmute(addfilterinfo), forceflag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddSecurityGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pserver: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddSecurityGroup(pserver: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpAddSecurityGroup(pserver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpAddServer(::core::mem::transmute(flags), ::core::mem::transmute(idinfo), ::core::mem::transmute(newserver), ::core::mem::transmute(callbackfn), ::core::mem::transmute(callbackdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddSubnetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32;
        }
        ::core::mem::transmute(DhcpAddSubnetElement(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(addelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddSubnetElementV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32;
        }
        ::core::mem::transmute(DhcpAddSubnetElementV4(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(addelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddSubnetElementV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32;
        }
        ::core::mem::transmute(DhcpAddSubnetElementV5(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(addelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddSubnetElementV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAddSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32;
        }
        ::core::mem::transmute(DhcpAddSubnetElementV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(addelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAuditLogGetParams<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, auditlogdir: *mut super::super::Foundation::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAuditLogGetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: *mut super::super::Foundation::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpAuditLogGetParams(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(auditlogdir), ::core::mem::transmute(diskcheckinterval), ::core::mem::transmute(maxlogfilessize), ::core::mem::transmute(minspaceondisk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAuditLogSetParams<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, auditlogdir: Param2, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpAuditLogSetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: super::super::Foundation::PWSTR, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32;
        }
        ::core::mem::transmute(DhcpAuditLogSetParams(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), auditlogdir.into_param().abi(), ::core::mem::transmute(diskcheckinterval), ::core::mem::transmute(maxlogfilessize), ::core::mem::transmute(minspaceondisk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpCApiCleanup() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCApiCleanup();
        }
        DhcpCApiCleanup()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpCApiInitialize(version: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCApiInitialize(version: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpCApiInitialize(::core::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpCreateClass(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(classinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClassV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpCreateClassV6(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(classinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpCreateClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClientInfoV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
        }
        ::core::mem::transmute(DhcpCreateClientInfoV4(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClientInfoVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpCreateClientInfoVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateOption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpCreateOption(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateOptionV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpCreateOptionV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateOptionV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpCreateOptionV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateSubnet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpCreateSubnet(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateSubnetV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpCreateSubnetV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateSubnetVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpCreateSubnetVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpCreateSubnetVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpDeRegisterParamChange(::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(event)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteClass(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), classname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteClassV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteClassV6(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), classname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_SEARCH_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteClientInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteClientInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteFilterV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteFilterV4(serveripaddress: super::super::Foundation::PWSTR, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteFilterV4(serveripaddress.into_param().abi(), ::core::mem::transmute(deletefilterinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteServer(::core::mem::transmute(flags), ::core::mem::transmute(idinfo), ::core::mem::transmute(newserver), ::core::mem::transmute(callbackfn), ::core::mem::transmute(callbackdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteSubnet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteSubnet(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteSubnetV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteSubnetV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteSuperScopeV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, superscopename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDeleteSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, superscopename: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpDeleteSuperScopeV4(serveripaddress.into_param().abi(), superscopename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpDsCleanup() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDsCleanup();
        }
        DhcpDsCleanup()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpDsInit() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpDsInit() -> u32;
        }
        ::core::mem::transmute(DhcpDsInit())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumClasses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumClasses(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumClasses(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(classinfoarray), ::core::mem::transmute(nread), ::core::mem::transmute(ntotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumClassesV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumClassesV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumClassesV6(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(classinfoarray), ::core::mem::transmute(nread), ::core::mem::transmute(ntotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumFilterV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumFilterV4(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumFilterV4(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(listtype), ::core::mem::transmute(enumfilterinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptionValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptionValues(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(optionvalues), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptionValuesV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, classname: Param2, vendorname: Param3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptionValuesV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(optionvalues), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptionValuesV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, classname: Param2, vendorname: Param3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptionValuesV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(optionvalues), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptions(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptions(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(options), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptionsV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, classname: Param2, vendorname: Param3, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptionsV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptionsV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(options), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumOptionsV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, classname: Param2, vendorname: Param3, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumOptionsV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(options), ::core::mem::transmute(optionsread), ::core::mem::transmute(optionstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumServers(flags: u32, idinfo: *mut ::core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumServers(flags: u32, idinfo: *mut ::core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpEnumServers(::core::mem::transmute(flags), ::core::mem::transmute(idinfo), ::core::mem::transmute(servers), ::core::mem::transmute(callbackfn), ::core::mem::transmute(callbackdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClients<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClients(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsFilterStatusInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClientsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClientsV4(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClientsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClientsV5(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClientsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClientsV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetClientsVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetClientsVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetElements(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetElements(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enumelementtype), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enumelementinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetElementsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetElementsV4(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enumelementtype), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enumelementinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetElementsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetElementsV5(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enumelementtype), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enumelementinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetElementsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetElementsV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(enumelementtype), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enumelementinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnets(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnets(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enuminfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetsV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpEnumSubnetsV6(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpEnumSubnetsV6(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enuminfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptionValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
        }
        ::core::mem::transmute(DhcpGetAllOptionValues(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptionValuesV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetAllOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
        }
        ::core::mem::transmute(DhcpGetAllOptionValuesV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetAllOptions(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
        }
        ::core::mem::transmute(DhcpGetAllOptions(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionstruct)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptionsV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetAllOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
        }
        ::core::mem::transmute(DhcpGetAllOptionsV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionstruct)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClassInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClassInfo(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpGetClassInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(partialclassinfo), ::core::mem::transmute(filledclassinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpGetClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientInfoV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32;
        }
        ::core::mem::transmute(DhcpGetClientInfoV4(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpGetClientInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientInfoVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpGetClientInfoVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetClientOptions(serveripaddress: super::super::Foundation::PWSTR, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32;
        }
        ::core::mem::transmute(DhcpGetClientOptions(serveripaddress.into_param().abi(), ::core::mem::transmute(clientipaddress), ::core::mem::transmute(clientsubnetmask), ::core::mem::transmute(clientoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetFilterV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpGetFilterV4(serveripaddress.into_param().abi(), ::core::mem::transmute(globalfilterinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetMibInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetMibInfo(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpGetMibInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(mibinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetMibInfoV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetMibInfoV5(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32;
        }
        ::core::mem::transmute(DhcpGetMibInfoV5(serveripaddress.into_param().abi(), ::core::mem::transmute(mibinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetMibInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetMibInfoV6(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpGetMibInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(mibinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionInfoV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionInfoV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionValueV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionValueV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOptionValueV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
        }
        ::core::mem::transmute(DhcpGetOptionValueV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetOriginalSubnetMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(sadaptername: Param0, dwsubnetmask: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetOriginalSubnetMask(sadaptername: super::super::Foundation::PWSTR, dwsubnetmask: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpGetOriginalSubnetMask(sadaptername.into_param().abi(), ::core::mem::transmute(dwsubnetmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpGetServerBindingInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(bindelementsinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpGetServerBindingInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(bindelementsinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetServerSpecificStrings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetServerSpecificStrings(serveripaddress: super::super::Foundation::PWSTR, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32;
        }
        ::core::mem::transmute(DhcpGetServerSpecificStrings(serveripaddress.into_param().abi(), ::core::mem::transmute(serverspecificstrings)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetSubnetDelayOffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32;
        }
        ::core::mem::transmute(DhcpGetSubnetDelayOffer(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(timedelayinmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetSubnetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpGetSubnetInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetSubnetInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpGetSubnetInfoV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetSubnetInfoVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpGetSubnetInfoVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetSuperScopeInfoV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetSuperScopeInfoV4(serveripaddress: super::super::Foundation::PWSTR, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32;
        }
        ::core::mem::transmute(DhcpGetSuperScopeInfoV4(serveripaddress.into_param().abi(), ::core::mem::transmute(superscopetable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpGetThreadOptions(::core::mem::transmute(pflags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, majorversion: *mut u32, minorversion: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpGetVersion(serveripaddress: super::super::Foundation::PWSTR, majorversion: *mut u32, minorversion: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpGetVersion(serveripaddress.into_param().abi(), ::core::mem::transmute(majorversion), ::core::mem::transmute(minorversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyCondition<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: Param5, operator: DHCP_POL_COMPARATOR, value: *const u8, valuelength: u32, conditionindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprAddV4PolicyCondition(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: super::super::Foundation::PWSTR, operator: DHCP_POL_COMPARATOR, value: *const u8, valuelength: u32, conditionindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpHlprAddV4PolicyCondition(::core::mem::transmute(policy), ::core::mem::transmute(parentexpr), ::core::mem::transmute(r#type), ::core::mem::transmute(optionid), ::core::mem::transmute(suboptionid), vendorname.into_param().abi(), ::core::mem::transmute(operator), ::core::mem::transmute(value), ::core::mem::transmute(valuelength), ::core::mem::transmute(conditionindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpHlprAddV4PolicyExpr(::core::mem::transmute(policy), ::core::mem::transmute(parentexpr), ::core::mem::transmute(operator), ::core::mem::transmute(exprindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32;
        }
        ::core::mem::transmute(DhcpHlprAddV4PolicyRange(::core::mem::transmute(policy), ::core::mem::transmute(range)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprCreateV4Policy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(policyname: Param0, fglobalpolicy: Param1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: Param5, enabled: Param6, policy: *mut *mut DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprCreateV4Policy(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpHlprCreateV4Policy(policyname.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnet), ::core::mem::transmute(processingorder), ::core::mem::transmute(rootoperator), description.into_param().abi(), enabled.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprCreateV4PolicyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(policyname: Param0, fglobalpolicy: Param1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: Param5, enabled: Param6, policy: *mut *mut DHCP_POLICY_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprCreateV4PolicyEx(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY_EX) -> u32;
        }
        ::core::mem::transmute(DhcpHlprCreateV4PolicyEx(policyname.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnet), ::core::mem::transmute(processingorder), ::core::mem::transmute(rootoperator), description.into_param().abi(), enabled.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY;
        }
        ::core::mem::transmute(DhcpHlprFindV4DhcpProperty(::core::mem::transmute(propertyarray), ::core::mem::transmute(id), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY);
        }
        DhcpHlprFreeV4DhcpProperty(::core::mem::transmute(property))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY);
        }
        DhcpHlprFreeV4DhcpPropertyArray(::core::mem::transmute(propertyarray))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY);
        }
        DhcpHlprFreeV4Policy(::core::mem::transmute(policy))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY);
        }
        DhcpHlprFreeV4PolicyArray(::core::mem::transmute(policyarray))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX);
        }
        DhcpHlprFreeV4PolicyEx(::core::mem::transmute(policyex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY);
        }
        DhcpHlprFreeV4PolicyExArray(::core::mem::transmute(policyexarray))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DhcpHlprIsV4PolicySingleUC(::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpHlprIsV4PolicyValid(::core::mem::transmute(ppolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DhcpHlprIsV4PolicyWellFormed(::core::mem::transmute(ppolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32;
        }
        ::core::mem::transmute(DhcpHlprModifyV4PolicyExpr(::core::mem::transmute(policy), ::core::mem::transmute(operator)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpHlprResetV4PolicyExpr(::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpModifyClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpModifyClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpModifyClass(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(classinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpModifyClassV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpModifyClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpModifyClassV6(serveripaddress.into_param().abi(), ::core::mem::transmute(reservedmustbezero), ::core::mem::transmute(classinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRegisterParamChange<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, DHCPCAPI_PARAMS_ARRAY>>(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: Param2, classid: *mut DHCPCAPI_CLASSID, params: Param4, handle: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpRegisterParamChange(::core::mem::transmute(flags), ::core::mem::transmute(reserved), adaptername.into_param().abi(), ::core::mem::transmute(classid), params.into_param().abi(), ::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpRemoveDNSRegistrations() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveDNSRegistrations() -> u32;
        }
        ::core::mem::transmute(DhcpRemoveDNSRegistrations())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOption(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOptionV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOptionV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOptionV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOptionV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(scopeinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOptionValueV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOptionValueV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveOptionValueV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveOptionValueV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveSubnetElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveSubnetElement(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(removeelementinfo), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveSubnetElementV4(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(removeelementinfo), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveSubnetElementV5(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(removeelementinfo), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRemoveSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32;
        }
        ::core::mem::transmute(DhcpRemoveSubnetElementV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(removeelementinfo), ::core::mem::transmute(forceflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRequestParams<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, DHCPCAPI_PARAMS_ARRAY>, Param5: ::windows::core::IntoParam<'a, DHCPCAPI_PARAMS_ARRAY>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: Param2, classid: *mut DHCPCAPI_CLASSID, sendparams: Param4, recdparams: Param5, buffer: *mut u8, psize: *mut u32, requestidstr: Param8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpRequestParams(::core::mem::transmute(flags), ::core::mem::transmute(reserved), adaptername.into_param().abi(), ::core::mem::transmute(classid), sendparams.into_param().abi(), recdparams.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(psize), requestidstr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpRpcFreeMemory(bufferpointer: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpRpcFreeMemory(bufferpointer: *mut ::core::ffi::c_void);
        }
        DhcpRpcFreeMemory(::core::mem::transmute(bufferpointer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpScanDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpScanDatabase(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32;
        }
        ::core::mem::transmute(DhcpScanDatabase(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(fixflag), ::core::mem::transmute(scanlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ);
        }
        DhcpServerAuditlogParamsFree(::core::mem::transmute(configinfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerBackupDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, path: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerBackupDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpServerBackupDatabase(serveripaddress.into_param().abi(), path.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerGetConfig(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpServerGetConfig(serveripaddress.into_param().abi(), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerGetConfigV4(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
        }
        ::core::mem::transmute(DhcpServerGetConfigV4(serveripaddress.into_param().abi(), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerGetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpServerGetConfigV6(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerGetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpServerGetConfigVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerQueryAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddr: Param0, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerQueryAttribute(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32;
        }
        ::core::mem::transmute(DhcpServerQueryAttribute(serveripaddr.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(dhcpattribid), ::core::mem::transmute(pdhcpattrib)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerQueryAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddr: Param0, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerQueryAttributes(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpServerQueryAttributes(serveripaddr.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(dwattribcount), ::core::mem::transmute(pdhcpattribs), ::core::mem::transmute(pdhcpattribarr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerQueryDnsRegCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, unamesize: u32, uname: super::super::Foundation::PWSTR, domainsize: u32, domain: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerQueryDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, unamesize: u32, uname: super::super::Foundation::PWSTR, domainsize: u32, domain: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpServerQueryDnsRegCredentials(serveripaddress.into_param().abi(), ::core::mem::transmute(unamesize), ::core::mem::transmute(uname), ::core::mem::transmute(domainsize), ::core::mem::transmute(domain)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerRedoAuthorization<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddr: Param0, dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerRedoAuthorization(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(DhcpServerRedoAuthorization(serveripaddr.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerRestoreDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, path: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerRestoreDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpServerRestoreDatabase(serveripaddress.into_param().abi(), path.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetConfig(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetConfig(serveripaddress.into_param().abi(), ::core::mem::transmute(fieldstoset), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetConfigV4(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetConfigV4(serveripaddress.into_param().abi(), ::core::mem::transmute(fieldstoset), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetConfigV6(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(fieldstoset), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetConfigVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(fieldstoset), ::core::mem::transmute(configinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, uname: Param1, domain: Param2, passwd: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetDnsRegCredentials(serveripaddress.into_param().abi(), uname.into_param().abi(), domain.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentialsV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, uname: Param1, domain: Param2, passwd: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpServerSetDnsRegCredentialsV5(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpServerSetDnsRegCredentialsV5(serveripaddress.into_param().abi(), uname.into_param().abi(), domain.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpSetClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetClientInfoV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
        }
        ::core::mem::transmute(DhcpSetClientInfoV4(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetClientInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpSetClientInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetClientInfoVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpSetClientInfoVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetFilterV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpSetFilterV4(serveripaddress.into_param().abi(), ::core::mem::transmute(globalfilterinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionInfoV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionInfoV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, optioninfo: *mut DHCP_OPTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(optioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(optionid), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionValueV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionValueV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionValueV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, classname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionValueV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionValues(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalues)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetOptionValuesV5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, classname: Param2, vendorname: Param3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpSetOptionValuesV5(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), classname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalues)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpSetServerBindingInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(bindelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpSetServerBindingInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(bindelementinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSubnetDelayOffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32;
        }
        ::core::mem::transmute(DhcpSetSubnetDelayOffer(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(timedelayinmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSubnetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpSetSubnetInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSubnetInfoV6<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, subnetaddress: Param1, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpSetSubnetInfoV6(serveripaddress.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSubnetInfoVQ<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
        }
        ::core::mem::transmute(DhcpSetSubnetInfoVQ(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(subnetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSuperScopeV4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, subnetaddress: u32, superscopename: Param2, changeexisting: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, superscopename: super::super::Foundation::PWSTR, changeexisting: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DhcpSetSuperScopeV4(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), superscopename.into_param().abi(), changeexisting.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn DhcpSetThreadOptions(flags: u32, reserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpSetThreadOptions(flags: u32, reserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DhcpSetThreadOptions(::core::mem::transmute(flags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpUndoRequestParams<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: Param2, requestidstr: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpUndoRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, requestidstr: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpUndoRequestParams(::core::mem::transmute(flags), ::core::mem::transmute(reserved), adaptername.into_param().abi(), requestidstr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4AddPolicyRange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, policyname: Param2, range: *const DHCP_IP_RANGE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4AddPolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
        }
        ::core::mem::transmute(DhcpV4AddPolicyRange(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(range)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreateClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32;
        }
        ::core::mem::transmute(DhcpV4CreateClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreateClientInfoEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4CreateClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32;
        }
        ::core::mem::transmute(DhcpV4CreateClientInfoEx(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreatePolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, ppolicy: *const DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4CreatePolicy(serveripaddress: super::super::Foundation::PWSTR, ppolicy: *const DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpV4CreatePolicy(serveripaddress.into_param().abi(), ::core::mem::transmute(ppolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreatePolicyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, policyex: *const DHCP_POLICY_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4CreatePolicyEx(serveripaddress: super::super::Foundation::PWSTR, policyex: *const DHCP_POLICY_EX) -> u32;
        }
        ::core::mem::transmute(DhcpV4CreatePolicyEx(serveripaddress.into_param().abi(), ::core::mem::transmute(policyex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4DeletePolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fglobalpolicy: Param1, subnetaddress: u32, policyname: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4DeletePolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpV4DeletePolicy(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumPolicies<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: Param3, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4EnumPolicies(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4EnumPolicies(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enuminfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumPoliciesEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: Param3, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4EnumPoliciesEx(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4EnumPoliciesEx(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), globalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enuminfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumSubnetClients<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4EnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4EnumSubnetClients(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumSubnetClientsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4EnumSubnetClientsEx(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4EnumSubnetClientsEx(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(clientinfo), ::core::mem::transmute(clientsread), ::core::mem::transmute(clientstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumSubnetReservations<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4EnumSubnetReservations(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4EnumSubnetReservations(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(enumelementinfo), ::core::mem::transmute(elementsread), ::core::mem::transmute(elementstotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverAddScopeToRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverAddScopeToRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverAddScopeToRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverCreateRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverCreateRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverCreateRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverDeleteRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, prelationshipname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverDeleteRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverDeleteRelationship(serveripaddress.into_param().abi(), prelationshipname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverDeleteScopeFromRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverEnumRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverEnumRelationship(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverEnumRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(resumehandle), ::core::mem::transmute(preferredmaximum), ::core::mem::transmute(prelationship), ::core::mem::transmute(relationshipread), ::core::mem::transmute(relationshiptotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetAddressStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, pstatus: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetAddressStatus(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, pstatus: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetAddressStatus(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(pstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, prelationshipname: Param1, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetRelationship(serveripaddress.into_param().abi(), prelationshipname.into_param().abi(), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetScopeRelationship(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetScopeRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeid), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeStatistics<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetScopeStatistics(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetScopeStatistics(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeid), ::core::mem::transmute(pstats)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetSystemTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverGetSystemTime(serveripaddress: super::super::Foundation::PWSTR, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverGetSystemTime(serveripaddress.into_param().abi(), ::core::mem::transmute(ptime), ::core::mem::transmute(pmaxalloweddeltatime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverSetRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverSetRelationship(serveripaddress: super::super::Foundation::PWSTR, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverSetRelationship(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(prelationship)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverTriggerAddrAllocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, pfailrelname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress: super::super::Foundation::PWSTR, pfailrelname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DhcpV4FailoverTriggerAddrAllocation(serveripaddress.into_param().abi(), pfailrelname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetAllOptionValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetAllOptionValues(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetClientInfoEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetClientInfoEx(serveripaddress.into_param().abi(), ::core::mem::transmute(searchinfo), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetFreeIPAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetFreeIPAddress(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeid), ::core::mem::transmute(startip), ::core::mem::transmute(endip), ::core::mem::transmute(numfreeaddrreq), ::core::mem::transmute(ipaddrlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, policyname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), policyname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fglobalpolicy: Param1, subnetaddress: u32, policyname: Param3, policy: *mut *mut DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetPolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetPolicy(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetPolicyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, globalpolicy: Param1, subnetaddress: u32, policyname: Param3, policy: *mut *mut DHCP_POLICY_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4GetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY_EX) -> u32;
        }
        ::core::mem::transmute(DhcpV4GetPolicyEx(serveripaddress.into_param().abi(), globalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4QueryPolicyEnforcement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, fglobalpolicy: Param1, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4QueryPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DhcpV4QueryPolicyEnforcement(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), ::core::mem::transmute(enabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4RemoveOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, policyname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4RemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
        }
        ::core::mem::transmute(DhcpV4RemoveOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), policyname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4RemovePolicyRange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, subnetaddress: u32, policyname: Param2, range: *const DHCP_IP_RANGE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4RemovePolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
        }
        ::core::mem::transmute(DhcpV4RemovePolicyRange(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(range)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetOptionValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, optionid: u32, policyname: Param3, vendorname: Param4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4SetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
        }
        ::core::mem::transmute(DhcpV4SetOptionValue(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(optionid), policyname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetOptionValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, flags: u32, policyname: Param2, vendorname: Param3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4SetOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpV4SetOptionValues(serveripaddress.into_param().abi(), ::core::mem::transmute(flags), policyname.into_param().abi(), vendorname.into_param().abi(), ::core::mem::transmute(scopeinfo), ::core::mem::transmute(optionvalues)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fieldsmodified: u32, fglobalpolicy: Param2, subnetaddress: u32, policyname: Param4, policy: *const DHCP_POLICY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4SetPolicy(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY) -> u32;
        }
        ::core::mem::transmute(DhcpV4SetPolicy(serveripaddress.into_param().abi(), ::core::mem::transmute(fieldsmodified), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicyEnforcement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(serveripaddress: Param0, fglobalpolicy: Param1, subnetaddress: u32, enable: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4SetPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enable: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DhcpV4SetPolicyEnforcement(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), enable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, fieldsmodified: u32, globalpolicy: Param2, subnetaddress: u32, policyname: Param4, policy: *const DHCP_POLICY_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV4SetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY_EX) -> u32;
        }
        ::core::mem::transmute(DhcpV4SetPolicyEx(serveripaddress.into_param().abi(), ::core::mem::transmute(fieldsmodified), globalpolicy.into_param().abi(), ::core::mem::transmute(subnetaddress), policyname.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6CreateClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV6CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
        }
        ::core::mem::transmute(DhcpV6CreateClientInfo(serveripaddress.into_param().abi(), ::core::mem::transmute(clientinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6GetFreeIPAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>, Param2: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>, Param3: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, scopeid: Param1, startip: Param2, endip: Param3, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV6GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32;
        }
        ::core::mem::transmute(DhcpV6GetFreeIPAddress(serveripaddress.into_param().abi(), scopeid.into_param().abi(), startip.into_param().abi(), endip.into_param().abi(), ::core::mem::transmute(numfreeaddrreq), ::core::mem::transmute(ipaddrlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6GetStatelessStatistics<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(serveripaddress: Param0, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV6GetStatelessStatistics(serveripaddress: super::super::Foundation::PWSTR, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32;
        }
        ::core::mem::transmute(DhcpV6GetStatelessStatistics(serveripaddress.into_param().abi(), ::core::mem::transmute(statelessstats)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6GetStatelessStoreParams<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, fserverlevel: Param1, subnetaddress: Param2, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV6GetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32;
        }
        ::core::mem::transmute(DhcpV6GetStatelessStoreParams(serveripaddress.into_param().abi(), fserverlevel.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6SetStatelessStoreParams<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, DHCP_IPV6_ADDRESS>>(serveripaddress: Param0, fserverlevel: Param1, subnetaddress: Param2, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DhcpV6SetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32;
        }
        ::core::mem::transmute(DhcpV6SetStatelessStoreParams(serveripaddress.into_param().abi(), fserverlevel.into_param().abi(), subnetaddress.into_param().abi(), ::core::mem::transmute(fieldmodified), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn Dhcpv6CApiCleanup() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6CApiCleanup();
        }
        Dhcpv6CApiCleanup()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
#[inline]
pub unsafe fn Dhcpv6CApiInitialize(version: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6CApiInitialize(version: *mut u32);
        }
        Dhcpv6CApiInitialize(::core::mem::transmute(version))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Dhcpv6ReleasePrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(adaptername: Param0, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6ReleasePrefix(adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32;
        }
        ::core::mem::transmute(Dhcpv6ReleasePrefix(adaptername.into_param().abi(), ::core::mem::transmute(classid), ::core::mem::transmute(leaseinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Dhcpv6RenewPrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(adaptername: Param0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6RenewPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32;
        }
        ::core::mem::transmute(Dhcpv6RenewPrefix(adaptername.into_param().abi(), ::core::mem::transmute(pclassid), ::core::mem::transmute(prefixleaseinfo), ::core::mem::transmute(pdwtimetowait), ::core::mem::transmute(bvalidateprefix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Dhcpv6RequestParams<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, DHCPV6CAPI_PARAMS_ARRAY>>(forcenewinform: Param0, reserved: *mut ::core::ffi::c_void, adaptername: Param2, classid: *mut DHCPV6CAPI_CLASSID, recdparams: Param4, buffer: *mut u8, psize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6RequestParams(forcenewinform: super::super::Foundation::BOOL, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32;
        }
        ::core::mem::transmute(Dhcpv6RequestParams(forcenewinform.into_param().abi(), ::core::mem::transmute(reserved), adaptername.into_param().abi(), ::core::mem::transmute(classid), recdparams.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(psize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Dhcpv6RequestPrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(adaptername: Param0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Dhcpv6RequestPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32;
        }
        ::core::mem::transmute(Dhcpv6RequestPrefix(adaptername.into_param().abi(), ::core::mem::transmute(pclassid), ::core::mem::transmute(prefixleaseinfo), ::core::mem::transmute(pdwtimetowait)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: u32 = 20078u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_CLASS_EXISTS: u32 = 20077u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: u32 = 20074u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_NO_DHCP_ROOT: u32 = 20071u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_NO_DS_AVAILABLE: u32 = 20070u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: u32 = 20075u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: u32 = 20076u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: u32 = 20087u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: u32 = 20088u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_RESERVATION_CONFLICT: u32 = 20086u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: u32 = 20085u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: u32 = 20081u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: u32 = 20079u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: u32 = 20080u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SUBNET_EXISTS: u32 = 20082u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: u32 = 20083u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_SUBNET_NOT_PRESENT: u32 = 20084u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_TOO_MANY_ERRORS: u32 = 20073u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DDS_UNEXPECTED_ERROR: u32 = 20072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: u32 = 20011u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: u32 = 20051u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: u32 = 20048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: u32 = 20045u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_CLASS_NOT_FOUND: u32 = 20044u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_CLIENT_EXISTS: u32 = 20014u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_DATABASE_INIT_FAILED: u32 = 20001u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: u32 = 20047u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: u32 = 20089u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: u32 = 20007u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_EXEMPTION_EXISTS: u32 = 20055u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: u32 = 20056u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: u32 = 20127u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: u32 = 20131u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: u32 = 20134u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: u32 = 20129u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: u32 = 20130u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: u32 = 20128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_NOT_SUPPORTED: u32 = 20118u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: u32 = 20132u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: u32 = 20115u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: u32 = 20114u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: u32 = 20125u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: u32 = 20117u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: u32 = 20113u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: u32 = 20116u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: u32 = 20133u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: u32 = 20120u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: u32 = 20119u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: u32 = 20101u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_DELAY: u32 = 20092u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: u32 = 20016u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: u32 = 20015u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: u32 = 20057u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: u32 = 20109u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: u32 = 20110u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_RANGE: u32 = 20023u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: u32 = 20091u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: u32 = 20049u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_IPRANGE_EXITS: u32 = 20021u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: u32 = 20032u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_JET97_CONV_REQUIRED: u32 = 20036u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_JET_CONV_REQUIRED: u32 = 20027u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_JET_ERROR: u32 = 20013u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: u32 = 20095u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: u32 = 20093u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: u32 = 20094u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: u32 = 20033u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_MSCOPE_EXISTS: u32 = 20053u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: u32 = 20138u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_NETWORK_CHANGED: u32 = 20050u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_NETWORK_INIT_FAILED: u32 = 20003u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: u32 = 20018u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: u32 = 20121u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_OPTION_EXITS: u32 = 20009u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_OPTION_NOT_PRESENT: u32 = 20010u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: u32 = 20103u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: u32 = 20104u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: u32 = 20137u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_EXISTS: u32 = 20105u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: u32 = 20136u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: u32 = 20135u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_NOT_FOUND: u32 = 20111u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_RANGE_BAD: u32 = 20107u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: u32 = 20106u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: u32 = 20006u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RANGE_EXTENDED: u32 = 20024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RANGE_FULL: u32 = 20012u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: u32 = 20108u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RANGE_TOO_SMALL: u32 = 20020u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: u32 = 20126u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: u32 = 20000u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RESERVEDIP_EXITS: u32 = 20022u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RESERVED_CLIENT: u32 = 20019u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: u32 = 20041u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: u32 = 20040u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_INIT_FAILED: u32 = 20037u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: u32 = 20039u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: u32 = 20042u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: u32 = 20038u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: u32 = 20043u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_RPC_INIT_FAILED: u32 = 20002u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: u32 = 20046u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: u32 = 20124u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: u32 = 20122u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SERVER_NOT_RUNNING: u32 = 20123u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SERVICE_PAUSED: u32 = 20017u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SUBNET_EXISTS: u32 = 20052u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SUBNET_EXITS: u32 = 20004u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: u32 = 20005u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: u32 = 20030u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: u32 = 20102u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: u32 = 20034u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_EXTEND_TOO_SMALL: u32 = 20025u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_LAST_DHCP_SERVER_ERROR: u32 = 20139u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: u32 = 20054u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: u32 = 20112u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: u32 = 20028u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: u32 = 20029u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const FILTER_STATUS_NONE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type FSM_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const NO_STATE: FSM_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const INIT: FSM_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const STARTUP: FSM_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const NORMAL: FSM_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const COMMUNICATION_INT: FSM_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PARTNER_DOWN: FSM_STATE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const POTENTIAL_CONFLICT: FSM_STATE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const CONFLICT_DONE: FSM_STATE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const RESOLUTION_INT: FSM_STATE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const RECOVER: FSM_STATE = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const RECOVER_WAIT: FSM_STATE = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const RECOVER_DONE: FSM_STATE = 11i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PAUSED: FSM_STATE = 12i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const SHUTDOWN: FSM_STATE = 13i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const HWTYPE_ETHERNET_10MB: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_CONTROL = ::core::option::Option<unsafe extern "system" fn(dwcontrolcode: u32, lpreserved: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_DELETE_CLIENT = ::core::option::Option<unsafe extern "system" fn(ipaddress: u32, hwaddress: *mut u8, hwaddresslength: u32, reserved: u32, clienttype: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_DROP_SEND = ::core::option::Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, controlcode: u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_ENTRY_POINT_FUNC = ::core::option::Option<unsafe extern "system" fn(chaindlls: super::super::Foundation::PWSTR, calloutversion: u32, callouttbl: *mut DHCP_CALLOUT_TABLE) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_GIVE_ADDRESS = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, addrtype: u32, leasetime: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_HANDLE_OPTIONS = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void, serveroptions: *mut DHCP_SERVER_OPTIONS) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_NEWPKT = ::core::option::Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut *mut ::core::ffi::c_void, processit: *mut i32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type LPDHCP_PROB = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const MAC_ADDRESS_LENGTH: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const MAX_PATTERN_LENGTH: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const MCLT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const MODE: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ALL_SUBNETS_MTU: u32 = 27u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ARP_CACHE_TIMEOUT: u32 = 35u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_BE_A_MASK_SUPPLIER: u32 = 30u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_BE_A_ROUTER: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_BOOTFILE_NAME: u32 = 67u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_BOOT_FILE_SIZE: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_BROADCAST_ADDRESS: u32 = 28u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_CLIENT_CLASS_INFO: u32 = 60u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_CLIENT_ID: u32 = 61u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_COOKIE_SERVERS: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_DEFAULT_TTL: u32 = 23u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_DOMAIN_NAME: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_DOMAIN_NAME_SERVERS: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_END: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ETHERNET_ENCAPSULATION: u32 = 36u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_EXTENSIONS_PATH: u32 = 18u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_HOST_NAME: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_IEN116_NAME_SERVERS: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_IMPRESS_SERVERS: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_KEEP_ALIVE_DATA_SIZE: u32 = 39u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_KEEP_ALIVE_INTERVAL: u32 = 38u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_LEASE_TIME: u32 = 51u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_LOG_SERVERS: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_LPR_SERVERS: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MAX_REASSEMBLY_SIZE: u32 = 22u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MERIT_DUMP_FILE: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MESSAGE: u32 = 56u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MESSAGE_LENGTH: u32 = 57u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MESSAGE_TYPE: u32 = 53u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MSFT_IE_PROXY: u32 = 252u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_MTU: u32 = 26u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETBIOS_DATAGRAM_SERVER: u32 = 45u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETBIOS_NAME_SERVER: u32 = 44u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETBIOS_NODE_TYPE: u32 = 46u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETBIOS_SCOPE_OPTION: u32 = 47u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETWORK_INFO_SERVERS: u32 = 41u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETWORK_INFO_SERVICE_DOM: u32 = 40u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NETWORK_TIME_SERVERS: u32 = 42u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_NON_LOCAL_SOURCE_ROUTING: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_OK_TO_OVERLAY: u32 = 52u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PAD: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PARAMETER_REQUEST_LIST: u32 = 55u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PERFORM_MASK_DISCOVERY: u32 = 29u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PERFORM_ROUTER_DISCOVERY: u32 = 31u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PMTU_AGING_TIMEOUT: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_PMTU_PLATEAU_TABLE: u32 = 25u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_POLICY_FILTER_FOR_NLSR: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_REBIND_TIME: u32 = 59u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_RENEWAL_TIME: u32 = 58u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_REQUESTED_ADDRESS: u32 = 50u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_RLP_SERVERS: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ROOT_DISK: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ROUTER_ADDRESS: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_ROUTER_SOLICITATION_ADDR: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_SERVER_IDENTIFIER: u32 = 54u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_STATIC_ROUTES: u32 = 33u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_SUBNET_MASK: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_SWAP_SERVER: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_TFTP_SERVER_NAME: u32 = 66u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_TIME_OFFSET: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_TIME_SERVERS: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_TRAILERS: u32 = 34u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_TTL: u32 = 37u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_VENDOR_SPEC_INFO: u32 = 43u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_XWINDOW_DISPLAY_MANAGER: u32 = 49u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const OPTION_XWINDOW_FONT_SERVER: u32 = 48u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PERCENTAGE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PREVSTATE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const QUARANTINE_CONFIG_OPTION: u32 = 43222u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: u32 = 43221u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const QUARANTIN_OPTION_BASE: u32 = 43220u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type QuarantineStatus = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const NOQUARANTINE: QuarantineStatus = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const RESTRICTEDACCESS: QuarantineStatus = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DROPPACKET: QuarantineStatus = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const PROBATION: QuarantineStatus = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const EXEMPT: QuarantineStatus = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const DEFAULTQUARSETTING: QuarantineStatus = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const NOQUARINFO: QuarantineStatus = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const SAFEPERIOD: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct SCOPE_MIB_INFO {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO {}
impl ::core::clone::Clone for SCOPE_MIB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCOPE_MIB_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCOPE_MIB_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO {}
impl ::core::default::Default for SCOPE_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_V5 {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCOPE_MIB_INFO_V5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCOPE_MIB_INFO_V5>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V5 {}
impl ::core::default::Default for SCOPE_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: u64,
    pub NumAddressesFree: u64,
    pub NumPendingAdvertises: u64,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_V6 {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCOPE_MIB_INFO_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCOPE_MIB_INFO_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V6 {}
impl ::core::default::Default for SCOPE_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub struct SCOPE_MIB_INFO_VQ {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_VQ {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCOPE_MIB_INFO_VQ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCOPE_MIB_INFO_VQ>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_VQ {}
impl ::core::default::Default for SCOPE_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const SHAREDSECRET: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_APIProtocolSupport: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_AuditLogState: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_BackupInterval: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_BackupPath: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_BootFileTable: u32 = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_DatabaseCleanupInterval: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_DatabaseLoggingFlag: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_DatabaseName: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_DatabasePath: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_DebugFlag: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_PingRetries: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_PreferredLifetime: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_PreferredLifetimeIATA: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_QuarantineDefFail: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_QuarantineON: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_RapidCommitFlag: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_RestoreFlag: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_T1: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_T2: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_UnicastFlag: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_ValidLifetime: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const Set_ValidLifetimeIATA: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub type StatusCode = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const STATUS_NO_ERROR: StatusCode = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const STATUS_UNSPECIFIED_FAILURE: StatusCode = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const STATUS_NO_BINDING: StatusCode = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const STATUS_NOPREFIX_AVAIL: StatusCode = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_BIT_DELETED: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_STATE_DOOM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_Dhcp'*"]
pub const WARNING_EXTENDED_LESS: i32 = 20026i32;
