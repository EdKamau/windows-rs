#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequest(::windows::core::IUnknown);
impl BarcodeScannerDisableScannerRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDisableScannerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDisableScannerRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerDisableScannerRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest;{88ecf7c0-37b9-4275-8e77-c8e52ae5a9c8})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88ecf7c0_37b9_4275_8e77_c8e52ae5a9c8);
}
impl ::windows::core::RuntimeName for BarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerDisableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerDisableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerDisableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerDisableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerDisableScannerRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerDisableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerDisableScannerRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDisableScannerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs;{7006e142-e802-46f5-b604-352a15ce9232})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7006e142_e802_46f5_b604_352a15ce9232);
}
impl ::windows::core::RuntimeName for BarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequest(::windows::core::IUnknown);
impl BarcodeScannerEnableScannerRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerEnableScannerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerEnableScannerRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerEnableScannerRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest;{c0b3e9ba-816a-452b-bd77-b7e453ec446d})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b3e9ba_816a_452b_bd77_b7e453ec446d);
}
impl ::windows::core::RuntimeName for BarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerEnableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerEnableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerEnableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerEnableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerEnableScannerRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerEnableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerEnableScannerRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerEnableScannerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs;{956c9419-7b4e-4451-8c41-8e10cfbc5b41})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x956c9419_7b4e_4451_8c41_8e10cfbc5b41);
}
impl ::windows::core::RuntimeName for BarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerFrameReader(::windows::core::IUnknown);
impl BarcodeScannerFrameReader {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryAcquireLatestFrameAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerFrameReader {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader;{dbc72b07-64c3-482b-93c8-65fb33c22208})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbc72b07_64c3_482b_93c8_65fb33c22208);
}
impl ::windows::core::RuntimeName for BarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
}
impl ::core::convert::From<BarcodeScannerFrameReader> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReader> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerFrameReader> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReader> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BarcodeScannerFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScannerFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerFrameReader {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReader {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerFrameReaderFrameArrivedEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs;{b0bbd604-54fd-436d-8629-712e787223dd})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bbd604_54fd_436d_8629_712e787223dd);
}
impl ::windows::core::RuntimeName for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(::windows::core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Symbology(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::BarcodeSymbologyAttributes>>(&self, attributes: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerGetSymbologyAttributesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest;{9774c46a-58e4-4c5f-b8e9-e41467632700})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9774c46a_58e4_4c5f_b8e9_e41467632700);
}
impl ::windows::core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerGetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerGetSymbologyAttributesRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs;{7f89de3e-fb5d-493c-b402-356b24d574a6})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f89de3e_fb5d_493c_b402_356b24d574a6);
}
impl ::windows::core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequest(::windows::core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerHideVideoPreviewRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest;{fa4ebe7f-6670-40e1-b90b-bb10d8d425fa})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa4ebe7f_6670_40e1_b90b_bb10d8d425fa);
}
impl ::windows::core::RuntimeName for BarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerHideVideoPreviewRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerHideVideoPreviewRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerHideVideoPreviewRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerHideVideoPreviewRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerHideVideoPreviewRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerHideVideoPreviewRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs;{16a281fc-d6be-4bc7-9df1-33741f3eadea})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a281fc_d6be_4bc7_9df1_33741f3eadea);
}
impl ::windows::core::RuntimeName for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerProviderConnection(::windows::core::IUnknown);
impl BarcodeScannerProviderConnection {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSymbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetCompanyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportScannedDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::BarcodeScannerReport>>(&self, report: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), report.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), state, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportErrorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::UnifiedPosErrorData>>(&self, errordata: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), errordata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportErrorAsyncWithScanReport<'a, Param0: ::windows::core::IntoParam<'a, super::UnifiedPosErrorData>, Param2: ::windows::core::IntoParam<'a, super::BarcodeScannerReport>>(&self, errordata: Param0, isretriable: bool, scanreport: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), errordata.into_param().abi(), isretriable, scanreport.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableScannerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnableScannerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableScannerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisableScannerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActiveSymbologiesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetActiveSymbologiesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartSoftwareTriggerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStartSoftwareTriggerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopSoftwareTriggerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopSoftwareTriggerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn HideVideoPreviewRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHideVideoPreviewRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFrameReaderAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), preferredformat, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn CreateFrameReaderWithFormatAndSizeAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Imaging::BitmapSize>>(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), preferredformat, preferredsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerProviderConnection {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection;{b44acbed-0b3a-4fa3-86c5-491ea30780eb})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44acbed_0b3a_4fa3_86c5_491ea30780eb);
}
impl ::windows::core::RuntimeName for BarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
}
impl ::core::convert::From<BarcodeScannerProviderConnection> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerProviderConnection> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BarcodeScannerProviderConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerProviderConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScannerProviderConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerProviderConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerProviderConnection {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderConnection {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerProviderTriggerDetails(::windows::core::IUnknown);
impl BarcodeScannerProviderTriggerDetails {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerProviderTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails;{50856d82-24e3-48ce-99c7-70aac1cbc9f7})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50856d82_24e3_48ce_99c7_70aac1cbc9f7);
}
impl ::windows::core::RuntimeName for BarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
}
impl ::core::convert::From<BarcodeScannerProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderTriggerDetails {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(::windows::core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Symbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetActiveSymbologiesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest;{db3f32b9-f7da-41a1-9f79-07bcd95f0bdf})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb3f32b9_f7da_41a1_9f79_07bcd95f0bdf);
}
impl ::windows::core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetActiveSymbologiesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerSetActiveSymbologiesRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs;{06305afa-7bf6-4d52-801a-330272f60ae1})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06305afa_7bf6_4d52_801a_330272f60ae1);
}
impl ::windows::core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(::windows::core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Symbology(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Attributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetSymbologyAttributesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest;{32fb814f-a37f-48b0-acea-dce1480f12ae})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32fb814f_a37f_48b0_acea_dce1480f12ae);
}
impl ::windows::core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerSetSymbologyAttributesRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs;{b2b89809-9824-47d4-85bd-d0077baa7bd2})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b89809_9824_47d4_85bd_d0077baa7bd2);
}
impl ::windows::core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(::windows::core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStartSoftwareTriggerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest;{e3fa7b27-ff62-4454-af4a-cb6144a3e3f7})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3fa7b27_ff62_4454_af4a_cb6144a3e3f7);
}
impl ::windows::core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerStartSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStartSoftwareTriggerRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs;{2305d843-c88f-4f3b-8c3b-d3df071051ec})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2305d843_c88f_4f3b_8c3b_d3df071051ec);
}
impl ::windows::core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(::windows::core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequest {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStopSoftwareTriggerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest;{6f9faf35-e287-4ca8-b70d-5a91d694f668})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f9faf35_e287_4ca8_b70d_5a91d694f668);
}
impl ::windows::core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequest> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequest> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequest> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequest> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequest {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows::core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<BarcodeScannerStopSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStopSoftwareTriggerRequest>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs;{eac34450-4eb7-481a-9273-147a273b99b8})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeac34450_4eb7_481a_9273_147a273b99b8);
}
impl ::windows::core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl ::core::marker::Copy for BarcodeScannerTriggerState {}
impl ::core::clone::Clone for BarcodeScannerTriggerState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BarcodeScannerTriggerState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BarcodeScannerTriggerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerTriggerState {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerTriggerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState;i4)");
}
impl ::windows::core::DefaultType for BarcodeScannerTriggerState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeScannerVideoFrame(::windows::core::IUnknown);
impl BarcodeScannerVideoFrame {
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::Imaging::BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerVideoFrame {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerVideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame;{7e585248-9df7-4121-a175-801d8000112e})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e585248_9df7_4121_a175_801d8000112e);
}
impl ::windows::core::RuntimeName for BarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
}
impl ::core::convert::From<BarcodeScannerVideoFrame> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerVideoFrame> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerVideoFrame> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerVideoFrame> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BarcodeScannerVideoFrame> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerVideoFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScannerVideoFrame> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerVideoFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerVideoFrame {}
unsafe impl ::core::marker::Sync for BarcodeScannerVideoFrame {}
#[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
#[repr(transparent)]
pub struct BarcodeSymbologyAttributesBuilder(::windows::core::IUnknown);
impl BarcodeSymbologyAttributesBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeSymbologyAttributesBuilder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_PointOfService_Provider'*"]
    pub fn CreateAttributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeSymbologyAttributesBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeSymbologyAttributesBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeSymbologyAttributesBuilder {}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyAttributesBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder;{c57b0cbf-e4f5-40b9-84cf-e63fbaea42b4})");
}
unsafe impl ::windows::core::Interface for BarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc57b0cbf_e4f5_40b9_84cf_e63fbaea42b4);
}
impl ::windows::core::RuntimeName for BarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
}
impl ::core::convert::From<BarcodeSymbologyAttributesBuilder> for ::windows::core::IUnknown {
    fn from(value: BarcodeSymbologyAttributesBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributesBuilder> for ::windows::core::IUnknown {
    fn from(value: &BarcodeSymbologyAttributesBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeSymbologyAttributesBuilder> for ::windows::core::IInspectable {
    fn from(value: BarcodeSymbologyAttributesBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributesBuilder> for ::windows::core::IInspectable {
    fn from(value: &BarcodeSymbologyAttributesBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributesBuilder {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributesBuilder {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88ecf7c0_37b9_4275_8e77_c8e52ae5a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerDisableScannerRequest2 {
    type Vtable = IBarcodeScannerDisableScannerRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccdfe625_65c3_4ccc_b457_f39c7a9ea60d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7006e142_e802_46f5_b604_352a15ce9232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestEventArgsVtbl(
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
pub struct IBarcodeScannerEnableScannerRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b3e9ba_816a_452b_bd77_b7e453ec446d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerEnableScannerRequest2 {
    type Vtable = IBarcodeScannerEnableScannerRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71a4f2a8_9905_41ac_9121_b645916a84a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x956c9419_7b4e_4451_8c41_8e10cfbc5b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestEventArgsVtbl(
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
pub struct IBarcodeScannerFrameReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbc72b07_64c3_482b_93c8_65fb33c22208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bbd604_54fd_436d_8629_712e787223dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9774c46a_58e4_4c5f_b8e9_e41467632700);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a6a2b13_75a8_49fb_b852_bfb93d760af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f89de3e_fb5d_493c_b402_356b24d574a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl(
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
pub struct IBarcodeScannerHideVideoPreviewRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa4ebe7f_6670_40e1_b90b_bb10d8d425fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerHideVideoPreviewRequest2 {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e28435d_9814_431d_a2f2_d6248c5ad4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a281fc_d6be_4bc7_9df1_33741f3eadea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgsVtbl(
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
pub struct IBarcodeScannerProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44acbed_0b3a_4fa3_86c5_491ea30780eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, report: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: BarcodeScannerTriggerState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: ::windows::core::RawPtr, isretriable: bool, scanreport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerProviderConnection2 {
    type Vtable = IBarcodeScannerProviderConnection2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe9b53cd_1134_418c_a06b_04423a73f3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50856d82_24e3_48ce_99c7_70aac1cbc9f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderTriggerDetailsVtbl(
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
pub struct IBarcodeScannerSetActiveSymbologiesRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb3f32b9_f7da_41a1_9f79_07bcd95f0bdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5ff6edf_fa9a_4749_b11b_e8fccb75bc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06305afa_7bf6_4d52_801a_330272f60ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl(
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
pub struct IBarcodeScannerSetSymbologyAttributesRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32fb814f_a37f_48b0_acea_dce1480f12ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdffbbfc1_dba8_4b77_be1e_b56cd72f65b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b89809_9824_47d4_85bd_d0077baa7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl(
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
pub struct IBarcodeScannerStartSoftwareTriggerRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3fa7b27_ff62_4454_af4a_cb6144a3e3f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb72a25c_6658_4765_a68e_327482653deb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2305d843_c88f_4f3b_8c3b_d3df071051ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl(
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
pub struct IBarcodeScannerStopSoftwareTriggerRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f9faf35_e287_4ca8_b70d_5a91d694f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb57c5dd_fe50_49f8_a0b4_bdc230814da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeac34450_4eb7_481a_9273_147a273b99b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl(
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
pub struct IBarcodeScannerVideoFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e585248_9df7_4121_a175_801d8000112e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerVideoFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributesBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc57b0cbf_e4f5_40b9_84cf_e63fbaea42b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributesBuilderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
