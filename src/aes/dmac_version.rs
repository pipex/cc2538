#[doc = "Reader of register DMAC_VERSION"]
pub type R = crate::R<u32, super::DMAC_VERSION>;
#[doc = "Writer for register DMAC_VERSION"]
pub type W = crate::W<u32, super::DMAC_VERSION>;
#[doc = "Register DMAC_VERSION `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_VERSION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `HW_MAJOR_VERSION`"]
pub type HW_MAJOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_MAJOR_VERSION`"]
pub struct HW_MAJOR_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MAJOR_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HW_MINOR_VERSION`"]
pub type HW_MINOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_MINOR_VERSION`"]
pub struct HW_MINOR_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MINOR_VERSION_W<'a> {
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
#[doc = "Reader of field `EIP_NUMBER_COMPL`"]
pub type EIP_NUMBER_COMPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIP_NUMBER_COMPL`"]
pub struct EIP_NUMBER_COMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP_NUMBER_COMPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EIP_NUMBER`"]
pub type EIP_NUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIP_NUMBER`"]
pub struct EIP_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HW_MAJOR_VERSION_R {
        HW_MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HW_MINOR_VERSION_R {
        HW_MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\] Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&mut self) -> HW_MAJOR_VERSION_W {
        HW_MAJOR_VERSION_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&mut self) -> HW_MINOR_VERSION_W {
        HW_MINOR_VERSION_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&mut self) -> HW_PATCH_LEVEL_W {
        HW_PATCH_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&mut self) -> EIP_NUMBER_COMPL_W {
        EIP_NUMBER_COMPL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&mut self) -> EIP_NUMBER_W {
        EIP_NUMBER_W { w: self }
    }
}
