#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop_Core")]
pub mod Core;
#[doc = "*Required features: 'ApplicationModel_DataTransfer_DragDrop'*"]
#[repr(transparent)]
pub struct DragDropModifiers(pub u32);
impl DragDropModifiers {
    pub const None: Self = Self(0u32);
    pub const Shift: Self = Self(1u32);
    pub const Control: Self = Self(2u32);
    pub const Alt: Self = Self(4u32);
    pub const LeftButton: Self = Self(8u32);
    pub const MiddleButton: Self = Self(16u32);
    pub const RightButton: Self = Self(32u32);
}
impl ::core::marker::Copy for DragDropModifiers {}
impl ::core::clone::Clone for DragDropModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DragDropModifiers {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DragDropModifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragDropModifiers {}
unsafe impl ::windows::core::RuntimeType for DragDropModifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.DragDropModifiers;u4)");
}
impl ::windows::core::DefaultType for DragDropModifiers {
    type DefaultType = Self;
}
