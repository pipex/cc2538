#[doc = "Reader of register DIECFG2"]
pub type R = crate::R<u32, super::DIECFG2>;
#[doc = "Writer for register DIECFG2"]
pub type W = crate::W<u32, super::DIECFG2>;
#[doc = "Register DIECFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIECFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIE_MAJOR_REVISION`"]
pub type DIE_MAJOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIE_MAJOR_REVISION`"]
pub struct DIE_MAJOR_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIE_MAJOR_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIE_MINOR_REVISION`"]
pub type DIE_MINOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIE_MINOR_REVISION`"]
pub struct DIE_MINOR_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIE_MINOR_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `RF_CORE_EN`"]
pub type RF_CORE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_CORE_EN`"]
pub struct RF_CORE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CORE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AES_EN`"]
pub type AES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_EN`"]
pub struct AES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PKA_EN`"]
pub type PKA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_EN`"]
pub struct PKA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\] Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
    #[inline(always)]
    pub fn die_major_revision(&self) -> DIE_MAJOR_REVISION_R {
        DIE_MAJOR_REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\] Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
    #[inline(always)]
    pub fn die_minor_revision(&self) -> DIE_MINOR_REVISION_R {
        DIE_MINOR_REVISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\] Unused"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\] 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
    #[inline(always)]
    pub fn rf_core_en(&self) -> RF_CORE_EN_R {
        RF_CORE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 1: AES is enabled. 0: AES is permanently disabled."]
    #[inline(always)]
    pub fn aes_en(&self) -> AES_EN_R {
        AES_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 1: PKA is enabled. 0: PKA is permanently disabled."]
    #[inline(always)]
    pub fn pka_en(&self) -> PKA_EN_R {
        PKA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
    #[inline(always)]
    pub fn die_major_revision(&mut self) -> DIE_MAJOR_REVISION_W {
        DIE_MAJOR_REVISION_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
    #[inline(always)]
    pub fn die_minor_revision(&mut self) -> DIE_MINOR_REVISION_W {
        DIE_MINOR_REVISION_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Unused"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
    #[inline(always)]
    pub fn rf_core_en(&mut self) -> RF_CORE_EN_W {
        RF_CORE_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 1: AES is enabled. 0: AES is permanently disabled."]
    #[inline(always)]
    pub fn aes_en(&mut self) -> AES_EN_W {
        AES_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 1: PKA is enabled. 0: PKA is permanently disabled."]
    #[inline(always)]
    pub fn pka_en(&mut self) -> PKA_EN_W {
        PKA_EN_W { w: self }
    }
}
