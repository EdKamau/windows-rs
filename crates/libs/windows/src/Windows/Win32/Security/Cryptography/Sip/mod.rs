#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPAddProvider(::core::mem::transmute(psnewprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPCreateIndirectData(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(pcbindirectdata), ::core::mem::transmute(pindirectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPGetCaps(::core::mem::transmute(psubjinfo), ::core::mem::transmute(pcaps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPGetSealedDigest(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(psig), ::core::mem::transmute(dwsig), ::core::mem::transmute(pbdigest), ::core::mem::transmute(pcbdigest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPGetSignedDataMsg(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(pdwencodingtype), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcbsigneddatamsg), ::core::mem::transmute(pbsigneddatamsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPLoad(pgsubject: *const ::windows::core::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPLoad(pgsubject: *const ::windows::core::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPLoad(::core::mem::transmute(pgsubject), ::core::mem::transmute(dwflags), ::core::mem::transmute(psipdispatch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPPutSignedDataMsg(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(dwencodingtype), ::core::mem::transmute(pdwindex), ::core::mem::transmute(cbsigneddatamsg), ::core::mem::transmute(pbsigneddatamsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptSIPRemoveProvider(pgprov: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveProvider(pgprov: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPRemoveProvider(::core::mem::transmute(pgprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPRemoveSignedDataMsg(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(dwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(filename: Param0, hfilein: Param1, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuid(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPRetrieveSubjectGuid(filename.into_param().abi(), hfilein.into_param().abi(), ::core::mem::transmute(pgsubject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuidForCatalogFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(filename: Param0, hfilein: Param1, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPRetrieveSubjectGuidForCatalogFile(filename.into_param().abi(), hfilein.into_param().abi(), ::core::mem::transmute(pgsubject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[inline]
pub unsafe fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptSIPVerifyIndirectData(::core::mem::transmute(psubjectinfo), ::core::mem::transmute(pindirectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_ADDINFO_BLOB: u32 = 3u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_ADDINFO_CATMEMBER: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_ADDINFO_FLAT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_ADDINFO_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_ADDINFO_NONMSSIP: u32 = 500u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl ::core::marker::Copy for MS_ADDINFO_BLOB {}
impl ::core::clone::Clone for MS_ADDINFO_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MS_ADDINFO_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MS_ADDINFO_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MS_ADDINFO_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for MS_ADDINFO_BLOB {}
impl ::core::default::Default for MS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut super::Catalog::CRYPTCATSTORE,
    pub pMember: *mut super::Catalog::CRYPTCATMEMBER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for MS_ADDINFO_CATALOGMEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::core::Abi for MS_ADDINFO_CATALOGMEMBER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::PartialEq for MS_ADDINFO_CATALOGMEMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MS_ADDINFO_CATALOGMEMBER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::Eq for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::default::Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MS_ADDINFO_FLAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MS_ADDINFO_FLAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MS_ADDINFO_FLAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MS_ADDINFO_FLAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MS_ADDINFO_FLAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MS_ADDINFO_FLAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut ::windows::core::GUID,
    pub pwszDLLFileName: super::super::super::Foundation::PWSTR,
    pub pwszMagicNumber: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionName: super::super::super::Foundation::PWSTR,
    pub pwszGetFuncName: super::super::super::Foundation::PWSTR,
    pub pwszPutFuncName: super::super::super::Foundation::PWSTR,
    pub pwszCreateFuncName: super::super::super::Foundation::PWSTR,
    pub pwszVerifyFuncName: super::super::super::Foundation::PWSTR,
    pub pwszRemoveFuncName: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionNameFmt2: super::super::super::Foundation::PWSTR,
    pub pwszGetCapFuncName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_ADD_NEWPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SIP_ADD_NEWPROVIDER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SIP_ADD_NEWPROVIDER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_ADD_NEWPROVIDER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SIP_CAP_SET_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SIP_CAP_SET_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_CAP_SET_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SIP_CAP_SET_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SIP_CAP_SET_V3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SIP_CAP_SET_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_CAP_SET_V3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V3_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SIP_CAP_SET_V3_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SIP_CAP_SET_V3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_CAP_SET_V3_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SIP_CAP_SET_V3_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::super::super::Foundation::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_DISPATCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_DISPATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::core::Abi for SIP_DISPATCH_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::PartialEq for SIP_DISPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_DISPATCH_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::Eq for SIP_DISPATCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::default::Default for SIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_INDIRECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_INDIRECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SIP_INDIRECT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SIP_INDIRECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_INDIRECT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SIP_INDIRECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SIP_INDIRECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut ::windows::core::GUID,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub pwsFileName: super::super::super::Foundation::PWSTR,
    pub pwsDisplayName: super::super::super::Foundation::PWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_SUBJECTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_SUBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::core::Abi for SIP_SUBJECTINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::PartialEq for SIP_SUBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_SUBJECTINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::Eq for SIP_SUBJECTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::default::Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_SUBJECTINFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_SUBJECTINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::core::Abi for SIP_SUBJECTINFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::PartialEq for SIP_SUBJECTINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIP_SUBJECTINFO_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::cmp::Eq for SIP_SUBJECTINFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::default::Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip'*"]
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPCreateIndirectData = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetCaps = ::core::option::Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSealedDigest = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPPutSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPRemoveSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation', 'Win32_Security_Cryptography_Catalog'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPVerifyIndirectData = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupported = ::core::option::Option<unsafe extern "system" fn(hfile: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Cryptography_Sip', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupportedName = ::core::option::Option<unsafe extern "system" fn(pwszfilename: super::super::super::Foundation::PWSTR, pgsubject: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOL>;
