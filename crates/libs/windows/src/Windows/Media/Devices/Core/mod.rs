#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct CameraIntrinsics(::windows::core::IUnknown);
impl CameraIntrinsics {
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FocalLength(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PrincipalPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RadialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TangentialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn ImageWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn ImageHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn ProjectOntoFrame<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, coordinate: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), coordinate.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn UnprojectAtUnitDepth<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, pixelcoordinate: Param0) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), pixelcoordinate.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn ProjectManyOntoFrame(&self, coordinates: &[<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), coordinates.len() as u32, ::core::mem::transmute(coordinates.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), pixelcoordinates.len() as u32, ::core::mem::transmute(pixelcoordinates.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UndistortedProjectionTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DistortPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, input: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), inputs.len() as u32, ::core::mem::transmute(inputs.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, input: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), inputs.len() as u32, ::core::mem::transmute(inputs.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(focallength: Param0, principalpoint: Param1, radialdistortion: Param2, tangentialdistortion: Param3, imagewidth: u32, imageheight: u32) -> ::windows::core::Result<CameraIntrinsics> {
        Self::ICameraIntrinsicsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focallength.into_param().abi(), principalpoint.into_param().abi(), radialdistortion.into_param().abi(), tangentialdistortion.into_param().abi(), imagewidth, imageheight, &mut result__).from_abi::<CameraIntrinsics>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICameraIntrinsicsFactory<R, F: FnOnce(&ICameraIntrinsicsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraIntrinsics, ICameraIntrinsicsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CameraIntrinsics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraIntrinsics {}
unsafe impl ::windows::core::RuntimeType for CameraIntrinsics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.CameraIntrinsics;{0aa6ed32-6589-49da-afde-594270ca0aac})");
}
unsafe impl ::windows::core::Interface for CameraIntrinsics {
    type Vtable = ICameraIntrinsicsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa6ed32_6589_49da_afde_594270ca0aac);
}
impl ::windows::core::RuntimeName for CameraIntrinsics {
    const NAME: &'static str = "Windows.Media.Devices.Core.CameraIntrinsics";
}
impl ::core::convert::From<CameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraIntrinsics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CameraIntrinsics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraIntrinsics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CameraIntrinsics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraIntrinsics {}
unsafe impl ::core::marker::Sync for CameraIntrinsics {}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct DepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
impl DepthCorrelatedCoordinateMapper {
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics', 'Perception_Spatial'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, sourcepoint: Param0, targetcoordinatesystem: Param1) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourcepoint.into_param().abi(), targetcoordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Foundation_Numerics', 'Perception_Spatial'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoints<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: Param1, results: &mut [<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourcepoints.len() as u32, ::core::mem::transmute(sourcepoints.as_ptr()), targetcoordinatesystem.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Perception_Spatial'*"]
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param2: ::windows::core::IntoParam<'a, CameraIntrinsics>>(&self, sourcepoint: Param0, targetcoordinatesystem: Param1, targetcameraintrinsics: Param2) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sourcepoint.into_param().abi(), targetcoordinatesystem.into_param().abi(), targetcameraintrinsics.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation', 'Perception_Spatial'*"]
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoints<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param2: ::windows::core::IntoParam<'a, CameraIntrinsics>>(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: Param1, targetcameraintrinsics: Param2, results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sourcepoints.len() as u32, ::core::mem::transmute(sourcepoints.as_ptr()), targetcoordinatesystem.into_param().abi(), targetcameraintrinsics.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
}
impl ::core::clone::Clone for DepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthCorrelatedCoordinateMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthCorrelatedCoordinateMapper {}
unsafe impl ::windows::core::RuntimeType for DepthCorrelatedCoordinateMapper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper;{f95d89fb-8af0-4cb0-926d-696866e5046a})");
}
unsafe impl ::windows::core::Interface for DepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf95d89fb_8af0_4cb0_926d_696866e5046a);
}
impl ::windows::core::RuntimeName for DepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DepthCorrelatedCoordinateMapper) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DepthCorrelatedCoordinateMapper) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DepthCorrelatedCoordinateMapper {}
unsafe impl ::core::marker::Sync for DepthCorrelatedCoordinateMapper {}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameControlCapabilities(::windows::core::IUnknown);
impl FrameControlCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Exposure(&self) -> ::windows::core::Result<FrameExposureCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameExposureCapabilities>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn ExposureCompensation(&self) -> ::windows::core::Result<FrameExposureCompensationCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameExposureCompensationCapabilities>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn IsoSpeed(&self) -> ::windows::core::Result<FrameIsoSpeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameIsoSpeedCapabilities>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Focus(&self) -> ::windows::core::Result<FrameFocusCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameFocusCapabilities>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn PhotoConfirmationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Flash(&self) -> ::windows::core::Result<FrameFlashCapabilities> {
        let this = &::windows::core::Interface::cast::<IFrameControlCapabilities2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameFlashCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameControlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameControlCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameControlCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameControlCapabilities;{a8ffae60-4e9e-4377-a789-e24c4ae7e544})");
}
unsafe impl ::windows::core::Interface for FrameControlCapabilities {
    type Vtable = IFrameControlCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8ffae60_4e9e_4377_a789_e24c4ae7e544);
}
impl ::windows::core::RuntimeName for FrameControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameControlCapabilities";
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameController(::windows::core::IUnknown);
impl FrameController {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FrameController, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn ExposureControl(&self) -> ::windows::core::Result<FrameExposureControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameExposureControl>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn ExposureCompensationControl(&self) -> ::windows::core::Result<FrameExposureCompensationControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameExposureCompensationControl>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn IsoSpeedControl(&self) -> ::windows::core::Result<FrameIsoSpeedControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameIsoSpeedControl>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn FocusControl(&self) -> ::windows::core::Result<FrameFocusControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameFocusControl>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationEnabled(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPhotoConfirmationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn FlashControl(&self) -> ::windows::core::Result<FrameFlashControl> {
        let this = &::windows::core::Interface::cast::<IFrameController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameFlashControl>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameController {}
unsafe impl ::windows::core::RuntimeType for FrameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameController;{c16459d9-baef-4052-9177-48aff2af7522})");
}
unsafe impl ::windows::core::Interface for FrameController {
    type Vtable = IFrameControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16459d9_baef_4052_9177_48aff2af7522);
}
impl ::windows::core::RuntimeName for FrameController {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameController";
}
impl ::core::convert::From<FrameController> for ::windows::core::IUnknown {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows::core::IUnknown {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameController> for ::windows::core::IInspectable {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows::core::IInspectable {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FrameController {}
unsafe impl ::core::marker::Sync for FrameController {}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameExposureCapabilities(::windows::core::IUnknown);
impl FrameExposureCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameExposureCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCapabilities;{bdbe9ce3-3985-4e72-97c2-0590d61307a1})");
}
unsafe impl ::windows::core::Interface for FrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdbe9ce3_3985_4e72_97c2_0590d61307a1);
}
impl ::windows::core::RuntimeName for FrameExposureCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCapabilities";
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameExposureCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameExposureCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameExposureCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameExposureCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameExposureCompensationCapabilities(::windows::core::IUnknown);
impl FrameExposureCompensationCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameExposureCompensationCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationCapabilities;{b988a823-8065-41ee-b04f-722265954500})");
}
unsafe impl ::windows::core::Interface for FrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb988a823_8065_41ee_b04f_722265954500);
}
impl ::windows::core::RuntimeName for FrameExposureCompensationCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameExposureCompensationControl(::windows::core::IUnknown);
impl FrameExposureCompensationControl {
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationControl {}
unsafe impl ::windows::core::RuntimeType for FrameExposureCompensationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationControl;{e95896c9-f7f9-48ca-8591-a26531cb1578})");
}
unsafe impl ::windows::core::Interface for FrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe95896c9_f7f9_48ca_8591_a26531cb1578);
}
impl ::windows::core::RuntimeName for FrameExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationControl";
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameExposureControl(::windows::core::IUnknown);
impl FrameExposureControl {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureControl {}
unsafe impl ::windows::core::RuntimeType for FrameExposureControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureControl;{b1605a61-ffaf-4752-b621-f5b6f117f432})");
}
unsafe impl ::windows::core::Interface for FrameExposureControl {
    type Vtable = IFrameExposureControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1605a61_ffaf_4752_b621_f5b6f117f432);
}
impl ::windows::core::RuntimeName for FrameExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureControl";
}
impl ::core::convert::From<FrameExposureControl> for ::windows::core::IUnknown {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureControl> for ::windows::core::IInspectable {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameFlashCapabilities(::windows::core::IUnknown);
impl FrameFlashCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFlashCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameFlashCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashCapabilities;{bb9341a2-5ebe-4f62-8223-0e2b05bfbbd0})");
}
unsafe impl ::windows::core::Interface for FrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb9341a2_5ebe_4f62_8223_0e2b05bfbbd0);
}
impl ::windows::core::RuntimeName for FrameFlashCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashCapabilities";
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameFlashCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameFlashCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameFlashCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameFlashCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameFlashControl(::windows::core::IUnknown);
impl FrameFlashControl {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Mode(&self) -> ::windows::core::Result<FrameFlashMode> {
        let this = self;
        unsafe {
            let mut result__: FrameFlashMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameFlashMode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetMode(&self, value: FrameFlashMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn RedEyeReduction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for FrameFlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashControl {}
unsafe impl ::windows::core::RuntimeType for FrameFlashControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashControl;{75d5f6c7-bd45-4fab-9375-45ac04b332c2})");
}
unsafe impl ::windows::core::Interface for FrameFlashControl {
    type Vtable = IFrameFlashControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d5f6c7_bd45_4fab_9375_45ac04b332c2);
}
impl ::windows::core::RuntimeName for FrameFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashControl";
}
impl ::core::convert::From<FrameFlashControl> for ::windows::core::IUnknown {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows::core::IUnknown {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameFlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameFlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFlashControl> for ::windows::core::IInspectable {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows::core::IInspectable {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameFlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameFlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl ::core::marker::Copy for FrameFlashMode {}
impl ::core::clone::Clone for FrameFlashMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FrameFlashMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FrameFlashMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashMode {}
unsafe impl ::windows::core::RuntimeType for FrameFlashMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.Core.FrameFlashMode;i4)");
}
impl ::windows::core::DefaultType for FrameFlashMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameFocusCapabilities(::windows::core::IUnknown);
impl FrameFocusCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFocusCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameFocusCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusCapabilities;{7b25cd58-01c0-4065-9c40-c1a721425c1a})");
}
unsafe impl ::windows::core::Interface for FrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b25cd58_01c0_4065_9c40_c1a721425c1a);
}
impl ::windows::core::RuntimeName for FrameFocusCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusCapabilities";
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameFocusCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameFocusCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameFocusCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameFocusCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameFocusControl(::windows::core::IUnknown);
impl FrameFocusControl {
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameFocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusControl {}
unsafe impl ::windows::core::RuntimeType for FrameFocusControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusControl;{272df1d0-d912-4214-a67b-e38a8d48d8c6})");
}
unsafe impl ::windows::core::Interface for FrameFocusControl {
    type Vtable = IFrameFocusControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272df1d0_d912_4214_a67b_e38a8d48d8c6);
}
impl ::windows::core::RuntimeName for FrameFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusControl";
}
impl ::core::convert::From<FrameFocusControl> for ::windows::core::IUnknown {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows::core::IUnknown {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameFocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameFocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFocusControl> for ::windows::core::IInspectable {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows::core::IInspectable {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameFocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameFocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameIsoSpeedCapabilities(::windows::core::IUnknown);
impl FrameIsoSpeedCapabilities {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedCapabilities {}
unsafe impl ::windows::core::RuntimeType for FrameIsoSpeedCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedCapabilities;{16bdff61-6df6-4ac9-b92a-9f6ecd1ad2fa})");
}
unsafe impl ::windows::core::Interface for FrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16bdff61_6df6_4ac9_b92a_9f6ecd1ad2fa);
}
impl ::windows::core::RuntimeName for FrameIsoSpeedCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct FrameIsoSpeedControl(::windows::core::IUnknown);
impl FrameIsoSpeedControl {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedControl {}
unsafe impl ::windows::core::RuntimeType for FrameIsoSpeedControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedControl;{1a03efed-786a-4c75-a557-7ab9a85f588c})");
}
unsafe impl ::windows::core::Interface for FrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a03efed_786a_4c75_a557_7ab9a85f588c);
}
impl ::windows::core::RuntimeName for FrameIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedControl";
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameIsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameIsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameIsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameIsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsics {
    type Vtable = ICameraIntrinsicsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa6ed32_6589_49da_afde_594270ca0aac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsicsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinate: super::super::super::Foundation::Numerics::Vector3, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinates_array_size: u32, coordinates: *const super::super::super::Foundation::Numerics::Vector3, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelCoordinates_array_size: u32, pixelcoordinates: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsics2 {
    type Vtable = ICameraIntrinsics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cdaa447_0798_4b4d_839f_c5ec414db27a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsicsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsicsFactory {
    type Vtable = ICameraIntrinsicsFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ddc486_2132_4a34_a659_9bfe2a055712);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsicsFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf95d89fb_8af0_4cb0_926d_696866e5046a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthCorrelatedCoordinateMapperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, targetcameraintrinsics: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, targetcameraintrinsics: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameControlCapabilities {
    type Vtable = IFrameControlCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8ffae60_4e9e_4377_a789_e24c4ae7e544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameControlCapabilities2 {
    type Vtable = IFrameControlCapabilities2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce9b0464_4730_440f_bd3e_efe8a8f230a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilities2Vtbl(
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
pub struct IFrameController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameController {
    type Vtable = IFrameControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16459d9_baef_4052_9177_48aff2af7522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameController2 {
    type Vtable = IFrameController2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d3bc75_d87c_485b_8a09_5c358568b427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameController2Vtbl(
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
pub struct IFrameExposureCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdbe9ce3_3985_4e72_97c2_0590d61307a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb988a823_8065_41ee_b04f_722265954500);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe95896c9_f7f9_48ca_8591_a26531cb1578);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationControlVtbl(
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
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureControl {
    type Vtable = IFrameExposureControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1605a61_ffaf_4752_b621_f5b6f117f432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb9341a2_5ebe_4f62_8223_0e2b05bfbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFlashControl {
    type Vtable = IFrameFlashControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d5f6c7_bd45_4fab_9375_45ac04b332c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FrameFlashMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FrameFlashMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b25cd58_01c0_4065_9c40_c1a721425c1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFocusControl {
    type Vtable = IFrameFocusControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272df1d0_d912_4214_a67b_e38a8d48d8c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusControlVtbl(
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
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16bdff61_6df6_4ac9_b92a_9f6ecd1ad2fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a03efed_786a_4c75_a557_7ab9a85f588c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fbff880_ed8c_43fd_a7c3_b35809e4229a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Media_Devices_Core'*"]
#[repr(transparent)]
pub struct VariablePhotoSequenceController(::windows::core::IUnknown);
impl VariablePhotoSequenceController {
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::MediaProperties::IMediaEncodingProperties>>(&self, captureproperties: Param0) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), captureproperties.into_param().abi(), &mut result__).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core'*"]
    pub fn FrameCapabilities(&self) -> ::windows::core::Result<FrameControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FrameControlCapabilities>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Devices_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFrameControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<FrameController>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<FrameController>>(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoSequenceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceController {}
unsafe impl ::windows::core::RuntimeType for VariablePhotoSequenceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.VariablePhotoSequenceController;{7fbff880-ed8c-43fd-a7c3-b35809e4229a})");
}
unsafe impl ::windows::core::Interface for VariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fbff880_ed8c_43fd_a7c3_b35809e4229a);
}
impl ::windows::core::RuntimeName for VariablePhotoSequenceController {
    const NAME: &'static str = "Windows.Media.Devices.Core.VariablePhotoSequenceController";
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows::core::IUnknown {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows::core::IUnknown {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VariablePhotoSequenceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VariablePhotoSequenceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows::core::IInspectable {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows::core::IInspectable {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VariablePhotoSequenceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VariablePhotoSequenceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
