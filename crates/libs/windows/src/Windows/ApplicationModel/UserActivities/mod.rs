#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivity {
    type Vtable = IUserActivityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivity2 {
    type Vtable = IUserActivity2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dc40c62_08c4_47ac_aa9c_2bb2221c55fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivity3 {
    type Vtable = IUserActivity3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7697744_e1a2_5147_8e06_55f1eeef271c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttribution(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityAttribution {
    type Vtable = IUserActivityAttributionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a5c8b5_86dd_4aec_a491_6a4faea5d22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttributionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttributionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityAttributionFactory {
    type Vtable = IUserActivityAttributionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62bd252_c566_4f42_9974_916c4d76377e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttributionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannel {
    type Vtable = IUserActivityChannelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannel2 {
    type Vtable = IUserActivityChannel2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1698e35b_eb7e_4ea0_bf17_a459e8be706c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxuniqueactivities: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics {
    type Vtable = IUserActivityChannelStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8c005ab_198d_4d80_abb2_c9775ec4a729);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics2 {
    type Vtable = IUserActivityChannelStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e87de30_aa4f_4624_9ad0_d40f3ba0317c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics3 {
    type Vtable = IUserActivityChannelStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53bc4ddb_bbdf_5984_802a_5305874e205c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct IUserActivityContentInfo(::windows::core::IUnknown);
impl IUserActivityContentInfo {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IUserActivityContentInfo> for ::windows::core::IInspectable {
    fn from(value: IUserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityContentInfo> for ::windows::core::IInspectable {
    fn from(value: &IUserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IUserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserActivityContentInfo> for ::windows::core::IUnknown {
    fn from(value: IUserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityContentInfo> for ::windows::core::IUnknown {
    fn from(value: &IUserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityContentInfo {}
unsafe impl ::windows::core::RuntimeType for IUserActivityContentInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b399e5ad-137f-409d-822d-e1af27ce08dc}");
}
unsafe impl ::windows::core::Interface for IUserActivityContentInfo {
    type Vtable = IUserActivityContentInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb399e5ad_137f_409d_822d_e1af27ce08dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityContentInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityContentInfoStatics {
    type Vtable = IUserActivityContentInfoStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9988c34b_0386_4bc9_968a_8200b004144f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfoStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityFactory {
    type Vtable = IUserActivityFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c385758_361d_4a67_8a3b_34ca2978f9a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityRequest {
    type Vtable = IUserActivityRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0ef6355_cf35_4ff0_8833_50cb4b72e06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityRequestManager {
    type Vtable = IUserActivityRequestManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c30be4e_903d_48d6_82d4_4043ed57791b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityRequestManagerStatics {
    type Vtable = IUserActivityRequestManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0392df1_224a_432c_81e5_0c76b4c4cefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4cc7a4c_8229_4cfd_a3bc_c61d318575a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivitySession {
    type Vtable = IUserActivitySessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae434d78_24fa_44a3_ad48_6eda61aa1924);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySessionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySessionHistoryItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d59bd3_3e5d_49fd_98d7_6da97521e255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySessionHistoryItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityStatics {
    type Vtable = IUserActivityStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c8fd333_0e09_47f6_9ac7_95cf5c39367b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activities: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityVisualElements {
    type Vtable = IUserActivityVisualElementsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94757513_262f_49ef_bbbf_9b75d2e85250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElementsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Shell")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))] usize,
    #[cfg(feature = "UI_Shell")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityVisualElements2 {
    type Vtable = IUserActivityVisualElements2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaae7fc7_3eef_4359_825c_9d51b9220de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivity(::windows::core::IUnknown);
impl UserActivity {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn State(&self) -> ::windows::core::Result<UserActivityState> {
        let this = self;
        unsafe {
            let mut result__: UserActivityState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityState>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn VisualElements(&self) -> ::windows::core::Result<UserActivityVisualElements> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityVisualElements>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ActivationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ContentInfo(&self) -> ::windows::core::Result<IUserActivityContentInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IUserActivityContentInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetContentInfo<'a, Param0: ::windows::core::IntoParam<'a, IUserActivityContentInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn CreateSession(&self) -> ::windows::core::Result<UserActivitySession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivitySession>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserActivity2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn IsRoamable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUserActivity3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserActivity3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn CreateWithActivityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(activityid: Param0) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activityid.into_param().abi(), &mut result__).from_abi::<UserActivity>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn TryParseFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(json: Param0) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), json.into_param().abi(), &mut result__).from_abi::<UserActivity>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryParseFromJsonArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(json: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), json.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<UserActivity>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ToJsonArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<UserActivity>>>(activities: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), activities.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityFactory<R, F: FnOnce(&IUserActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivity, IUserActivityFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserActivityStatics<R, F: FnOnce(&IUserActivityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivity, IUserActivityStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivity {}
unsafe impl ::windows::core::RuntimeType for UserActivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivity;{fc103e9e-2cab-4d36-aea2-b4bb556cef0f})");
}
unsafe impl ::windows::core::Interface for UserActivity {
    type Vtable = IUserActivityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
}
impl ::windows::core::RuntimeName for UserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivity";
}
impl ::core::convert::From<UserActivity> for ::windows::core::IUnknown {
    fn from(value: UserActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivity> for ::windows::core::IUnknown {
    fn from(value: &UserActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivity> for ::windows::core::IInspectable {
    fn from(value: UserActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivity> for ::windows::core::IInspectable {
    fn from(value: &UserActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivity {}
unsafe impl ::core::marker::Sync for UserActivity {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityAttribution(::windows::core::IUnknown);
impl UserActivityAttribution {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityAttribution, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIconUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetAlternateText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn AddImageQuery(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetAddImageQuery(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(iconuri: Param0) -> ::windows::core::Result<UserActivityAttribution> {
        Self::IUserActivityAttributionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), iconuri.into_param().abi(), &mut result__).from_abi::<UserActivityAttribution>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityAttributionFactory<R, F: FnOnce(&IUserActivityAttributionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityAttribution, IUserActivityAttributionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityAttribution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityAttribution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityAttribution {}
unsafe impl ::windows::core::RuntimeType for UserActivityAttribution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityAttribution;{34a5c8b5-86dd-4aec-a491-6a4faea5d22e})");
}
unsafe impl ::windows::core::Interface for UserActivityAttribution {
    type Vtable = IUserActivityAttributionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a5c8b5_86dd_4aec_a491_6a4faea5d22e);
}
impl ::windows::core::RuntimeName for UserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityAttribution";
}
impl ::core::convert::From<UserActivityAttribution> for ::windows::core::IUnknown {
    fn from(value: UserActivityAttribution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityAttribution> for ::windows::core::IUnknown {
    fn from(value: &UserActivityAttribution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityAttribution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityAttribution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityAttribution> for ::windows::core::IInspectable {
    fn from(value: UserActivityAttribution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityAttribution> for ::windows::core::IInspectable {
    fn from(value: &UserActivityAttribution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityAttribution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityAttribution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityAttribution {}
unsafe impl ::core::marker::Sync for UserActivityAttribution {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityChannel(::windows::core::IUnknown);
impl UserActivityChannel {
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetOrCreateUserActivityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activityid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserActivity>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserActivity>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteActivityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activityid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAllActivitiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), maxuniqueactivities, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSessionHistoryItemsForUserActivityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, activityid: Param0, starttime: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activityid.into_param().abi(), starttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn GetDefault() -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityChannel>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn DisableAutoSessionCreation() -> ::windows::core::Result<()> {
        Self::IUserActivityChannelStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn TryGetForWebAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(account: Param0) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), account.into_param().abi(), &mut result__).from_abi::<UserActivityChannel>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'System'*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserActivityChannel>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics<R, F: FnOnce(&IUserActivityChannelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics2<R, F: FnOnce(&IUserActivityChannelStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics3<R, F: FnOnce(&IUserActivityChannelStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityChannel {}
unsafe impl ::windows::core::RuntimeType for UserActivityChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityChannel;{bac0f8b8-a0e4-483b-b948-9cbabd06070c})");
}
unsafe impl ::windows::core::Interface for UserActivityChannel {
    type Vtable = IUserActivityChannelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
}
impl ::windows::core::RuntimeName for UserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityChannel";
}
impl ::core::convert::From<UserActivityChannel> for ::windows::core::IUnknown {
    fn from(value: UserActivityChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityChannel> for ::windows::core::IUnknown {
    fn from(value: &UserActivityChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityChannel> for ::windows::core::IInspectable {
    fn from(value: UserActivityChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityChannel> for ::windows::core::IInspectable {
    fn from(value: &UserActivityChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityChannel {}
unsafe impl ::core::marker::Sync for UserActivityChannel {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityContentInfo(::windows::core::IUnknown);
impl UserActivityContentInfo {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn FromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<UserActivityContentInfo> {
        Self::IUserActivityContentInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<UserActivityContentInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityContentInfoStatics<R, F: FnOnce(&IUserActivityContentInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityContentInfo, IUserActivityContentInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityContentInfo {}
unsafe impl ::windows::core::RuntimeType for UserActivityContentInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityContentInfo;{b399e5ad-137f-409d-822d-e1af27ce08dc})");
}
unsafe impl ::windows::core::Interface for UserActivityContentInfo {
    type Vtable = IUserActivityContentInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb399e5ad_137f_409d_822d_e1af27ce08dc);
}
impl ::windows::core::RuntimeName for UserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityContentInfo";
}
impl ::core::convert::From<UserActivityContentInfo> for ::windows::core::IUnknown {
    fn from(value: UserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityContentInfo> for ::windows::core::IUnknown {
    fn from(value: &UserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityContentInfo> for ::windows::core::IInspectable {
    fn from(value: UserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityContentInfo> for ::windows::core::IInspectable {
    fn from(value: &UserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: UserActivityContentInfo) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivityContentInfo) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserActivityContentInfo> for UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IUserActivityContentInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserActivityContentInfo> for &UserActivityContentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IUserActivityContentInfo> {
        ::core::convert::TryInto::<IUserActivityContentInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserActivityContentInfo {}
unsafe impl ::core::marker::Sync for UserActivityContentInfo {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityRequest(::windows::core::IUnknown);
impl UserActivityRequest {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetUserActivity<'a, Param0: ::windows::core::IntoParam<'a, UserActivity>>(&self, activity: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activity.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserActivityRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequest {}
unsafe impl ::windows::core::RuntimeType for UserActivityRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequest;{a0ef6355-cf35-4ff0-8833-50cb4b72e06d})");
}
unsafe impl ::windows::core::Interface for UserActivityRequest {
    type Vtable = IUserActivityRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0ef6355_cf35_4ff0_8833_50cb4b72e06d);
}
impl ::windows::core::RuntimeName for UserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequest";
}
impl ::core::convert::From<UserActivityRequest> for ::windows::core::IUnknown {
    fn from(value: UserActivityRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequest> for ::windows::core::IUnknown {
    fn from(value: &UserActivityRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequest> for ::windows::core::IInspectable {
    fn from(value: UserActivityRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequest> for ::windows::core::IInspectable {
    fn from(value: &UserActivityRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityRequest {}
unsafe impl ::core::marker::Sync for UserActivityRequest {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityRequestManager(::windows::core::IUnknown);
impl UserActivityRequestManager {
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UserActivityRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserActivityRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<UserActivityRequestManager> {
        Self::IUserActivityRequestManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityRequestManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityRequestManagerStatics<R, F: FnOnce(&IUserActivityRequestManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserActivityRequestManager, IUserActivityRequestManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityRequestManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestManager {}
unsafe impl ::windows::core::RuntimeType for UserActivityRequestManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestManager;{0c30be4e-903d-48d6-82d4-4043ed57791b})");
}
unsafe impl ::windows::core::Interface for UserActivityRequestManager {
    type Vtable = IUserActivityRequestManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c30be4e_903d_48d6_82d4_4043ed57791b);
}
impl ::windows::core::RuntimeName for UserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestManager";
}
impl ::core::convert::From<UserActivityRequestManager> for ::windows::core::IUnknown {
    fn from(value: UserActivityRequestManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestManager> for ::windows::core::IUnknown {
    fn from(value: &UserActivityRequestManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityRequestManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityRequestManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequestManager> for ::windows::core::IInspectable {
    fn from(value: UserActivityRequestManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestManager> for ::windows::core::IInspectable {
    fn from(value: &UserActivityRequestManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityRequestManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityRequestManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityRequestedEventArgs(::windows::core::IUnknown);
impl UserActivityRequestedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn Request(&self) -> ::windows::core::Result<UserActivityRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivityRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestedEventArgs {}
unsafe impl ::windows::core::RuntimeType for UserActivityRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs;{a4cc7a4c-8229-4cfd-a3bc-c61d318575a4})");
}
unsafe impl ::windows::core::Interface for UserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4cc7a4c_8229_4cfd_a3bc_c61d318575a4);
}
impl ::windows::core::RuntimeName for UserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs";
}
impl ::core::convert::From<UserActivityRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserActivityRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserActivityRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserActivityRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserActivityRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityRequestedEventArgs {}
unsafe impl ::core::marker::Sync for UserActivityRequestedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivitySession(::windows::core::IUnknown);
impl UserActivitySession {
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySession {}
unsafe impl ::windows::core::RuntimeType for UserActivitySession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySession;{ae434d78-24fa-44a3-ad48-6eda61aa1924})");
}
unsafe impl ::windows::core::Interface for UserActivitySession {
    type Vtable = IUserActivitySessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae434d78_24fa_44a3_ad48_6eda61aa1924);
}
impl ::windows::core::RuntimeName for UserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySession";
}
impl ::core::convert::From<UserActivitySession> for ::windows::core::IUnknown {
    fn from(value: UserActivitySession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySession> for ::windows::core::IUnknown {
    fn from(value: &UserActivitySession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivitySession> for ::windows::core::IInspectable {
    fn from(value: UserActivitySession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySession> for ::windows::core::IInspectable {
    fn from(value: &UserActivitySession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UserActivitySession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: UserActivitySession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserActivitySession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivitySession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &UserActivitySession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserActivitySession {}
unsafe impl ::core::marker::Sync for UserActivitySession {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivitySessionHistoryItem(::windows::core::IUnknown);
impl UserActivitySessionHistoryItem {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn UserActivity(&self) -> ::windows::core::Result<UserActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivity>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySessionHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySessionHistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySessionHistoryItem {}
unsafe impl ::windows::core::RuntimeType for UserActivitySessionHistoryItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem;{e8d59bd3-3e5d-49fd-98d7-6da97521e255})");
}
unsafe impl ::windows::core::Interface for UserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d59bd3_3e5d_49fd_98d7_6da97521e255);
}
impl ::windows::core::RuntimeName for UserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem";
}
impl ::core::convert::From<UserActivitySessionHistoryItem> for ::windows::core::IUnknown {
    fn from(value: UserActivitySessionHistoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySessionHistoryItem> for ::windows::core::IUnknown {
    fn from(value: &UserActivitySessionHistoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivitySessionHistoryItem> for ::windows::core::IInspectable {
    fn from(value: UserActivitySessionHistoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySessionHistoryItem> for ::windows::core::IInspectable {
    fn from(value: &UserActivitySessionHistoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivitySessionHistoryItem {}
unsafe impl ::core::marker::Sync for UserActivitySessionHistoryItem {}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl ::core::marker::Copy for UserActivityState {}
impl ::core::clone::Clone for UserActivityState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UserActivityState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UserActivityState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityState {}
unsafe impl ::windows::core::RuntimeType for UserActivityState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserActivities.UserActivityState;i4)");
}
impl ::windows::core::DefaultType for UserActivityState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
#[repr(transparent)]
pub struct UserActivityVisualElements(::windows::core::IUnknown);
impl UserActivityVisualElements {
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetDisplayText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn Attribution(&self) -> ::windows::core::Result<UserActivityAttribution> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserActivityAttribution>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetAttribution<'a, Param0: ::windows::core::IntoParam<'a, UserActivityAttribution>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'UI_Shell'*"]
    #[cfg(feature = "UI_Shell")]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Shell::IAdaptiveCard>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities', 'UI_Shell'*"]
    #[cfg(feature = "UI_Shell")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Shell::IAdaptiveCard>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn AttributionDisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_UserActivities'*"]
    pub fn SetAttributionDisplayText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserActivityVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityVisualElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityVisualElements {}
unsafe impl ::windows::core::RuntimeType for UserActivityVisualElements {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityVisualElements;{94757513-262f-49ef-bbbf-9b75d2e85250})");
}
unsafe impl ::windows::core::Interface for UserActivityVisualElements {
    type Vtable = IUserActivityVisualElementsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94757513_262f_49ef_bbbf_9b75d2e85250);
}
impl ::windows::core::RuntimeName for UserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityVisualElements";
}
impl ::core::convert::From<UserActivityVisualElements> for ::windows::core::IUnknown {
    fn from(value: UserActivityVisualElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityVisualElements> for ::windows::core::IUnknown {
    fn from(value: &UserActivityVisualElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserActivityVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserActivityVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityVisualElements> for ::windows::core::IInspectable {
    fn from(value: UserActivityVisualElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityVisualElements> for ::windows::core::IInspectable {
    fn from(value: &UserActivityVisualElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserActivityVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserActivityVisualElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityVisualElements {}
unsafe impl ::core::marker::Sync for UserActivityVisualElements {}
