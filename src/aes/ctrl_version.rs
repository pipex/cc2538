#[doc = "Reader of register CTRL_VERSION"]
pub type R = crate::R<u32, super::CTRL_VERSION>;
#[doc = "Writer for register CTRL_VERSION"]
pub type W = crate::W<u32, super::CTRL_VERSION>;
#[doc = "Register CTRL_VERSION `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_VERSION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MAJOR_VERSION`"]
pub type MAJOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJOR_VERSION`"]
pub struct MAJOR_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MINOR_VERSION`"]
pub type MINOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINOR_VERSION`"]
pub struct MINOR_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PATCH_LEVEL`"]
pub type PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PATCH_LEVEL`"]
pub struct PATCH_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_LEVEL_W<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Bit should be ignored"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline(always)]
    pub fn major_version(&self) -> MAJOR_VERSION_R {
        MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline(always)]
    pub fn minor_version(&self) -> MINOR_VERSION_R {
        MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn patch_level(&self) -> PATCH_LEVEL_R {
        PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\] These bits simply contain the complement of bits \\[7:0\\] (0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\] These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\] Bit should be ignored"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline(always)]
    pub fn major_version(&mut self) -> MAJOR_VERSION_W {
        MAJOR_VERSION_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline(always)]
    pub fn minor_version(&mut self) -> MINOR_VERSION_W {
        MINOR_VERSION_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn patch_level(&mut self) -> PATCH_LEVEL_W {
        PATCH_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] These bits simply contain the complement of bits \\[7:0\\] (0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn eip_number_compl(&mut self) -> EIP_NUMBER_COMPL_W {
        EIP_NUMBER_COMPL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn eip_number(&mut self) -> EIP_NUMBER_W {
        EIP_NUMBER_W { w: self }
    }
}
