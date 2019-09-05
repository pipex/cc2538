#[doc = "Reader of register DMAC_PERSR"]
pub type R = crate::R<u32, super::DMAC_PERSR>;
#[doc = "Writer for register DMAC_PERSR"]
pub type W = crate::W<u32, super::DMAC_PERSR>;
#[doc = "Register DMAC_PERSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_PERSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved19`"]
pub type RESERVED19_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved19`"]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 13)) | (((value as u32) & 0x0007_ffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `PORT1_AHB_ERROR`"]
pub type PORT1_AHB_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT1_AHB_ERROR`"]
pub struct PORT1_AHB_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT1_AHB_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PORT1_CHANNEL`"]
pub type PORT1_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT1_CHANNEL`"]
pub struct PORT1_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT1_CHANNEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `Reserved9`"]
pub type RESERVED9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:31 - 31:13\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 13) & 0x0007_ffff) as u32)
    }
    #[doc = "Bit 12 - 12:12\\] A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    pub fn port1_ahb_error(&self) -> PORT1_AHB_ERROR_R {
        PORT1_AHB_ERROR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\] Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    pub fn port1_channel(&self) -> PORT1_CHANNEL_R {
        PORT1_CHANNEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - 8:0\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 13:31 - 31:13\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    pub fn port1_ahb_error(&mut self) -> PORT1_AHB_ERROR_W {
        PORT1_AHB_ERROR_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    pub fn port1_channel(&mut self) -> PORT1_CHANNEL_W {
        PORT1_CHANNEL_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
}
