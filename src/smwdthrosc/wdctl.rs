#[doc = "Reader of register WDCTL"]
pub type R = crate::R<u32, super::WDCTL>;
#[doc = "Writer for register WDCTL"]
pub type W = crate::W<u32, super::WDCTL>;
#[doc = "Register WDCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLR`"]
pub type CLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLR`"]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
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
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 4:7 - 7:4\\] Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\] is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\] Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\] is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
}
