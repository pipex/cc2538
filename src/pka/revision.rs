#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Writer for register REVISION"]
pub type W = crate::W<u32, super::REVISION>;
#[doc = "Register REVISION `reset()`'s with value 0"]
impl crate::ResetValue for super::REVISION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAJOR_HW_REVISION`"]
pub type MAJOR_HW_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJOR_HW_REVISION`"]
pub struct MAJOR_HW_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_HW_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MINOR_HW_REVISION`"]
pub type MINOR_HW_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINOR_HW_REVISION`"]
pub struct MINOR_HW_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_HW_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `HW_PATCH_LEVEL`"]
pub type HW_PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_PATCH_LEVEL`"]
pub struct HW_PATCH_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_PATCH_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMPLEMENT_OF_BASIC_EIP_NUMBER`"]
pub type COMPLEMENT_OF_BASIC_EIP_NUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPLEMENT_OF_BASIC_EIP_NUMBER`"]
pub struct COMPLEMENT_OF_BASIC_EIP_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLEMENT_OF_BASIC_EIP_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BASIC_EIP_NUMBER`"]
pub type BASIC_EIP_NUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BASIC_EIP_NUMBER`"]
pub struct BASIC_EIP_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_EIP_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    pub fn major_hw_revision(&self) -> MAJOR_HW_REVISION_R {
        MAJOR_HW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    pub fn minor_hw_revision(&self) -> MINOR_HW_REVISION_R {
        MINOR_HW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_R {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\] 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    pub fn basic_eip_number(&self) -> BASIC_EIP_NUMBER_R {
        BASIC_EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    pub fn major_hw_revision(&mut self) -> MAJOR_HW_REVISION_W {
        MAJOR_HW_REVISION_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    pub fn minor_hw_revision(&mut self) -> MINOR_HW_REVISION_W {
        MINOR_HW_REVISION_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn hw_patch_level(&mut self) -> HW_PATCH_LEVEL_W {
        HW_PATCH_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&mut self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_W {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    pub fn basic_eip_number(&mut self) -> BASIC_EIP_NUMBER_W {
        BASIC_EIP_NUMBER_W { w: self }
    }
}
