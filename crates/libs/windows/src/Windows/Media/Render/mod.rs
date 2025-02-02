#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Media_Render'*"]
#[repr(transparent)]
pub struct AudioRenderCategory(pub i32);
impl AudioRenderCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for AudioRenderCategory {}
impl ::core::clone::Clone for AudioRenderCategory {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AudioRenderCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AudioRenderCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioRenderCategory {}
unsafe impl ::windows::core::RuntimeType for AudioRenderCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Render.AudioRenderCategory;i4)");
}
impl ::windows::core::DefaultType for AudioRenderCategory {
    type DefaultType = Self;
}
