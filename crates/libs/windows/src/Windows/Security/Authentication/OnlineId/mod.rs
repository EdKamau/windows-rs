#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialPromptType {}
impl ::core::clone::Clone for CredentialPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CredentialPromptType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CredentialPromptType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialPromptType {}
unsafe impl ::windows::core::RuntimeType for CredentialPromptType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
}
impl ::windows::core::DefaultType for CredentialPromptType {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdAuthenticator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: ::windows::core::RawPtr, credentialprompttype: CredentialPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequestFactory {
    type Vtable = IOnlineIdServiceTicketRequestFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbebb0a08_9e73_4077_9614_08614c0bc245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUserVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUserVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorStatics {
    type Vtable = IOnlineIdSystemAuthenticatorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85047792_f634_41e3_96a4_5164e902c740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemIdentity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemTicketResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OnlineIdSystemTicketStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserIdentity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserIdentity {
    type Vtable = IUserIdentityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserIdentityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdAuthenticator(::windows::core::IUnknown);
impl OnlineIdAuthenticator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdAuthenticator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateUserAsync<'a, Param0: ::windows::core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AuthenticateUserAsyncAdvanced<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>>(&self, requests: Param0, credentialprompttype: CredentialPromptType) -> ::windows::core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), requests.into_param().abi(), credentialprompttype, &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutUserAsync(&self) -> ::windows::core::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SignOutUserOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn SetApplicationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn CanSignOut(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn AuthenticatedSafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdAuthenticator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdAuthenticator {}
unsafe impl ::windows::core::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator;{a003f58a-29ab-4817-b884-d7516dad18b9})");
}
unsafe impl ::windows::core::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
impl ::windows::core::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows::core::IUnknown {
    fn from(value: OnlineIdAuthenticator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows::core::IInspectable {
    fn from(value: OnlineIdAuthenticator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdAuthenticator {}
unsafe impl ::core::marker::Sync for OnlineIdAuthenticator {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdServiceTicket(::windows::core::IUnknown);
impl OnlineIdServiceTicket {
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Request(&self) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdServiceTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicket {}
unsafe impl ::windows::core::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket;{c95c547f-d781-4a94-acb8-c59874238c26})");
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows::core::IUnknown {
    fn from(value: OnlineIdServiceTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows::core::IInspectable {
    fn from(value: OnlineIdServiceTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicket {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicket {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdServiceTicketRequest(::windows::core::IUnknown);
impl OnlineIdServiceTicketRequest {
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Policy(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn CreateOnlineIdServiceTicketRequest<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(service: Param0, policy: Param1) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), service.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn CreateOnlineIdServiceTicketRequestAdvanced<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(service: Param0) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), service.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OnlineIdServiceTicketRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicketRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicketRequest {}
unsafe impl ::windows::core::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest;{297445d3-fb63-4135-8909-4e354c061466})");
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows::core::IUnknown {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows::core::IInspectable {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicketRequest {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicketRequest {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
pub struct OnlineIdSystemAuthenticator {}
impl OnlineIdSystemAuthenticator {
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Default() -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'System'*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdSystemAuthenticatorForUser(::windows::core::IUnknown);
impl OnlineIdSystemAuthenticatorForUser {
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTicketAsync<'a, Param0: ::windows::core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn SetApplicationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemAuthenticatorForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemAuthenticatorForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser;{5798befb-1de4-4186-a2e6-b563f86aaf44})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUserVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::core::marker::Sync for OnlineIdSystemAuthenticatorForUser {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdSystemIdentity(::windows::core::IUnknown);
impl OnlineIdSystemIdentity {
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Ticket(&self) -> ::windows::core::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicket>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemIdentity {}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity;{743cd20d-b6ca-434d-8124-53ea12685307})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
impl ::windows::core::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemIdentity {}
unsafe impl ::core::marker::Sync for OnlineIdSystemIdentity {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdSystemTicketResult(::windows::core::IUnknown);
impl OnlineIdSystemTicketResult {
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Identity(&self) -> ::windows::core::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemIdentity>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Status(&self) -> ::windows::core::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__: OnlineIdSystemTicketStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemTicketStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemTicketResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemTicketResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemTicketResult {}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult;{db0a5ff8-b098-4acd-9d13-9e640652b5b6})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
impl ::windows::core::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemTicketResult {}
unsafe impl ::core::marker::Sync for OnlineIdSystemTicketResult {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl ::core::marker::Copy for OnlineIdSystemTicketStatus {}
impl ::core::clone::Clone for OnlineIdSystemTicketStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OnlineIdSystemTicketStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OnlineIdSystemTicketStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemTicketStatus {}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
}
impl ::windows::core::DefaultType for OnlineIdSystemTicketStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct SignOutUserOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl SignOutUserOperation {
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for SignOutUserOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for SignOutUserOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for SignOutUserOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.SignOutUserOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for SignOutUserOperation {
    type Vtable = super::super::super::Foundation::IAsyncActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
#[cfg(all(feature = "Foundation", feature = "std"))]
impl SignOutUserOperation {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "std"))]
impl ::std::future::Future for SignOutUserOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<SignOutUserOperation> for ::windows::core::IUnknown {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&SignOutUserOperation> for ::windows::core::IUnknown {
    fn from(value: &SignOutUserOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<SignOutUserOperation> for ::windows::core::IInspectable {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&SignOutUserOperation> for ::windows::core::IInspectable {
    fn from(value: &SignOutUserOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    type Error = ::windows::core::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    type Error = ::windows::core::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncAction>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for SignOutUserOperation {}
#[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct UserAuthenticationOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl UserAuthenticationOperation {
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserIdentity>(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for UserAuthenticationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for UserAuthenticationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserAuthenticationOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for UserAuthenticationOperation {
    type Vtable = super::super::super::Foundation::IAsyncOperationVtbl<UserIdentity>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
#[cfg(all(feature = "Foundation", feature = "std"))]
impl UserAuthenticationOperation {
    pub fn get(&self) -> ::windows::core::Result<UserIdentity> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "std"))]
impl ::std::future::Future for UserAuthenticationOperation {
    type Output = ::windows::core::Result<UserIdentity>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<UserAuthenticationOperation> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<UserAuthenticationOperation> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    type Error = ::windows::core::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncOperation<UserIdentity>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for UserAuthenticationOperation {}
#[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
#[repr(transparent)]
pub struct UserIdentity(::windows::core::IUnknown);
impl UserIdentity {
    #[doc = "*Required features: 'Security_Authentication_OnlineId', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tickets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn SafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn SignInName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn IsBetaAccount(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_OnlineId'*"]
    pub fn IsConfirmedPC(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for UserIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserIdentity {}
unsafe impl ::windows::core::RuntimeType for UserIdentity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})");
}
unsafe impl ::windows::core::Interface for UserIdentity {
    type Vtable = IUserIdentityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
impl ::windows::core::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
impl ::core::convert::From<UserIdentity> for ::windows::core::IUnknown {
    fn from(value: UserIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows::core::IUnknown {
    fn from(value: &UserIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserIdentity> for ::windows::core::IInspectable {
    fn from(value: UserIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows::core::IInspectable {
    fn from(value: &UserIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserIdentity {}
unsafe impl ::core::marker::Sync for UserIdentity {}
