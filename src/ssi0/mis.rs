#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
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
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIS`"]
pub struct TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMIS`"]
pub struct RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMIS_W<'a> {
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
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMIS`"]
pub struct RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIS_W<'a> {
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
#[doc = "Reader of field `RORMIS`"]
pub type RORMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RORMIS`"]
pub struct RORMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RORMIS_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 4:15 - 15:4\\] Reserved, read as zero, do not modify."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 3 - 3:3\\] SSI SSITXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSITXINTR"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] SSI SSIRXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRXINTR"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] SSI SSIRTINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRTINTR"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] SSI SSIRORINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRORINTR"]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\] Reserved, read as zero, do not modify."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] SSI SSITXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSITXINTR"]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] SSI SSIRXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRXINTR"]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] SSI SSIRTINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRTINTR"]
    #[inline(always)]
    pub fn rtmis(&mut self) -> RTMIS_W {
        RTMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] SSI SSIRORINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRORINTR"]
    #[inline(always)]
    pub fn rormis(&mut self) -> RORMIS_W {
        RORMIS_W { w: self }
    }
}
