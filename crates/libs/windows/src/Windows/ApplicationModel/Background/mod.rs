#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ActivitySensorTrigger(::windows::core::IUnknown);
impl ActivitySensorTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Sensors', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Sensors', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(reportintervalinmilliseconds: u32) -> ::windows::core::Result<ActivitySensorTrigger> {
        Self::IActivitySensorTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reportintervalinmilliseconds, &mut result__).from_abi::<ActivitySensorTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorTriggerFactory<R, F: FnOnce(&IActivitySensorTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ActivitySensorTrigger, IActivitySensorTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ActivitySensorTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensorTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorTrigger {}
unsafe impl ::windows::core::RuntimeType for ActivitySensorTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ActivitySensorTrigger;{d0dd4342-e37b-4823-a5fe-6b31dfefdeb0})");
}
unsafe impl ::windows::core::Interface for ActivitySensorTrigger {
    type Vtable = IActivitySensorTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0dd4342_e37b_4823_a5fe_6b31dfefdeb0);
}
impl ::windows::core::RuntimeName for ActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ActivitySensorTrigger";
}
impl ::core::convert::From<ActivitySensorTrigger> for ::windows::core::IUnknown {
    fn from(value: ActivitySensorTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorTrigger> for ::windows::core::IUnknown {
    fn from(value: &ActivitySensorTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensorTrigger> for ::windows::core::IInspectable {
    fn from(value: ActivitySensorTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorTrigger> for ::windows::core::IInspectable {
    fn from(value: &ActivitySensorTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ActivitySensorTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ActivitySensorTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ActivitySensorTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ActivitySensorTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ActivitySensorTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ActivitySensorTrigger {}
unsafe impl ::core::marker::Sync for ActivitySensorTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithWakeupCapability: Self = Self(1i32);
    pub const AllowedWithoutWakeupCapability: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
}
impl ::core::marker::Copy for AlarmAccessStatus {}
impl ::core::clone::Clone for AlarmAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AlarmAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AlarmAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlarmAccessStatus {}
unsafe impl ::windows::core::RuntimeType for AlarmAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.AlarmAccessStatus;i4)");
}
impl ::windows::core::DefaultType for AlarmAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
pub struct AlarmApplicationManager {}
impl AlarmApplicationManager {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetAccessStatus() -> ::windows::core::Result<AlarmAccessStatus> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__: AlarmAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AlarmAccessStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAlarmApplicationManagerStatics<R, F: FnOnce(&IAlarmApplicationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AlarmApplicationManager, IAlarmApplicationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AlarmApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AlarmApplicationManager";
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct AppBroadcastTrigger(::windows::core::IUnknown);
impl AppBroadcastTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetProviderInfo<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastTriggerProviderInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn ProviderInfo(&self) -> ::windows::core::Result<AppBroadcastTriggerProviderInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastTriggerProviderInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CreateAppBroadcastTrigger<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(providerkey: Param0) -> ::windows::core::Result<AppBroadcastTrigger> {
        Self::IAppBroadcastTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), providerkey.into_param().abi(), &mut result__).from_abi::<AppBroadcastTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastTriggerFactory<R, F: FnOnce(&IAppBroadcastTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastTrigger, IAppBroadcastTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppBroadcastTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTrigger {}
unsafe impl ::windows::core::RuntimeType for AppBroadcastTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTrigger;{74d4f496-8d37-44ec-9481-2a0b9854eb48})");
}
unsafe impl ::windows::core::Interface for AppBroadcastTrigger {
    type Vtable = IAppBroadcastTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d4f496_8d37_44ec_9481_2a0b9854eb48);
}
impl ::windows::core::RuntimeName for AppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTrigger";
}
impl ::core::convert::From<AppBroadcastTrigger> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTrigger> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastTrigger> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTrigger> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppBroadcastTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: AppBroadcastTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppBroadcastTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppBroadcastTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &AppBroadcastTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastTrigger {}
unsafe impl ::core::marker::Sync for AppBroadcastTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct AppBroadcastTriggerProviderInfo(::windows::core::IUnknown);
impl AppBroadcastTriggerProviderInfo {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetDisplayNameResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetLogoResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn LogoResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetVideoKeyFrameInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoKeyFrameInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetMaxVideoBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn MaxVideoBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetMaxVideoWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn MaxVideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetMaxVideoHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn MaxVideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastTriggerProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTriggerProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTriggerProviderInfo {}
unsafe impl ::windows::core::RuntimeType for AppBroadcastTriggerProviderInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo;{f219352d-9de8-4420-9ce2-5eff8f17376b})");
}
unsafe impl ::windows::core::Interface for AppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf219352d_9de8_4420_9ce2_5eff8f17376b);
}
impl ::windows::core::RuntimeName for AppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo";
}
impl ::core::convert::From<AppBroadcastTriggerProviderInfo> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastTriggerProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerProviderInfo> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastTriggerProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastTriggerProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppBroadcastTriggerProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastTriggerProviderInfo> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastTriggerProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerProviderInfo> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastTriggerProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastTriggerProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppBroadcastTriggerProviderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastTriggerProviderInfo {}
unsafe impl ::core::marker::Sync for AppBroadcastTriggerProviderInfo {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ApplicationTrigger(::windows::core::IUnknown);
impl ApplicationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAsyncWithArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, arguments: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for ApplicationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationTrigger {}
unsafe impl ::windows::core::RuntimeType for ApplicationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTrigger;{0b468630-9574-492c-9e93-1a3ae6335fe9})");
}
unsafe impl ::windows::core::Interface for ApplicationTrigger {
    type Vtable = IApplicationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b468630_9574_492c_9e93_1a3ae6335fe9);
}
impl ::windows::core::RuntimeName for ApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTrigger";
}
impl ::core::convert::From<ApplicationTrigger> for ::windows::core::IUnknown {
    fn from(value: ApplicationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationTrigger> for ::windows::core::IUnknown {
    fn from(value: &ApplicationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationTrigger> for ::windows::core::IInspectable {
    fn from(value: ApplicationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationTrigger> for ::windows::core::IInspectable {
    fn from(value: &ApplicationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ApplicationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ApplicationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ApplicationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ApplicationTrigger {}
unsafe impl ::core::marker::Sync for ApplicationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ApplicationTriggerDetails(::windows::core::IUnknown);
impl ApplicationTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ApplicationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for ApplicationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTriggerDetails;{97dc6ab2-2219-4a9e-9c5e-41d047f76e82})");
}
unsafe impl ::windows::core::Interface for ApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dc6ab2_2219_4a9e_9c5e_41d047f76e82);
}
impl ::windows::core::RuntimeName for ApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTriggerDetails";
}
impl ::core::convert::From<ApplicationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ApplicationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ApplicationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ApplicationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ApplicationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ApplicationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ApplicationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationTriggerDetails {}
unsafe impl ::core::marker::Sync for ApplicationTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ApplicationTriggerResult {}
impl ::core::clone::Clone for ApplicationTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ApplicationTriggerResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationTriggerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationTriggerResult {}
unsafe impl ::windows::core::RuntimeType for ApplicationTriggerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.ApplicationTriggerResult;i4)");
}
impl ::windows::core::DefaultType for ApplicationTriggerResult {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct AppointmentStoreNotificationTrigger(::windows::core::IUnknown);
impl AppointmentStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentStoreNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppointmentStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for AppointmentStoreNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger;{64d4040c-c201-42ad-aa2a-e21ba3425b6d})");
}
unsafe impl ::windows::core::Interface for AppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d4040c_c201_42ad_aa2a_e21ba3425b6d);
}
impl ::windows::core::RuntimeName for AppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger";
}
impl ::core::convert::From<AppointmentStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: AppointmentStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &AppointmentStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: AppointmentStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &AppointmentStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &AppointmentStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundAccessRequestKind {}
impl ::core::clone::Clone for BackgroundAccessRequestKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundAccessRequestKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundAccessRequestKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundAccessRequestKind {}
unsafe impl ::windows::core::RuntimeType for BackgroundAccessRequestKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessRequestKind;i4)");
}
impl ::windows::core::DefaultType for BackgroundAccessRequestKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: Self = Self(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const AlwaysAllowed: Self = Self(4i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(5i32);
    pub const DeniedBySystemPolicy: Self = Self(6i32);
    pub const DeniedByUser: Self = Self(7i32);
}
impl ::core::marker::Copy for BackgroundAccessStatus {}
impl ::core::clone::Clone for BackgroundAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundAccessStatus {}
unsafe impl ::windows::core::RuntimeType for BackgroundAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessStatus;i4)");
}
impl ::windows::core::DefaultType for BackgroundAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
pub struct BackgroundExecutionManager {}
impl BackgroundExecutionManager {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForApplicationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn RemoveAccess() -> ::windows::core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn RemoveAccessForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), applicationid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetAccessStatus() -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetAccessStatusForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(requestedaccess: BackgroundAccessRequestKind, reason: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requestedaccess, reason.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindForModernStandbyAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(requestedaccess: BackgroundAccessRequestKind, reason: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requestedaccess, reason.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetAccessStatusForModernStandby() -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetAccessStatusForModernStandbyForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__: BackgroundAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<BackgroundAccessStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics<R, F: FnOnce(&IBackgroundExecutionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics2<R, F: FnOnce(&IBackgroundExecutionManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics3<R, F: FnOnce(&IBackgroundExecutionManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BackgroundExecutionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundExecutionManager";
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskBuilder(::windows::core::IUnknown);
impl BackgroundTaskBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTaskBuilder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetTaskEntryPoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskEntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetTrigger<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundTrigger>>(&self, trigger: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), trigger.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn AddCondition<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCondition>>(&self, condition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), condition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Register(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CancelOnConditionLoss(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetIsNetworkRequested(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn IsNetworkRequested(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetTaskGroup<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskRegistrationGroup>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetTaskEntryPointClsid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskentrypoint: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskBuilder5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), taskentrypoint.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for BackgroundTaskBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskBuilder {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskBuilder;{0351550e-3e64-4572-a93a-84075a37c917})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0351550e_3e64_4572_a93a_84075a37c917);
}
impl ::windows::core::RuntimeName for BackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskBuilder";
}
impl ::core::convert::From<BackgroundTaskBuilder> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskBuilder> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskBuilder> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskBuilder> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskCanceledEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskCanceledEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCanceledEventHandlerBox::<F> { vtable: &BackgroundTaskCanceledEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundTaskInstance>>(&self, sender: Param0, reason: BackgroundTaskCancellationReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), reason).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCanceledEventHandlerBox<F: FnMut(&::core::option::Option<IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + 'static> {
    vtable: *const BackgroundTaskCanceledEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + 'static> BackgroundTaskCanceledEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCanceledEventHandlerVtbl = BackgroundTaskCanceledEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<BackgroundTaskCanceledEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, reason: BackgroundTaskCancellationReason) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType), reason).into()
    }
}
impl ::core::clone::Clone for BackgroundTaskCanceledEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCanceledEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCanceledEventHandler {}
unsafe impl ::windows::core::Interface for BackgroundTaskCanceledEventHandler {
    type Vtable = BackgroundTaskCanceledEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4bac0_51f8_4c57_ac3f_156dd1680c4f);
}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskCanceledEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a6c4bac0-51f8-4c57-ac3f-156dd1680c4f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCanceledEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, reason: BackgroundTaskCancellationReason) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: Self = Self(0i32);
    pub const Terminating: Self = Self(1i32);
    pub const LoggingOff: Self = Self(2i32);
    pub const ServicingUpdate: Self = Self(3i32);
    pub const IdleTask: Self = Self(4i32);
    pub const Uninstall: Self = Self(5i32);
    pub const ConditionLoss: Self = Self(6i32);
    pub const SystemPolicy: Self = Self(7i32);
    pub const QuietHoursEntered: Self = Self(8i32);
    pub const ExecutionTimeExceeded: Self = Self(9i32);
    pub const ResourceRevocation: Self = Self(10i32);
    pub const EnergySaver: Self = Self(11i32);
}
impl ::core::marker::Copy for BackgroundTaskCancellationReason {}
impl ::core::clone::Clone for BackgroundTaskCancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTaskCancellationReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTaskCancellationReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCancellationReason {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskCancellationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskCancellationReason;i4)");
}
impl ::windows::core::DefaultType for BackgroundTaskCancellationReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventArgs(::windows::core::IUnknown);
impl BackgroundTaskCompletedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CheckResult(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BackgroundTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCompletedEventArgs {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs;{565d25cf-f209-48f4-9967-2b184f7bfbf0})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x565d25cf_f209_48f4_9967_2b184f7bfbf0);
}
impl ::windows::core::RuntimeName for BackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs";
}
impl ::core::convert::From<BackgroundTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskCompletedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCompletedEventHandlerBox::<F> { vtable: &BackgroundTaskCompletedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskRegistration>, Param1: ::windows::core::IntoParam<'a, BackgroundTaskCompletedEventArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCompletedEventHandlerBox<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const BackgroundTaskCompletedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + 'static> BackgroundTaskCompletedEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCompletedEventHandlerVtbl = BackgroundTaskCompletedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<BackgroundTaskCompletedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <BackgroundTaskRegistration as ::windows::core::Abi>::Abi as *const <BackgroundTaskRegistration as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <BackgroundTaskCompletedEventArgs as ::windows::core::Abi>::Abi as *const <BackgroundTaskCompletedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for BackgroundTaskCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCompletedEventHandler {}
unsafe impl ::windows::core::Interface for BackgroundTaskCompletedEventHandler {
    type Vtable = BackgroundTaskCompletedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b38e929_a086_46a7_a678_439135822bcf);
}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5b38e929-a086-46a7-a678-439135822bcf}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCompletedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskDeferral(::windows::core::IUnknown);
impl BackgroundTaskDeferral {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BackgroundTaskDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskDeferral {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskDeferral;{93cc156d-af27-4dd3-846e-24ee40cadd25})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cc156d_af27_4dd3_846e_24ee40cadd25);
}
impl ::windows::core::RuntimeName for BackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskDeferral";
}
impl ::core::convert::From<BackgroundTaskDeferral> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskDeferral> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskDeferral> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskDeferral> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTaskDeferral {}
unsafe impl ::core::marker::Sync for BackgroundTaskDeferral {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskProgressEventArgs(::windows::core::IUnknown);
impl BackgroundTaskProgressEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundTaskProgressEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskProgressEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskProgressEventArgs {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskProgressEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs;{fb1468ac-8332-4d0a-9532-03eae684da31})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb1468ac_8332_4d0a_9532_03eae684da31);
}
impl ::windows::core::RuntimeName for BackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs";
}
impl ::core::convert::From<BackgroundTaskProgressEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskProgressEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskProgressEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskProgressEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskProgressEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskProgressEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskProgressEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskProgressEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskProgressEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskProgressEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskProgressEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskProgressEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTaskProgressEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskProgressEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskProgressEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskProgressEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskProgressEventHandlerBox::<F> { vtable: &BackgroundTaskProgressEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskRegistration>, Param1: ::windows::core::IntoParam<'a, BackgroundTaskProgressEventArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskProgressEventHandlerBox<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const BackgroundTaskProgressEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<BackgroundTaskRegistration>, &::core::option::Option<BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + 'static> BackgroundTaskProgressEventHandlerBox<F> {
    const VTABLE: BackgroundTaskProgressEventHandlerVtbl = BackgroundTaskProgressEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<BackgroundTaskProgressEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <BackgroundTaskRegistration as ::windows::core::Abi>::Abi as *const <BackgroundTaskRegistration as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <BackgroundTaskProgressEventArgs as ::windows::core::Abi>::Abi as *const <BackgroundTaskProgressEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for BackgroundTaskProgressEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskProgressEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskProgressEventHandler {}
unsafe impl ::windows::core::Interface for BackgroundTaskProgressEventHandler {
    type Vtable = BackgroundTaskProgressEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46e0683c_8a88_4c99_804c_76897f6277a6);
}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskProgressEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{46e0683c-8a88-4c99-804c-76897f6277a6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskProgressEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskRegistration(::windows::core::IUnknown);
impl BackgroundTaskRegistration {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskProgressEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCompletedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), canceltask).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>> {
        Self::IBackgroundTaskRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTaskGroups() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetTaskGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(groupid: Param0) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), groupid.into_param().abi(), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics<R, F: FnOnce(&IBackgroundTaskRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics2<R, F: FnOnce(&IBackgroundTaskRegistrationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTaskRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskRegistration {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistration;{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10654cc2_a26e_43bf_8c12_1fb40dbfbfa0);
}
impl ::windows::core::RuntimeName for BackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistration";
}
impl ::core::convert::From<BackgroundTaskRegistration> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskRegistration> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskRegistration> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskRegistration> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundTaskRegistration> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundTaskRegistration> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for &BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::core::convert::TryInto::<IBackgroundTaskRegistration>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BackgroundTaskRegistration> for IBackgroundTaskRegistration2 {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundTaskRegistration> for IBackgroundTaskRegistration2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration2> for BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration2> for &BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration2> {
        ::core::convert::TryInto::<IBackgroundTaskRegistration2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BackgroundTaskRegistration> for IBackgroundTaskRegistration3 {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundTaskRegistration> for IBackgroundTaskRegistration3 {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundTaskRegistration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration3> for BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration3> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration3> for &BackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration3> {
        ::core::convert::TryInto::<IBackgroundTaskRegistration3>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundTaskRegistration {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistration {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskRegistrationGroup(::windows::core::IUnknown);
impl BackgroundTaskRegistrationGroup {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'ApplicationModel_Activation', 'Foundation'*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CreateWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0, name: Param1) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationGroupFactory<R, F: FnOnce(&IBackgroundTaskRegistrationGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTaskRegistrationGroup, IBackgroundTaskRegistrationGroupFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTaskRegistrationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskRegistrationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskRegistrationGroup {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskRegistrationGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup;{2ab1919a-871b-4167-8a76-055cd67b5b23})");
}
unsafe impl ::windows::core::Interface for BackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ab1919a_871b_4167_8a76_055cd67b5b23);
}
impl ::windows::core::RuntimeName for BackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup";
}
impl ::core::convert::From<BackgroundTaskRegistrationGroup> for ::windows::core::IUnknown {
    fn from(value: BackgroundTaskRegistrationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskRegistrationGroup> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTaskRegistrationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTaskRegistrationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTaskRegistrationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTaskRegistrationGroup> for ::windows::core::IInspectable {
    fn from(value: BackgroundTaskRegistrationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTaskRegistrationGroup> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTaskRegistrationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTaskRegistrationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTaskRegistrationGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTaskRegistrationGroup {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistrationGroup {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTaskThrottleCounter {}
impl ::core::clone::Clone for BackgroundTaskThrottleCounter {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTaskThrottleCounter {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTaskThrottleCounter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskThrottleCounter {}
unsafe impl ::windows::core::RuntimeType for BackgroundTaskThrottleCounter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskThrottleCounter;i4)");
}
impl ::windows::core::DefaultType for BackgroundTaskThrottleCounter {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
pub struct BackgroundWorkCost {}
impl BackgroundWorkCost {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CurrentBackgroundWorkCost() -> ::windows::core::Result<BackgroundWorkCostValue> {
        Self::IBackgroundWorkCostStatics(|this| unsafe {
            let mut result__: BackgroundWorkCostValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundWorkCostValue>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundWorkCostStatics<R, F: FnOnce(&IBackgroundWorkCostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundWorkCost, IBackgroundWorkCostStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BackgroundWorkCost {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundWorkCost";
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundWorkCostValue {}
impl ::core::clone::Clone for BackgroundWorkCostValue {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundWorkCostValue {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundWorkCostValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundWorkCostValue {}
unsafe impl ::windows::core::RuntimeType for BackgroundWorkCostValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundWorkCostValue;i4)");
}
impl ::windows::core::DefaultType for BackgroundWorkCostValue {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTrigger(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisherTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementPublisherTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Advertisement'*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Advertisement(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn UseExtendedFormat(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetUseExtendedFormat(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger;{ab3e2612-25d3-48ae-8724-d81877ae6129})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3e2612_25d3_48ae_8724_d81877ae6129);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTrigger> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTrigger> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTrigger> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTrigger> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BluetoothLEAdvertisementPublisherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: BluetoothLEAdvertisementPublisherTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BluetoothLEAdvertisementPublisherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &BluetoothLEAdvertisementPublisherTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &BluetoothLEAdvertisementPublisherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTrigger(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcherTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementWatcherTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth'*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth'*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SetSignalStrengthFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Advertisement'*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn AdvertisementFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Advertisement'*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn SetAdvertisementFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger;{1aab1819-bce1-48eb-a827-59fb7cee52a6})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aab1819_bce1_48eb_a827_59fb7cee52a6);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTrigger> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTrigger> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTrigger> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTrigger> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BluetoothLEAdvertisementWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: BluetoothLEAdvertisementWatcherTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BluetoothLEAdvertisementWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &BluetoothLEAdvertisementWatcherTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &BluetoothLEAdvertisementWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct CachedFileUpdaterTrigger(::windows::core::IUnknown);
impl CachedFileUpdaterTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CachedFileUpdaterTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterTrigger {}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTrigger;{e21caeeb-32f2-4d31-b553-b9e01bde37e0})");
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe21caeeb_32f2_4d31_b553_b9e01bde37e0);
}
impl ::windows::core::RuntimeName for CachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTrigger";
}
impl ::core::convert::From<CachedFileUpdaterTrigger> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterTrigger> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CachedFileUpdaterTrigger> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterTrigger> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &CachedFileUpdaterTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterTrigger {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct CachedFileUpdaterTriggerDetails(::windows::core::IUnknown);
impl CachedFileUpdaterTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Storage_Provider'*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateTarget(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__: super::super::Storage::Provider::CachedFileTarget = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileTarget>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Storage_Provider'*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateRequest(&self) -> ::windows::core::Result<super::super::Storage::Provider::FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::FileUpdateRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CanRequestUserInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails;{71838c13-1314-47b4-9597-dc7e248c17cc})");
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71838c13_1314_47b4_9597_dc7e248c17cc);
}
impl ::windows::core::RuntimeName for CachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails";
}
impl ::core::convert::From<CachedFileUpdaterTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CachedFileUpdaterTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CachedFileUpdaterTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CachedFileUpdaterTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterTriggerDetails {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ChatMessageNotificationTrigger(::windows::core::IUnknown);
impl ChatMessageNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ChatMessageNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for ChatMessageNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageNotificationTrigger;{513b43bf-1d40-5c5d-78f5-c923fee3739e})");
}
unsafe impl ::windows::core::Interface for ChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x513b43bf_1d40_5c5d_78f5_c923fee3739e);
}
impl ::windows::core::RuntimeName for ChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageNotificationTrigger";
}
impl ::core::convert::From<ChatMessageNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: ChatMessageNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: ChatMessageNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ChatMessageNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ChatMessageNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChatMessageNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatMessageNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ChatMessageNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChatMessageNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ChatMessageReceivedNotificationTrigger(::windows::core::IUnknown);
impl ChatMessageReceivedNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageReceivedNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ChatMessageReceivedNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageReceivedNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for ChatMessageReceivedNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger;{3ea3760e-baf5-4077-88e9-060cf6f0c6d5})");
}
unsafe impl ::windows::core::Interface for ChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ea3760e_baf5_4077_88e9_060cf6f0c6d5);
}
impl ::windows::core::RuntimeName for ChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger";
}
impl ::core::convert::From<ChatMessageReceivedNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: ChatMessageReceivedNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageReceivedNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageReceivedNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageReceivedNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: ChatMessageReceivedNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageReceivedNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageReceivedNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ChatMessageReceivedNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ChatMessageReceivedNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChatMessageReceivedNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatMessageReceivedNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ChatMessageReceivedNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageReceivedNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(::windows::core::IUnknown);
impl CommunicationBlockingAppSetAsActiveTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppSetAsActiveTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CommunicationBlockingAppSetAsActiveTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommunicationBlockingAppSetAsActiveTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::windows::core::RuntimeType for CommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger;{fb91f28a-16a5-486d-974c-7835a8477be2})");
}
unsafe impl ::windows::core::Interface for CommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb91f28a_16a5_486d_974c_7835a8477be2);
}
impl ::windows::core::RuntimeName for CommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger";
}
impl ::core::convert::From<CommunicationBlockingAppSetAsActiveTrigger> for ::windows::core::IUnknown {
    fn from(value: CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommunicationBlockingAppSetAsActiveTrigger> for ::windows::core::IUnknown {
    fn from(value: &CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommunicationBlockingAppSetAsActiveTrigger> for ::windows::core::IInspectable {
    fn from(value: CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommunicationBlockingAppSetAsActiveTrigger> for ::windows::core::IInspectable {
    fn from(value: &CommunicationBlockingAppSetAsActiveTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CommunicationBlockingAppSetAsActiveTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: CommunicationBlockingAppSetAsActiveTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommunicationBlockingAppSetAsActiveTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommunicationBlockingAppSetAsActiveTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &CommunicationBlockingAppSetAsActiveTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::core::marker::Sync for CommunicationBlockingAppSetAsActiveTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ContactStoreNotificationTrigger(::windows::core::IUnknown);
impl ContactStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactStoreNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContactStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactStoreNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for ContactStoreNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContactStoreNotificationTrigger;{c833419b-4705-4571-9a16-06b997bf9c96})");
}
unsafe impl ::windows::core::Interface for ContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc833419b_4705_4571_9a16_06b997bf9c96);
}
impl ::windows::core::RuntimeName for ContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContactStoreNotificationTrigger";
}
impl ::core::convert::From<ContactStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: ContactStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &ContactStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: ContactStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &ContactStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ContactStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for ContactStoreNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ContentPrefetchTrigger(::windows::core::IUnknown);
impl ContentPrefetchTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentPrefetchTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(waitinterval: Param0) -> ::windows::core::Result<ContentPrefetchTrigger> {
        Self::IContentPrefetchTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), waitinterval.into_param().abi(), &mut result__).from_abi::<ContentPrefetchTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetchTriggerFactory<R, F: FnOnce(&IContentPrefetchTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentPrefetchTrigger, IContentPrefetchTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentPrefetchTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentPrefetchTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentPrefetchTrigger {}
unsafe impl ::windows::core::RuntimeType for ContentPrefetchTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContentPrefetchTrigger;{710627ee-04fa-440b-80c0-173202199e5d})");
}
unsafe impl ::windows::core::Interface for ContentPrefetchTrigger {
    type Vtable = IContentPrefetchTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x710627ee_04fa_440b_80c0_173202199e5d);
}
impl ::windows::core::RuntimeName for ContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContentPrefetchTrigger";
}
impl ::core::convert::From<ContentPrefetchTrigger> for ::windows::core::IUnknown {
    fn from(value: ContentPrefetchTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentPrefetchTrigger> for ::windows::core::IUnknown {
    fn from(value: &ContentPrefetchTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentPrefetchTrigger> for ::windows::core::IInspectable {
    fn from(value: ContentPrefetchTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentPrefetchTrigger> for ::windows::core::IInspectable {
    fn from(value: &ContentPrefetchTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContentPrefetchTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentPrefetchTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContentPrefetchTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentPrefetchTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ContentPrefetchTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ConversationalAgentTrigger(::windows::core::IUnknown);
impl ConversationalAgentTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConversationalAgentTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConversationalAgentTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentTrigger {}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ConversationalAgentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for ConversationalAgentTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for ConversationalAgentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ConversationalAgentTrigger";
}
impl ::core::convert::From<ConversationalAgentTrigger> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentTrigger> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentTrigger> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentTrigger> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConversationalAgentTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ConversationalAgentTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConversationalAgentTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConversationalAgentTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ConversationalAgentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct CustomSystemEventTrigger(::windows::core::IUnknown);
impl CustomSystemEventTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Recurrence(&self) -> ::windows::core::Result<CustomSystemEventTriggerRecurrence> {
        let this = self;
        unsafe {
            let mut result__: CustomSystemEventTriggerRecurrence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CustomSystemEventTriggerRecurrence>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(triggerid: Param0, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows::core::Result<CustomSystemEventTrigger> {
        Self::ICustomSystemEventTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerid.into_param().abi(), recurrence, &mut result__).from_abi::<CustomSystemEventTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomSystemEventTriggerFactory<R, F: FnOnce(&ICustomSystemEventTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CustomSystemEventTrigger, ICustomSystemEventTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CustomSystemEventTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomSystemEventTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSystemEventTrigger {}
unsafe impl ::windows::core::RuntimeType for CustomSystemEventTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CustomSystemEventTrigger;{f3596798-cf6b-4ef4-a0ca-29cf4a278c87})");
}
unsafe impl ::windows::core::Interface for CustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3596798_cf6b_4ef4_a0ca_29cf4a278c87);
}
impl ::windows::core::RuntimeName for CustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CustomSystemEventTrigger";
}
impl ::core::convert::From<CustomSystemEventTrigger> for ::windows::core::IUnknown {
    fn from(value: CustomSystemEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSystemEventTrigger> for ::windows::core::IUnknown {
    fn from(value: &CustomSystemEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomSystemEventTrigger> for ::windows::core::IInspectable {
    fn from(value: CustomSystemEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSystemEventTrigger> for ::windows::core::IInspectable {
    fn from(value: &CustomSystemEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CustomSystemEventTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: CustomSystemEventTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CustomSystemEventTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &CustomSystemEventTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &CustomSystemEventTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for CustomSystemEventTriggerRecurrence {}
impl ::core::clone::Clone for CustomSystemEventTriggerRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CustomSystemEventTriggerRecurrence {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CustomSystemEventTriggerRecurrence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSystemEventTriggerRecurrence {}
unsafe impl ::windows::core::RuntimeType for CustomSystemEventTriggerRecurrence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.CustomSystemEventTriggerRecurrence;i4)");
}
impl ::windows::core::DefaultType for CustomSystemEventTriggerRecurrence {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct DeviceConnectionChangeTrigger(::windows::core::IUnknown);
impl DeviceConnectionChangeTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn CanMaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn MaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>> {
        Self::IDeviceConnectionChangeTriggerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceConnectionChangeTriggerStatics<R, F: FnOnce(&IDeviceConnectionChangeTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceConnectionChangeTrigger, IDeviceConnectionChangeTriggerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeviceConnectionChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceConnectionChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceConnectionChangeTrigger {}
unsafe impl ::windows::core::RuntimeType for DeviceConnectionChangeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger;{90875e64-3cdd-4efb-ab1c-5b3b6a60ce34})");
}
unsafe impl ::windows::core::Interface for DeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90875e64_3cdd_4efb_ab1c_5b3b6a60ce34);
}
impl ::windows::core::RuntimeName for DeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger";
}
impl ::core::convert::From<DeviceConnectionChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: DeviceConnectionChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &DeviceConnectionChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceConnectionChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: DeviceConnectionChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: &DeviceConnectionChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceConnectionChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceConnectionChangeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceConnectionChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceConnectionChangeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &DeviceConnectionChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTrigger {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct DeviceManufacturerNotificationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl DeviceManufacturerNotificationTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn TriggerQualifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(triggerqualifier: Param0, oneshot: bool) -> ::windows::core::Result<DeviceManufacturerNotificationTrigger> {
        Self::IDeviceManufacturerNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerqualifier.into_param().abi(), oneshot, &mut result__).from_abi::<DeviceManufacturerNotificationTrigger>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDeviceManufacturerNotificationTriggerFactory<R, F: FnOnce(&IDeviceManufacturerNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceManufacturerNotificationTrigger, IDeviceManufacturerNotificationTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for DeviceManufacturerNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for DeviceManufacturerNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for DeviceManufacturerNotificationTrigger {}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for DeviceManufacturerNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger;{81278ab5-41ab-16da-86c2-7f7bf0912f5b})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for DeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81278ab5_41ab_16da_86c2_7f7bf0912f5b);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for DeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<DeviceManufacturerNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: DeviceManufacturerNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&DeviceManufacturerNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &DeviceManufacturerNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<DeviceManufacturerNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: DeviceManufacturerNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&DeviceManufacturerNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &DeviceManufacturerNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<DeviceManufacturerNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceManufacturerNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&DeviceManufacturerNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceManufacturerNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &DeviceManufacturerNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct DeviceServicingTrigger(::windows::core::IUnknown);
impl DeviceServicingTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceServicingTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, deviceid: Param0, expectedduration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), expectedduration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, deviceid: Param0, expectedduration: Param1, arguments: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), expectedduration.into_param().abi(), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceServicingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceServicingTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceServicingTrigger {}
unsafe impl ::windows::core::RuntimeType for DeviceServicingTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceServicingTrigger;{1ab217ad-6e34-49d3-9e6f-17f1b6dfa881})");
}
unsafe impl ::windows::core::Interface for DeviceServicingTrigger {
    type Vtable = IDeviceServicingTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab217ad_6e34_49d3_9e6f_17f1b6dfa881);
}
impl ::windows::core::RuntimeName for DeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceServicingTrigger";
}
impl ::core::convert::From<DeviceServicingTrigger> for ::windows::core::IUnknown {
    fn from(value: DeviceServicingTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceServicingTrigger> for ::windows::core::IUnknown {
    fn from(value: &DeviceServicingTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceServicingTrigger> for ::windows::core::IInspectable {
    fn from(value: DeviceServicingTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceServicingTrigger> for ::windows::core::IInspectable {
    fn from(value: &DeviceServicingTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceServicingTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceServicingTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceServicingTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceServicingTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &DeviceServicingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceServicingTrigger {}
unsafe impl ::core::marker::Sync for DeviceServicingTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const LowBattery: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceTriggerResult {}
impl ::core::clone::Clone for DeviceTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DeviceTriggerResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DeviceTriggerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceTriggerResult {}
unsafe impl ::windows::core::RuntimeType for DeviceTriggerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.DeviceTriggerResult;i4)");
}
impl ::windows::core::DefaultType for DeviceTriggerResult {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct DeviceUseTrigger(::windows::core::IUnknown);
impl DeviceUseTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceUseTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, deviceid: Param0, arguments: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceUseTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceUseTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUseTrigger {}
unsafe impl ::windows::core::RuntimeType for DeviceUseTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceUseTrigger;{0da68011-334f-4d57-b6ec-6dca64b412e4})");
}
unsafe impl ::windows::core::Interface for DeviceUseTrigger {
    type Vtable = IDeviceUseTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0da68011_334f_4d57_b6ec_6dca64b412e4);
}
impl ::windows::core::RuntimeName for DeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceUseTrigger";
}
impl ::core::convert::From<DeviceUseTrigger> for ::windows::core::IUnknown {
    fn from(value: DeviceUseTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUseTrigger> for ::windows::core::IUnknown {
    fn from(value: &DeviceUseTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceUseTrigger> for ::windows::core::IInspectable {
    fn from(value: DeviceUseTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUseTrigger> for ::windows::core::IInspectable {
    fn from(value: &DeviceUseTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceUseTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceUseTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceUseTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceUseTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &DeviceUseTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceUseTrigger {}
unsafe impl ::core::marker::Sync for DeviceUseTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct DeviceWatcherTrigger(::windows::core::IUnknown);
impl DeviceWatcherTrigger {}
impl ::core::clone::Clone for DeviceWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherTrigger {}
unsafe impl ::windows::core::RuntimeType for DeviceWatcherTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceWatcherTrigger;{a4617fdd-8573-4260-befc-5bec89cb693d})");
}
unsafe impl ::windows::core::Interface for DeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4617fdd_8573_4260_befc_5bec89cb693d);
}
impl ::windows::core::RuntimeName for DeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceWatcherTrigger";
}
impl ::core::convert::From<DeviceWatcherTrigger> for ::windows::core::IUnknown {
    fn from(value: DeviceWatcherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherTrigger> for ::windows::core::IUnknown {
    fn from(value: &DeviceWatcherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceWatcherTrigger> for ::windows::core::IInspectable {
    fn from(value: DeviceWatcherTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherTrigger> for ::windows::core::IInspectable {
    fn from(value: &DeviceWatcherTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceWatcherTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceWatcherTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceWatcherTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &DeviceWatcherTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct EmailStoreNotificationTrigger(::windows::core::IUnknown);
impl EmailStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EmailStoreNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStoreNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for EmailStoreNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.EmailStoreNotificationTrigger;{986d06da-47eb-4268-a4f2-f3f77188388a})");
}
unsafe impl ::windows::core::Interface for EmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d06da_47eb_4268_a4f2_f3f77188388a);
}
impl ::windows::core::RuntimeName for EmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.EmailStoreNotificationTrigger";
}
impl ::core::convert::From<EmailStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: EmailStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStoreNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &EmailStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: EmailStoreNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStoreNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &EmailStoreNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EmailStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: EmailStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailStoreNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &EmailStoreNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &EmailStoreNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EmailStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for EmailStoreNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct GattCharacteristicNotificationTrigger(::windows::core::IUnknown);
impl GattCharacteristicNotificationTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Background'*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn EventTriggeringMode(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTrigger2>(self)?;
        unsafe {
            let mut result__: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>>(characteristic: Param0) -> ::windows::core::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), characteristic.into_param().abi(), &mut result__).from_abi::<GattCharacteristicNotificationTrigger>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub fn CreateWithEventTriggeringMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>>(characteristic: Param0, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), characteristic.into_param().abi(), eventtriggeringmode, &mut result__).from_abi::<GattCharacteristicNotificationTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory2<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GattCharacteristicNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattCharacteristicNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristicNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for GattCharacteristicNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger;{e25f8fc8-0696-474f-a732-f292b0cebc5d})");
}
unsafe impl ::windows::core::Interface for GattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe25f8fc8_0696_474f_a732_f292b0cebc5d);
}
impl ::windows::core::RuntimeName for GattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger";
}
impl ::core::convert::From<GattCharacteristicNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: GattCharacteristicNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &GattCharacteristicNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattCharacteristicNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: GattCharacteristicNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &GattCharacteristicNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GattCharacteristicNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: GattCharacteristicNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GattCharacteristicNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &GattCharacteristicNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &GattCharacteristicNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTrigger {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct GattServiceProviderTrigger(::windows::core::IUnknown);
impl GattServiceProviderTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn SetAdvertisingParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_GenericAttributeProfile'*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn AdvertisingParameters(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(triggerid: Param0, serviceuuid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>> {
        Self::IGattServiceProviderTriggerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerid.into_param().abi(), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderTriggerStatics<R, F: FnOnce(&IGattServiceProviderTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceProviderTrigger, IGattServiceProviderTriggerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GattServiceProviderTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderTrigger {}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTrigger;{ddc6a3e9-1557-4bd8-8542-468aa0c696f6})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc6a3e9_1557_4bd8_8542_468aa0c696f6);
}
impl ::windows::core::RuntimeName for GattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTrigger";
}
impl ::core::convert::From<GattServiceProviderTrigger> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTrigger> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattServiceProviderTrigger> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTrigger> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GattServiceProviderTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: GattServiceProviderTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GattServiceProviderTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &GattServiceProviderTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &GattServiceProviderTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderTrigger {}
unsafe impl ::core::marker::Sync for GattServiceProviderTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct GattServiceProviderTriggerResult(::windows::core::IUnknown);
impl GattServiceProviderTriggerResult {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Trigger(&self) -> ::windows::core::Result<GattServiceProviderTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth'*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn Error(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Bluetooth::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::BluetoothError>(result__)
        }
    }
}
impl ::core::clone::Clone for GattServiceProviderTriggerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderTriggerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderTriggerResult {}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderTriggerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTriggerResult;{3c4691b1-b198-4e84-bad4-cf4ad299ed3a})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4691b1_b198_4e84_bad4_cf4ad299ed3a);
}
impl ::windows::core::RuntimeName for GattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTriggerResult";
}
impl ::core::convert::From<GattServiceProviderTriggerResult> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderTriggerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerResult> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderTriggerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderTriggerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GattServiceProviderTriggerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattServiceProviderTriggerResult> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderTriggerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerResult> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderTriggerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderTriggerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GattServiceProviderTriggerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderTriggerResult {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerResult {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct GeovisitTrigger(::windows::core::IUnknown);
impl GeovisitTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeovisitTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Geolocation'*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn MonitoringScope(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Geolocation::VisitMonitoringScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::VisitMonitoringScope>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Geolocation'*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for GeovisitTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitTrigger {}
unsafe impl ::windows::core::RuntimeType for GeovisitTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GeovisitTrigger;{4818edaa-04e1-4127-9a4c-19351b8a80a4})");
}
unsafe impl ::windows::core::Interface for GeovisitTrigger {
    type Vtable = IGeovisitTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4818edaa_04e1_4127_9a4c_19351b8a80a4);
}
impl ::windows::core::RuntimeName for GeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GeovisitTrigger";
}
impl ::core::convert::From<GeovisitTrigger> for ::windows::core::IUnknown {
    fn from(value: GeovisitTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitTrigger> for ::windows::core::IUnknown {
    fn from(value: &GeovisitTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeovisitTrigger> for ::windows::core::IInspectable {
    fn from(value: GeovisitTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitTrigger> for ::windows::core::IInspectable {
    fn from(value: &GeovisitTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GeovisitTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: GeovisitTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GeovisitTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &GeovisitTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &GeovisitTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GeovisitTrigger {}
unsafe impl ::core::marker::Sync for GeovisitTrigger {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorTrigger {
    type Vtable = IActivitySensorTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0dd4342_e37b_4823_a5fe_6b31dfefdeb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorTriggerFactory {
    type Vtable = IActivitySensorTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa72691c3_3837_44f7_831b_0132cc872bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportintervalinmilliseconds: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmApplicationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlarmApplicationManagerStatics {
    type Vtable = IAlarmApplicationManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca03fa3b_cce6_4de2_b09b_9628bd33bbbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmApplicationManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlarmAccessStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTrigger {
    type Vtable = IAppBroadcastTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d4f496_8d37_44ec_9481_2a0b9854eb48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerFactory {
    type Vtable = IAppBroadcastTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x280b9f44_22f4_4618_a02e_e7e411eb7238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerProviderInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf219352d_9de8_4420_9ce2_5eff8f17376b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerProviderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationTrigger {
    type Vtable = IApplicationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b468630_9574_492c_9e93_1a3ae6335fe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dc6ab2_2219_4a9e_9c5e_41d047f76e82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d4040c_c201_42ad_aa2a_e21ba3425b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundCondition(::windows::core::IUnknown);
impl IBackgroundCondition {}
impl ::core::convert::From<IBackgroundCondition> for ::windows::core::IInspectable {
    fn from(value: IBackgroundCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCondition> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCondition> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCondition> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCondition {}
unsafe impl ::windows::core::RuntimeType for IBackgroundCondition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ae48a1ee-8951-400a-8302-9c9c9a2a3a3b}");
}
unsafe impl ::windows::core::Interface for IBackgroundCondition {
    type Vtable = IBackgroundConditionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae48a1ee_8951_400a_8302_9c9c9a2a3a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundConditionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics {
    type Vtable = IBackgroundExecutionManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe826ea58_66a9_4d41_83d4_b4c18c87b846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics2 {
    type Vtable = IBackgroundExecutionManagerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x469b24ef_9bbb_4e18_999a_fd6512931be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics3 {
    type Vtable = IBackgroundExecutionManagerStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a5d3f6_5a25_5b6c_9192_d77a43dfedc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTask(::windows::core::IUnknown);
impl IBackgroundTask {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Run<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundTaskInstance>>(&self, taskinstance: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), taskinstance.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IBackgroundTask> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTask> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTask> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTask> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTask {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7d13d534-fd12-43ce-8c22-ea1ff13c06df}");
}
unsafe impl ::windows::core::Interface for IBackgroundTask {
    type Vtable = IBackgroundTaskVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d13d534_fd12_43ce_8c22_ea1ff13c06df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0351550e_3e64_4572_a93a_84075a37c917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trigger: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder2 {
    type Vtable = IBackgroundTaskBuilder2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ae7cfb1_104f_406d_8db6_844a570f42bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder3 {
    type Vtable = IBackgroundTaskBuilder3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28c74f4a_8ba9_4c09_a24f_19683e2c924c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder4 {
    type Vtable = IBackgroundTaskBuilder4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4755e522_cba2_4e35_bd16_a6da7f1c19aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder5 {
    type Vtable = IBackgroundTaskBuilder5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077103f6_99f5_4af4_bcad_4731d0330d43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskentrypoint: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x565d25cf_f209_48f4_9967_2b184f7bfbf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskCompletedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cc156d_af27_4dd3_846e_24ee40cadd25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance(::windows::core::IUnknown);
impl IBackgroundTaskInstance {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCanceledEventHandler>>(&self, cancelhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cancelhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTaskInstance> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskInstance> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTaskInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{865bda7a-21d8-4573-8f32-928a1b0641f6}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance {
    type Vtable = IBackgroundTaskInstanceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x865bda7a_21d8_4573_8f32_928a1b0641f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstanceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancelhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance2(::windows::core::IUnknown);
impl IBackgroundTaskInstance2 {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), counter, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCanceledEventHandler>>(&self, cancelhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cancelhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTaskInstance2> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskInstance2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance2> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskInstance2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskInstance2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskInstance2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskInstance2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBackgroundTaskInstance2> for IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: IBackgroundTaskInstance2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBackgroundTaskInstance2> for IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBackgroundTaskInstance2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskInstance> for IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskInstance> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskInstance> for &IBackgroundTaskInstance2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskInstance> {
        ::core::convert::TryInto::<IBackgroundTaskInstance>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBackgroundTaskInstance2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskInstance2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance2 {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskInstance2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4f7d0176-0c76-4fb4-896d-5de1864122f6}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance2 {
    type Vtable = IBackgroundTaskInstance2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f7d0176_0c76_4fb4_896d_5de1864122f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance4(::windows::core::IUnknown);
impl IBackgroundTaskInstance4 {
    #[doc = "*Required features: 'ApplicationModel_Background', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCanceledEventHandler>>(&self, cancelhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cancelhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskDeferral>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTaskInstance4> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskInstance4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance4> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskInstance4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskInstance4> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskInstance4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskInstance4> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskInstance4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBackgroundTaskInstance4> for IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: IBackgroundTaskInstance4) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBackgroundTaskInstance4> for IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBackgroundTaskInstance4) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskInstance> for IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskInstance> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskInstance> for &IBackgroundTaskInstance4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskInstance> {
        ::core::convert::TryInto::<IBackgroundTaskInstance>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBackgroundTaskInstance4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskInstance4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance4 {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskInstance4 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7f29f23c-aa04-4b08-97b0-06d874cdabf5}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance4 {
    type Vtable = IBackgroundTaskInstance4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f29f23c_aa04_4b08_97b0_06d874cdabf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskProgressEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb1468ac_8332_4d0a_9532_03eae684da31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskProgressEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration(::windows::core::IUnknown);
impl IBackgroundTaskRegistration {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskProgressEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCompletedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), canceltask).ok() }
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTaskRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10654cc2_a26e_43bf_8c12_1fb40dbfbfa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration2(::windows::core::IUnknown);
impl IBackgroundTaskRegistration2 {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskProgressEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCompletedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), canceltask).ok() }
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration2> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskRegistration2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration2> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskRegistration2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskRegistration2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskRegistration2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBackgroundTaskRegistration2> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: IBackgroundTaskRegistration2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBackgroundTaskRegistration2> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBackgroundTaskRegistration2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for &IBackgroundTaskRegistration2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::core::convert::TryInto::<IBackgroundTaskRegistration>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBackgroundTaskRegistration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration2 {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskRegistration2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6138c703-bb86-4112-afc3-7f939b166e3b}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration2 {
    type Vtable = IBackgroundTaskRegistration2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6138c703_bb86_4112_afc3_7f939b166e3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration3(::windows::core::IUnknown);
impl IBackgroundTaskRegistration3 {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTaskRegistrationGroup>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskProgressEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTaskCompletedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), canceltask).ok() }
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration3> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTaskRegistration3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration3> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTaskRegistration3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTaskRegistration3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTaskRegistration3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTaskRegistration3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTaskRegistration3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBackgroundTaskRegistration3> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: IBackgroundTaskRegistration3) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBackgroundTaskRegistration3> for IBackgroundTaskRegistration {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBackgroundTaskRegistration3) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTaskRegistration> for &IBackgroundTaskRegistration3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTaskRegistration> {
        ::core::convert::TryInto::<IBackgroundTaskRegistration>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBackgroundTaskRegistration3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration3 {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTaskRegistration3 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fe338195-9423-4d8b-830d-b1dd2c7badd5}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration3 {
    type Vtable = IBackgroundTaskRegistration3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe338195_9423_4d8b_830d_b1dd2c7badd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration3Vtbl(
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
pub struct IBackgroundTaskRegistrationGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ab1919a_871b_4167_8a76_055cd67b5b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationGroupFactory {
    type Vtable = IBackgroundTaskRegistrationGroupFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d92b69_44cf_4631_9740_03c7d8741bc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationStatics {
    type Vtable = IBackgroundTaskRegistrationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c542f69_b000_42ba_a093_6a563c65e3f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationStatics2 {
    type Vtable = IBackgroundTaskRegistrationStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x174b671e_b20d_4fa9_ad9a_e93ad6c71e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct IBackgroundTrigger(::windows::core::IUnknown);
impl IBackgroundTrigger {}
impl ::core::convert::From<IBackgroundTrigger> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTrigger> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTrigger> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTrigger> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTrigger {}
unsafe impl ::windows::core::RuntimeType for IBackgroundTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{84b3a058-6027-4b87-9790-bdf3f757dbd7}");
}
unsafe impl ::windows::core::Interface for IBackgroundTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundWorkCostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundWorkCostStatics {
    type Vtable = IBackgroundWorkCostStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc740a662_c310_4b82_b3e3_3bcfb9e4c77d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundWorkCostStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundWorkCostValue) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3e2612_25d3_48ae_8724_d81877ae6129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa28d064_38f4_597d_b597_4e55588c6503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2Vtbl(
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
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aab1819_bce1_48eb_a827_59fb7cee52a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Devices_Bluetooth")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
    #[cfg(feature = "Devices_Bluetooth")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b56799_eb39_5ab6_9932_aa9e4549604d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2Vtbl(
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
pub struct ICachedFileUpdaterTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe21caeeb_32f2_4d31_b553_b9e01bde37e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71838c13_1314_47b4_9597_dc7e248c17cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
    #[cfg(feature = "Storage_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x513b43bf_1d40_5c5d_78f5_c923fee3739e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReceivedNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ea3760e_baf5_4077_88e9_060cf6f0c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReceivedNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb91f28a_16a5_486d_974c_7835a8477be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppSetAsActiveTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc833419b_4705_4571_9a16_06b997bf9c96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetchTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetchTrigger {
    type Vtable = IContentPrefetchTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x710627ee_04fa_440b_80c0_173202199e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetchTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetchTriggerFactory {
    type Vtable = IContentPrefetchTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2643eda_8a03_409e_b8c4_88814c28ccb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitinterval: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSystemEventTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3596798_cf6b_4ef4_a0ca_29cf4a278c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSystemEventTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSystemEventTriggerFactory {
    type Vtable = ICustomSystemEventTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bcb16c5_f2dc_41b2_9efd_b96bdcd13ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90875e64_3cdd_4efb_ab1c_5b3b6a60ce34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceConnectionChangeTriggerStatics {
    type Vtable = IDeviceConnectionChangeTriggerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3ea246a_4efd_4498_aa60_a4e4e3b17ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IDeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81278ab5_41ab_16da_86c2_7f7bf0912f5b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTriggerFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IDeviceManufacturerNotificationTriggerFactory {
    type Vtable = IDeviceManufacturerNotificationTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7955de75_25bb_4153_a1a2_3029fcabb652);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerqualifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceServicingTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceServicingTrigger {
    type Vtable = IDeviceServicingTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab217ad_6e34_49d3_9e6f_17f1b6dfa881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUseTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceUseTrigger {
    type Vtable = IDeviceUseTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0da68011_334f_4d57_b6ec_6dca64b412e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4617fdd_8573_4260_befc_5bec89cb693d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d06da_47eb_4268_a4f2_f3f77188388a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe25f8fc8_0696_474f_a732_f292b0cebc5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTrigger2 {
    type Vtable = IGattCharacteristicNotificationTrigger2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9322a2c4_ae0e_42f2_b28c_f51372e69245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerFactory {
    type Vtable = IGattCharacteristicNotificationTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57ba1995_b143_4575_9f6b_fd59d93ace1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerFactory2 {
    type Vtable = IGattCharacteristicNotificationTriggerFactory2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5998e91f_8a53_4e9f_a32c_23cd33664cee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc6a3e9_1557_4bd8_8542_468aa0c696f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4691b1_b198_4e84_bad4_cf4ad299ed3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerStatics {
    type Vtable = IGattServiceProviderTriggerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb413a36a_e294_4591_a5a6_64891a828153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeovisitTrigger {
    type Vtable = IGeovisitTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4818edaa_04e1_4127_9a4c_19351b8a80a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocationTrigger {
    type Vtable = ILocationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47666a1c_6877_481e_8026_ff7e14a811a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocationTriggerType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocationTriggerFactory {
    type Vtable = ILocationTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1106bb07_ff69_4e09_aa8b_1384ea475e98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: LocationTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMaintenanceTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMaintenanceTrigger {
    type Vtable = IMaintenanceTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68184c83_fc22_4ce5_841a_7239a9810047);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMaintenanceTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMaintenanceTriggerFactory {
    type Vtable = IMaintenanceTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3ddb2e_97dd_4629_88b0_b06cf9482ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProcessingTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaProcessingTrigger {
    type Vtable = IMediaProcessingTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a95be65_8a52_4b30_9011_cf38040ea8b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_3001_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorHotspotAuthenticationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90089cc6_63cd_480c_95d1_6e6aef801e4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerVtbl(
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
pub struct INetworkOperatorNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorNotificationTriggerFactory {
    type Vtable = INetworkOperatorNotificationTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a223e00_27d7_4353_adb9_9265aaea579d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneTrigger {
    type Vtable = IPhoneTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dcfe99b_d4c5_49f1_b7d3_82e87a0e9dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneTriggerFactory {
    type Vtable = IPhoneTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0d93cda_5fc1_48fb_a546_32262040157b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationTriggerFactory {
    type Vtable = IPushNotificationTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dd8ed1b_458e_4fc2_bc2e_d5664f77ed19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d0d6a_b2f6_467f_a978_a44091c11a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommConnectionTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c4cae2_0b53_4464_9394_fd875654de64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf237f327_5181_4f24_96a7_700a4e5fac62);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc0f372_d48b_4b7f_abec_15f9bacc12e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTriggerFactory {
    type Vtable = ISensorDataThresholdTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921fe675_7df0_4da3_97b3_e544ee857fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sensors")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmartCardTrigger {
    type Vtable = ISmartCardTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53bc5ac_84ca_4972_8ce9_e58f97b37a50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_SmartCards")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmartCardTriggerFactory {
    type Vtable = ISmartCardTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bf54c3_89c1_4e00_a9d3_97c629269dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_SmartCards")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsMessageReceivedTriggerFactory {
    type Vtable = ISmsMessageReceivedTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3ad8c8_6ba4_4ab2_8d21_bc6b09c77564);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sms")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterrules: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityTrigger {
    type Vtable = ISocketActivityTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9bbf810_9dde_4f8a_83e3_b0e0e7a50d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTrackerTriggerFactory {
    type Vtable = IStorageLibraryChangeTrackerTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eb0ffd0_5a85_499e_a888_824607124f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracker: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1637e0a7_829c_45bc_929b_a1e7ea78d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryContentChangedTriggerStatics {
    type Vtable = IStorageLibraryContentChangedTriggerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f9f1b39_5f90_4e12_914e_a7d8e0bbfb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibrary: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibraries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemCondition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemCondition {
    type Vtable = ISystemConditionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc15fb476_89c5_420b_abd3_fb3030472128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemConditionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemConditionType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemConditionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemConditionFactory {
    type Vtable = ISystemConditionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269d1f1_05a7_49ae_87d7_16b2b8b9a553);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemConditionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditiontype: SystemConditionType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemTrigger {
    type Vtable = ISystemTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d80c776_3748_4463_8d7e_276dc139ac1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemTriggerType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemTriggerFactory {
    type Vtable = ISystemTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe80423d4_8791_4579_8126_87ec8aaa407a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: SystemTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeTrigger {
    type Vtable = ITimeTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x656e5556_0b2a_4377_ba70_3b45a935547f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeTriggerFactory {
    type Vtable = ITimeTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38c682fe_9b54_45e6_b2f3_269b87a6f734);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationActionTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationActionTriggerFactory {
    type Vtable = IToastNotificationActionTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb09dfc27_6480_4349_8125_97b3efaa0a3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerFactory {
    type Vtable = IToastNotificationHistoryChangedTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c6faad_8797_4785_81b4_b0cccb73d1d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationChangedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserNotificationChangedTriggerFactory {
    type Vtable = IUserNotificationChangedTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcad4436c_69ab_4e18_a48a_5ed2ac435957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct LocationTrigger(::windows::core::IUnknown);
impl LocationTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerType(&self) -> ::windows::core::Result<LocationTriggerType> {
        let this = self;
        unsafe {
            let mut result__: LocationTriggerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LocationTriggerType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(triggertype: LocationTriggerType) -> ::windows::core::Result<LocationTrigger> {
        Self::ILocationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggertype, &mut result__).from_abi::<LocationTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocationTriggerFactory<R, F: FnOnce(&ILocationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocationTrigger, ILocationTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LocationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocationTrigger {}
unsafe impl ::windows::core::RuntimeType for LocationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.LocationTrigger;{47666a1c-6877-481e-8026-ff7e14a811a0})");
}
unsafe impl ::windows::core::Interface for LocationTrigger {
    type Vtable = ILocationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47666a1c_6877_481e_8026_ff7e14a811a0);
}
impl ::windows::core::RuntimeName for LocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.LocationTrigger";
}
impl ::core::convert::From<LocationTrigger> for ::windows::core::IUnknown {
    fn from(value: LocationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocationTrigger> for ::windows::core::IUnknown {
    fn from(value: &LocationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocationTrigger> for ::windows::core::IInspectable {
    fn from(value: LocationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocationTrigger> for ::windows::core::IInspectable {
    fn from(value: &LocationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LocationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: LocationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LocationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &LocationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &LocationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LocationTrigger {}
unsafe impl ::core::marker::Sync for LocationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: Self = Self(0i32);
}
impl ::core::marker::Copy for LocationTriggerType {}
impl ::core::clone::Clone for LocationTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LocationTriggerType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LocationTriggerType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocationTriggerType {}
unsafe impl ::windows::core::RuntimeType for LocationTriggerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.LocationTriggerType;i4)");
}
impl ::windows::core::DefaultType for LocationTriggerType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MaintenanceTrigger(::windows::core::IUnknown);
impl MaintenanceTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn FreshnessTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<MaintenanceTrigger> {
        Self::IMaintenanceTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), freshnesstime, oneshot, &mut result__).from_abi::<MaintenanceTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMaintenanceTriggerFactory<R, F: FnOnce(&IMaintenanceTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MaintenanceTrigger, IMaintenanceTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MaintenanceTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MaintenanceTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MaintenanceTrigger {}
unsafe impl ::windows::core::RuntimeType for MaintenanceTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MaintenanceTrigger;{68184c83-fc22-4ce5-841a-7239a9810047})");
}
unsafe impl ::windows::core::Interface for MaintenanceTrigger {
    type Vtable = IMaintenanceTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68184c83_fc22_4ce5_841a_7239a9810047);
}
impl ::windows::core::RuntimeName for MaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MaintenanceTrigger";
}
impl ::core::convert::From<MaintenanceTrigger> for ::windows::core::IUnknown {
    fn from(value: MaintenanceTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MaintenanceTrigger> for ::windows::core::IUnknown {
    fn from(value: &MaintenanceTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MaintenanceTrigger> for ::windows::core::IInspectable {
    fn from(value: MaintenanceTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MaintenanceTrigger> for ::windows::core::IInspectable {
    fn from(value: &MaintenanceTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MaintenanceTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MaintenanceTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MaintenanceTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MaintenanceTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MaintenanceTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MediaProcessingTrigger(::windows::core::IUnknown);
impl MediaProcessingTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaProcessingTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAsyncWithArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, arguments: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaProcessingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProcessingTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProcessingTrigger {}
unsafe impl ::windows::core::RuntimeType for MediaProcessingTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MediaProcessingTrigger;{9a95be65-8a52-4b30-9011-cf38040ea8b0})");
}
unsafe impl ::windows::core::Interface for MediaProcessingTrigger {
    type Vtable = IMediaProcessingTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a95be65_8a52_4b30_9011_cf38040ea8b0);
}
impl ::windows::core::RuntimeName for MediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MediaProcessingTrigger";
}
impl ::core::convert::From<MediaProcessingTrigger> for ::windows::core::IUnknown {
    fn from(value: MediaProcessingTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTrigger> for ::windows::core::IUnknown {
    fn from(value: &MediaProcessingTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProcessingTrigger> for ::windows::core::IInspectable {
    fn from(value: MediaProcessingTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTrigger> for ::windows::core::IInspectable {
    fn from(value: &MediaProcessingTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaProcessingTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaProcessingTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaProcessingTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaProcessingTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MediaProcessingTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaProcessingTriggerResult {}
impl ::core::clone::Clone for MediaProcessingTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaProcessingTriggerResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaProcessingTriggerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProcessingTriggerResult {}
unsafe impl ::windows::core::RuntimeType for MediaProcessingTriggerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.MediaProcessingTriggerResult;i4)");
}
impl ::windows::core::DefaultType for MediaProcessingTriggerResult {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandDeviceServiceNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceNotificationTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MobileBroadbandDeviceServiceNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MobileBroadbandDeviceServiceNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MobileBroadbandDeviceServiceNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MobileBroadbandDeviceServiceNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MobileBroadbandDeviceServiceNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandPcoDataChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandPcoDataChangeTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandPcoDataChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPcoDataChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPcoDataChangeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for MobileBroadbandPcoDataChangeTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for MobileBroadbandPcoDataChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger";
}
impl ::core::convert::From<MobileBroadbandPcoDataChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPcoDataChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPcoDataChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPcoDataChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPcoDataChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPcoDataChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPcoDataChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPcoDataChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MobileBroadbandPcoDataChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MobileBroadbandPcoDataChangeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MobileBroadbandPcoDataChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MobileBroadbandPcoDataChangeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MobileBroadbandPcoDataChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPcoDataChangeTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandPinLockStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandPinLockStateChangeTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinLockStateChangeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinLockStateChangeTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for MobileBroadbandPinLockStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger";
}
impl ::core::convert::From<MobileBroadbandPinLockStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPinLockStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPinLockStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPinLockStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPinLockStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPinLockStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MobileBroadbandPinLockStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MobileBroadbandPinLockStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MobileBroadbandPinLockStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MobileBroadbandPinLockStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MobileBroadbandPinLockStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChangeTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandRadioStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandRadioStateChangeTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandRadioStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandRadioStateChangeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for MobileBroadbandRadioStateChangeTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for MobileBroadbandRadioStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger";
}
impl ::core::convert::From<MobileBroadbandRadioStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandRadioStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandRadioStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandRadioStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandRadioStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandRadioStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MobileBroadbandRadioStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MobileBroadbandRadioStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MobileBroadbandRadioStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MobileBroadbandRadioStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MobileBroadbandRadioStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChangeTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandRegistrationStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandRegistrationStateChangeTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandRegistrationStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRegistrationStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandRegistrationStateChangeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for MobileBroadbandRegistrationStateChangeTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for MobileBroadbandRegistrationStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger";
}
impl ::core::convert::From<MobileBroadbandRegistrationStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRegistrationStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandRegistrationStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRegistrationStateChangeTrigger> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MobileBroadbandRegistrationStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: MobileBroadbandRegistrationStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MobileBroadbandRegistrationStateChangeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &MobileBroadbandRegistrationStateChangeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &MobileBroadbandRegistrationStateChangeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRegistrationStateChangeTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTrigger(::windows::core::IUnknown);
impl NetworkOperatorDataUsageTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorDataUsageTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NetworkOperatorDataUsageTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorDataUsageTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorDataUsageTrigger {}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorDataUsageTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for NetworkOperatorDataUsageTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for NetworkOperatorDataUsageTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger";
}
impl ::core::convert::From<NetworkOperatorDataUsageTrigger> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorDataUsageTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorDataUsageTrigger> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorDataUsageTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorDataUsageTrigger> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorDataUsageTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorDataUsageTrigger> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorDataUsageTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<NetworkOperatorDataUsageTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: NetworkOperatorDataUsageTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&NetworkOperatorDataUsageTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &NetworkOperatorDataUsageTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &NetworkOperatorDataUsageTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for NetworkOperatorDataUsageTrigger {}
unsafe impl ::core::marker::Sync for NetworkOperatorDataUsageTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(::windows::core::IUnknown);
impl NetworkOperatorHotspotAuthenticationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorHotspotAuthenticationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NetworkOperatorHotspotAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorHotspotAuthenticationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorHotspotAuthenticationTrigger {}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger;{e756c791-3001-4de5-83c7-de61d88831d0})");
}
unsafe impl ::windows::core::Interface for NetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_3001_4de5_83c7_de61d88831d0);
}
impl ::windows::core::RuntimeName for NetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger";
}
impl ::core::convert::From<NetworkOperatorHotspotAuthenticationTrigger> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorHotspotAuthenticationTrigger> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorHotspotAuthenticationTrigger> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorHotspotAuthenticationTrigger> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorHotspotAuthenticationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<NetworkOperatorHotspotAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: NetworkOperatorHotspotAuthenticationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&NetworkOperatorHotspotAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &NetworkOperatorHotspotAuthenticationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &NetworkOperatorHotspotAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct NetworkOperatorNotificationTrigger(::windows::core::IUnknown);
impl NetworkOperatorNotificationTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<NetworkOperatorNotificationTrigger> {
        Self::INetworkOperatorNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<NetworkOperatorNotificationTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorNotificationTriggerFactory<R, F: FnOnce(&INetworkOperatorNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorNotificationTrigger, INetworkOperatorNotificationTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NetworkOperatorNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger;{90089cc6-63cd-480c-95d1-6e6aef801e4a})");
}
unsafe impl ::windows::core::Interface for NetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90089cc6_63cd_480c_95d1_6e6aef801e4a);
}
impl ::windows::core::RuntimeName for NetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger";
}
impl ::core::convert::From<NetworkOperatorNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<NetworkOperatorNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: NetworkOperatorNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&NetworkOperatorNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &NetworkOperatorNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &NetworkOperatorNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTrigger(::windows::core::IUnknown);
impl PaymentAppCanMakePaymentTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaymentAppCanMakePaymentTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppCanMakePaymentTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::windows::core::RuntimeType for PaymentAppCanMakePaymentTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for PaymentAppCanMakePaymentTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for PaymentAppCanMakePaymentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger";
}
impl ::core::convert::From<PaymentAppCanMakePaymentTrigger> for ::windows::core::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTrigger> for ::windows::core::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAppCanMakePaymentTrigger> for ::windows::core::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTrigger> for ::windows::core::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PaymentAppCanMakePaymentTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: PaymentAppCanMakePaymentTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PaymentAppCanMakePaymentTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &PaymentAppCanMakePaymentTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &PaymentAppCanMakePaymentTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct PhoneTrigger(::windows::core::IUnknown);
impl PhoneTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'ApplicationModel_Calls_Background'*"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn TriggerType(&self) -> ::windows::core::Result<super::Calls::Background::PhoneTriggerType> {
        let this = self;
        unsafe {
            let mut result__: super::Calls::Background::PhoneTriggerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::Background::PhoneTriggerType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'ApplicationModel_Calls_Background'*"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn Create(r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows::core::Result<PhoneTrigger> {
        Self::IPhoneTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, oneshot, &mut result__).from_abi::<PhoneTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneTriggerFactory<R, F: FnOnce(&IPhoneTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneTrigger, IPhoneTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneTrigger {}
unsafe impl ::windows::core::RuntimeType for PhoneTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PhoneTrigger;{8dcfe99b-d4c5-49f1-b7d3-82e87a0e9dde})");
}
unsafe impl ::windows::core::Interface for PhoneTrigger {
    type Vtable = IPhoneTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dcfe99b_d4c5_49f1_b7d3_82e87a0e9dde);
}
impl ::windows::core::RuntimeName for PhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PhoneTrigger";
}
impl ::core::convert::From<PhoneTrigger> for ::windows::core::IUnknown {
    fn from(value: PhoneTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneTrigger> for ::windows::core::IUnknown {
    fn from(value: &PhoneTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneTrigger> for ::windows::core::IInspectable {
    fn from(value: PhoneTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneTrigger> for ::windows::core::IInspectable {
    fn from(value: &PhoneTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhoneTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &PhoneTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhoneTrigger {}
unsafe impl ::core::marker::Sync for PhoneTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct PushNotificationTrigger(::windows::core::IUnknown);
impl PushNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<PushNotificationTrigger> {
        Self::IPushNotificationTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<PushNotificationTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationTriggerFactory<R, F: FnOnce(&IPushNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationTrigger, IPushNotificationTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PushNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationTrigger {}
unsafe impl ::windows::core::RuntimeType for PushNotificationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PushNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for PushNotificationTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for PushNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PushNotificationTrigger";
}
impl ::core::convert::From<PushNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: PushNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationTrigger> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: PushNotificationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationTrigger> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PushNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: PushNotificationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PushNotificationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &PushNotificationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &PushNotificationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PushNotificationTrigger {}
unsafe impl ::core::marker::Sync for PushNotificationTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTrigger(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RcsEndUserMessageAvailableTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger;{986d0d6a-b2f6-467f-a978-a44091c11a66})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d0d6a_b2f6_467f_a978_a44091c11a66);
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger";
}
impl ::core::convert::From<RcsEndUserMessageAvailableTrigger> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessageAvailableTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableTrigger> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessageAvailableTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessageAvailableTrigger> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessageAvailableTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableTrigger> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessageAvailableTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RcsEndUserMessageAvailableTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: RcsEndUserMessageAvailableTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RcsEndUserMessageAvailableTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &RcsEndUserMessageAvailableTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &RcsEndUserMessageAvailableTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct RfcommConnectionTrigger(::windows::core::IUnknown);
impl RfcommConnectionTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RfcommConnectionTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Background'*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn InboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Bluetooth_Background'*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn OutboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn AllowMultipleConnections(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn SetAllowMultipleConnections(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Networking_Sockets'*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Networking::Sockets::SocketProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Networking_Sockets'*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Networking'*"]
    #[cfg(feature = "Networking")]
    pub fn RemoteHostName(&self) -> ::windows::core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Networking'*"]
    #[cfg(feature = "Networking")]
    pub fn SetRemoteHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for RfcommConnectionTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommConnectionTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommConnectionTrigger {}
unsafe impl ::windows::core::RuntimeType for RfcommConnectionTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RfcommConnectionTrigger;{e8c4cae2-0b53-4464-9394-fd875654de64})");
}
unsafe impl ::windows::core::Interface for RfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c4cae2_0b53_4464_9394_fd875654de64);
}
impl ::windows::core::RuntimeName for RfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RfcommConnectionTrigger";
}
impl ::core::convert::From<RfcommConnectionTrigger> for ::windows::core::IUnknown {
    fn from(value: RfcommConnectionTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommConnectionTrigger> for ::windows::core::IUnknown {
    fn from(value: &RfcommConnectionTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommConnectionTrigger> for ::windows::core::IInspectable {
    fn from(value: RfcommConnectionTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommConnectionTrigger> for ::windows::core::IInspectable {
    fn from(value: &RfcommConnectionTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RfcommConnectionTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: RfcommConnectionTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RfcommConnectionTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &RfcommConnectionTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &RfcommConnectionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RfcommConnectionTrigger {}
unsafe impl ::core::marker::Sync for RfcommConnectionTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorAuthenticationTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationTrigger {}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger;{f237f327-5181-4f24-96a7-700a4e5fac62})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf237f327_5181_4f24_96a7_700a4e5fac62);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationTrigger> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationTrigger> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationTrigger> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationTrigger> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<SecondaryAuthenticationFactorAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SecondaryAuthenticationFactorAuthenticationTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SecondaryAuthenticationFactorAuthenticationTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SecondaryAuthenticationFactorAuthenticationTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SecondaryAuthenticationFactorAuthenticationTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SensorDataThresholdTrigger(::windows::core::IUnknown);
impl SensorDataThresholdTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Sensors'*"]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Sensors::ISensorDataThreshold>>(threshold: Param0) -> ::windows::core::Result<SensorDataThresholdTrigger> {
        Self::ISensorDataThresholdTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), threshold.into_param().abi(), &mut result__).from_abi::<SensorDataThresholdTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISensorDataThresholdTriggerFactory<R, F: FnOnce(&ISensorDataThresholdTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SensorDataThresholdTrigger, ISensorDataThresholdTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SensorDataThresholdTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SensorDataThresholdTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorDataThresholdTrigger {}
unsafe impl ::windows::core::RuntimeType for SensorDataThresholdTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SensorDataThresholdTrigger;{5bc0f372-d48b-4b7f-abec-15f9bacc12e2})");
}
unsafe impl ::windows::core::Interface for SensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc0f372_d48b_4b7f_abec_15f9bacc12e2);
}
impl ::windows::core::RuntimeName for SensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SensorDataThresholdTrigger";
}
impl ::core::convert::From<SensorDataThresholdTrigger> for ::windows::core::IUnknown {
    fn from(value: SensorDataThresholdTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorDataThresholdTrigger> for ::windows::core::IUnknown {
    fn from(value: &SensorDataThresholdTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SensorDataThresholdTrigger> for ::windows::core::IInspectable {
    fn from(value: SensorDataThresholdTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorDataThresholdTrigger> for ::windows::core::IInspectable {
    fn from(value: &SensorDataThresholdTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SensorDataThresholdTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SensorDataThresholdTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SensorDataThresholdTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SensorDataThresholdTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SensorDataThresholdTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SensorDataThresholdTrigger {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SmartCardTrigger(::windows::core::IUnknown);
impl SmartCardTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_SmartCards'*"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn TriggerType(&self) -> ::windows::core::Result<super::super::Devices::SmartCards::SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::SmartCards::SmartCardTriggerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::SmartCards::SmartCardTriggerType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_SmartCards'*"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn Create(triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::Result<SmartCardTrigger> {
        Self::ISmartCardTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggertype, &mut result__).from_abi::<SmartCardTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardTriggerFactory<R, F: FnOnce(&ISmartCardTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SmartCardTrigger, ISmartCardTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardTrigger {}
unsafe impl ::windows::core::RuntimeType for SmartCardTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmartCardTrigger;{f53bc5ac-84ca-4972-8ce9-e58f97b37a50})");
}
unsafe impl ::windows::core::Interface for SmartCardTrigger {
    type Vtable = ISmartCardTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53bc5ac_84ca_4972_8ce9_e58f97b37a50);
}
impl ::windows::core::RuntimeName for SmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmartCardTrigger";
}
impl ::core::convert::From<SmartCardTrigger> for ::windows::core::IUnknown {
    fn from(value: SmartCardTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardTrigger> for ::windows::core::IUnknown {
    fn from(value: &SmartCardTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardTrigger> for ::windows::core::IInspectable {
    fn from(value: SmartCardTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardTrigger> for ::windows::core::IInspectable {
    fn from(value: &SmartCardTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmartCardTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SmartCardTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmartCardTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SmartCardTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SmartCardTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SmsMessageReceivedTrigger(::windows::core::IUnknown);
impl SmsMessageReceivedTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Devices_Sms'*"]
    #[cfg(feature = "Devices_Sms")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Sms::SmsFilterRules>>(filterrules: Param0) -> ::windows::core::Result<SmsMessageReceivedTrigger> {
        Self::ISmsMessageReceivedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filterrules.into_param().abi(), &mut result__).from_abi::<SmsMessageReceivedTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsMessageReceivedTriggerFactory<R, F: FnOnce(&ISmsMessageReceivedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SmsMessageReceivedTrigger, ISmsMessageReceivedTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmsMessageReceivedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsMessageReceivedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageReceivedTrigger {}
unsafe impl ::windows::core::RuntimeType for SmsMessageReceivedTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmsMessageReceivedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for SmsMessageReceivedTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for SmsMessageReceivedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmsMessageReceivedTrigger";
}
impl ::core::convert::From<SmsMessageReceivedTrigger> for ::windows::core::IUnknown {
    fn from(value: SmsMessageReceivedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageReceivedTrigger> for ::windows::core::IUnknown {
    fn from(value: &SmsMessageReceivedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsMessageReceivedTrigger> for ::windows::core::IInspectable {
    fn from(value: SmsMessageReceivedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageReceivedTrigger> for ::windows::core::IInspectable {
    fn from(value: &SmsMessageReceivedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsMessageReceivedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SmsMessageReceivedTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsMessageReceivedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SmsMessageReceivedTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SmsMessageReceivedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsMessageReceivedTrigger {}
unsafe impl ::core::marker::Sync for SmsMessageReceivedTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SocketActivityTrigger(::windows::core::IUnknown);
impl SocketActivityTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocketActivityTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISocketActivityTrigger>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SocketActivityTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SocketActivityTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityTrigger {}
unsafe impl ::windows::core::RuntimeType for SocketActivityTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SocketActivityTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for SocketActivityTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for SocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SocketActivityTrigger";
}
impl ::core::convert::From<SocketActivityTrigger> for ::windows::core::IUnknown {
    fn from(value: SocketActivityTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityTrigger> for ::windows::core::IUnknown {
    fn from(value: &SocketActivityTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SocketActivityTrigger> for ::windows::core::IInspectable {
    fn from(value: SocketActivityTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityTrigger> for ::windows::core::IInspectable {
    fn from(value: &SocketActivityTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SocketActivityTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SocketActivityTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SocketActivityTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SocketActivityTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SocketActivityTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SocketActivityTrigger {}
unsafe impl ::core::marker::Sync for SocketActivityTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTrigger(::windows::core::IUnknown);
impl StorageLibraryChangeTrackerTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageLibraryChangeTracker>>(tracker: Param0) -> ::windows::core::Result<StorageLibraryChangeTrackerTrigger> {
        Self::IStorageLibraryChangeTrackerTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tracker.into_param().abi(), &mut result__).from_abi::<StorageLibraryChangeTrackerTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryChangeTrackerTriggerFactory<R, F: FnOnce(&IStorageLibraryChangeTrackerTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryChangeTrackerTrigger, IStorageLibraryChangeTrackerTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTrackerTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTrackerTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTrackerTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTrackerTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger";
}
impl ::core::convert::From<StorageLibraryChangeTrackerTrigger> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTrackerTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTrigger> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeTrackerTrigger> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTrackerTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTrigger> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTrackerTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageLibraryChangeTrackerTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageLibraryChangeTrackerTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageLibraryChangeTrackerTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageLibraryChangeTrackerTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &StorageLibraryChangeTrackerTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTrackerTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct StorageLibraryContentChangedTrigger(::windows::core::IUnknown);
impl StorageLibraryContentChangedTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageLibrary>>(storagelibrary: Param0) -> ::windows::core::Result<StorageLibraryContentChangedTrigger> {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), storagelibrary.into_param().abi(), &mut result__).from_abi::<StorageLibraryContentChangedTrigger>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Background', 'Foundation_Collections', 'Storage'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn CreateFromLibraries<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>>(storagelibraries: Param0) -> ::windows::core::Result<StorageLibraryContentChangedTrigger> {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), storagelibraries.into_param().abi(), &mut result__).from_abi::<StorageLibraryContentChangedTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryContentChangedTriggerStatics<R, F: FnOnce(&IStorageLibraryContentChangedTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryContentChangedTrigger, IStorageLibraryContentChangedTriggerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageLibraryContentChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryContentChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryContentChangedTrigger {}
unsafe impl ::windows::core::RuntimeType for StorageLibraryContentChangedTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger;{1637e0a7-829c-45bc-929b-a1e7ea78d89b})");
}
unsafe impl ::windows::core::Interface for StorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1637e0a7_829c_45bc_929b_a1e7ea78d89b);
}
impl ::windows::core::RuntimeName for StorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger";
}
impl ::core::convert::From<StorageLibraryContentChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryContentChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryContentChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryContentChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryContentChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryContentChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageLibraryContentChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageLibraryContentChangedTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageLibraryContentChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageLibraryContentChangedTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &StorageLibraryContentChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SystemCondition(::windows::core::IUnknown);
impl SystemCondition {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn ConditionType(&self) -> ::windows::core::Result<SystemConditionType> {
        let this = self;
        unsafe {
            let mut result__: SystemConditionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemConditionType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(conditiontype: SystemConditionType) -> ::windows::core::Result<SystemCondition> {
        Self::ISystemConditionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), conditiontype, &mut result__).from_abi::<SystemCondition>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemConditionFactory<R, F: FnOnce(&ISystemConditionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemCondition, ISystemConditionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCondition {}
unsafe impl ::windows::core::RuntimeType for SystemCondition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemCondition;{c15fb476-89c5-420b-abd3-fb3030472128})");
}
unsafe impl ::windows::core::Interface for SystemCondition {
    type Vtable = ISystemConditionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc15fb476_89c5_420b_abd3_fb3030472128);
}
impl ::windows::core::RuntimeName for SystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemCondition";
}
impl ::core::convert::From<SystemCondition> for ::windows::core::IUnknown {
    fn from(value: SystemCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCondition> for ::windows::core::IUnknown {
    fn from(value: &SystemCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemCondition> for ::windows::core::IInspectable {
    fn from(value: SystemCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCondition> for ::windows::core::IInspectable {
    fn from(value: &SystemCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SystemCondition> for IBackgroundCondition {
    type Error = ::windows::core::Error;
    fn try_from(value: SystemCondition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SystemCondition> for IBackgroundCondition {
    type Error = ::windows::core::Error;
    fn try_from(value: &SystemCondition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCondition> for SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCondition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCondition> for &SystemCondition {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCondition> {
        ::core::convert::TryInto::<IBackgroundCondition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: Self = Self(0i32);
    pub const UserPresent: Self = Self(1i32);
    pub const UserNotPresent: Self = Self(2i32);
    pub const InternetAvailable: Self = Self(3i32);
    pub const InternetNotAvailable: Self = Self(4i32);
    pub const SessionConnected: Self = Self(5i32);
    pub const SessionDisconnected: Self = Self(6i32);
    pub const FreeNetworkAvailable: Self = Self(7i32);
    pub const BackgroundWorkCostNotHigh: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemConditionType {}
impl ::core::clone::Clone for SystemConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SystemConditionType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SystemConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemConditionType {}
unsafe impl ::windows::core::RuntimeType for SystemConditionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemConditionType;i4)");
}
impl ::windows::core::DefaultType for SystemConditionType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SystemTrigger(::windows::core::IUnknown);
impl SystemTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn TriggerType(&self) -> ::windows::core::Result<SystemTriggerType> {
        let this = self;
        unsafe {
            let mut result__: SystemTriggerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemTriggerType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(triggertype: SystemTriggerType, oneshot: bool) -> ::windows::core::Result<SystemTrigger> {
        Self::ISystemTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggertype, oneshot, &mut result__).from_abi::<SystemTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemTriggerFactory<R, F: FnOnce(&ISystemTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemTrigger, ISystemTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemTrigger {}
unsafe impl ::windows::core::RuntimeType for SystemTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemTrigger;{1d80c776-3748-4463-8d7e-276dc139ac1c})");
}
unsafe impl ::windows::core::Interface for SystemTrigger {
    type Vtable = ISystemTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d80c776_3748_4463_8d7e_276dc139ac1c);
}
impl ::windows::core::RuntimeName for SystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemTrigger";
}
impl ::core::convert::From<SystemTrigger> for ::windows::core::IUnknown {
    fn from(value: SystemTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemTrigger> for ::windows::core::IUnknown {
    fn from(value: &SystemTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemTrigger> for ::windows::core::IInspectable {
    fn from(value: SystemTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemTrigger> for ::windows::core::IInspectable {
    fn from(value: &SystemTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SystemTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: SystemTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SystemTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &SystemTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &SystemTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: Self = Self(0i32);
    pub const SmsReceived: Self = Self(1i32);
    pub const UserPresent: Self = Self(2i32);
    pub const UserAway: Self = Self(3i32);
    pub const NetworkStateChange: Self = Self(4i32);
    pub const ControlChannelReset: Self = Self(5i32);
    pub const InternetAvailable: Self = Self(6i32);
    pub const SessionConnected: Self = Self(7i32);
    pub const ServicingComplete: Self = Self(8i32);
    pub const LockScreenApplicationAdded: Self = Self(9i32);
    pub const LockScreenApplicationRemoved: Self = Self(10i32);
    pub const TimeZoneChange: Self = Self(11i32);
    pub const OnlineIdConnectedStateChange: Self = Self(12i32);
    pub const BackgroundWorkCostChange: Self = Self(13i32);
    pub const PowerStateChange: Self = Self(14i32);
    pub const DefaultSignInAccountChange: Self = Self(15i32);
}
impl ::core::marker::Copy for SystemTriggerType {}
impl ::core::clone::Clone for SystemTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SystemTriggerType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SystemTriggerType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemTriggerType {}
unsafe impl ::windows::core::RuntimeType for SystemTriggerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemTriggerType;i4)");
}
impl ::windows::core::DefaultType for SystemTriggerType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct TetheringEntitlementCheckTrigger(::windows::core::IUnknown);
impl TetheringEntitlementCheckTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TetheringEntitlementCheckTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TetheringEntitlementCheckTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TetheringEntitlementCheckTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TetheringEntitlementCheckTrigger {}
unsafe impl ::windows::core::RuntimeType for TetheringEntitlementCheckTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for TetheringEntitlementCheckTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for TetheringEntitlementCheckTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger";
}
impl ::core::convert::From<TetheringEntitlementCheckTrigger> for ::windows::core::IUnknown {
    fn from(value: TetheringEntitlementCheckTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TetheringEntitlementCheckTrigger> for ::windows::core::IUnknown {
    fn from(value: &TetheringEntitlementCheckTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TetheringEntitlementCheckTrigger> for ::windows::core::IInspectable {
    fn from(value: TetheringEntitlementCheckTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TetheringEntitlementCheckTrigger> for ::windows::core::IInspectable {
    fn from(value: &TetheringEntitlementCheckTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TetheringEntitlementCheckTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: TetheringEntitlementCheckTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TetheringEntitlementCheckTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &TetheringEntitlementCheckTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &TetheringEntitlementCheckTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TetheringEntitlementCheckTrigger {}
unsafe impl ::core::marker::Sync for TetheringEntitlementCheckTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct TimeTrigger(::windows::core::IUnknown);
impl TimeTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn FreshnessTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<TimeTrigger> {
        Self::ITimeTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), freshnesstime, oneshot, &mut result__).from_abi::<TimeTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeTriggerFactory<R, F: FnOnce(&ITimeTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimeTrigger, ITimeTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimeTrigger {}
unsafe impl ::windows::core::RuntimeType for TimeTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TimeTrigger;{656e5556-0b2a-4377-ba70-3b45a935547f})");
}
unsafe impl ::windows::core::Interface for TimeTrigger {
    type Vtable = ITimeTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x656e5556_0b2a_4377_ba70_3b45a935547f);
}
impl ::windows::core::RuntimeName for TimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TimeTrigger";
}
impl ::core::convert::From<TimeTrigger> for ::windows::core::IUnknown {
    fn from(value: TimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimeTrigger> for ::windows::core::IUnknown {
    fn from(value: &TimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimeTrigger> for ::windows::core::IInspectable {
    fn from(value: TimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimeTrigger> for ::windows::core::IInspectable {
    fn from(value: &TimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: TimeTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimeTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimeTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &TimeTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ToastNotificationActionTrigger(::windows::core::IUnknown);
impl ToastNotificationActionTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationActionTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<ToastNotificationActionTrigger> {
        Self::IToastNotificationActionTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<ToastNotificationActionTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationActionTriggerFactory<R, F: FnOnce(&IToastNotificationActionTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationActionTrigger, IToastNotificationActionTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ToastNotificationActionTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActionTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActionTrigger {}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActionTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationActionTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for ToastNotificationActionTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for ToastNotificationActionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationActionTrigger";
}
impl ::core::convert::From<ToastNotificationActionTrigger> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationActionTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActionTrigger> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationActionTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationActionTrigger> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationActionTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActionTrigger> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationActionTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationActionTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActionTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActionTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActionTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ToastNotificationActionTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActionTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationActionTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTrigger(::windows::core::IUnknown);
impl ToastNotificationHistoryChangedTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationHistoryChangedTrigger, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Background'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<ToastNotificationHistoryChangedTrigger> {
        Self::IToastNotificationHistoryChangedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<ToastNotificationHistoryChangedTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationHistoryChangedTriggerFactory<R, F: FnOnce(&IToastNotificationHistoryChangedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ToastNotificationHistoryChangedTrigger, IToastNotificationHistoryChangedTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistoryChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::windows::core::RuntimeType for ToastNotificationHistoryChangedTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for ToastNotificationHistoryChangedTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for ToastNotificationHistoryChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger";
}
impl ::core::convert::From<ToastNotificationHistoryChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationHistoryChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationHistoryChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationHistoryChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationHistoryChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationHistoryChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationHistoryChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationHistoryChangedTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationHistoryChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationHistoryChangedTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &ToastNotificationHistoryChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationHistoryChangedTrigger {}
#[doc = "*Required features: 'ApplicationModel_Background'*"]
#[repr(transparent)]
pub struct UserNotificationChangedTrigger(::windows::core::IUnknown);
impl UserNotificationChangedTrigger {
    #[doc = "*Required features: 'ApplicationModel_Background', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn Create(notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows::core::Result<UserNotificationChangedTrigger> {
        Self::IUserNotificationChangedTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), notificationkinds, &mut result__).from_abi::<UserNotificationChangedTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserNotificationChangedTriggerFactory<R, F: FnOnce(&IUserNotificationChangedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserNotificationChangedTrigger, IUserNotificationChangedTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserNotificationChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotificationChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationChangedTrigger {}
unsafe impl ::windows::core::RuntimeType for UserNotificationChangedTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.UserNotificationChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows::core::Interface for UserNotificationChangedTrigger {
    type Vtable = IBackgroundTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
impl ::windows::core::RuntimeName for UserNotificationChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.UserNotificationChangedTrigger";
}
impl ::core::convert::From<UserNotificationChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: UserNotificationChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationChangedTrigger> for ::windows::core::IUnknown {
    fn from(value: &UserNotificationChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserNotificationChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: UserNotificationChangedTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationChangedTrigger> for ::windows::core::IInspectable {
    fn from(value: &UserNotificationChangedTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserNotificationChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: UserNotificationChangedTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserNotificationChangedTrigger> for IBackgroundTrigger {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserNotificationChangedTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTrigger> for &UserNotificationChangedTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTrigger> {
        ::core::convert::TryInto::<IBackgroundTrigger>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserNotificationChangedTrigger {}
unsafe impl ::core::marker::Sync for UserNotificationChangedTrigger {}
