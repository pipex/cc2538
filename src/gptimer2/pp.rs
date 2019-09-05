#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Writer for register PP"]
pub type W = crate::W<u32, super::PP>;
#[doc = "Register PP `reset()`'s with value 0"]
impl crate::ResetValue for super::PP {
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
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `ALTCLK`"]
pub type ALTCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTCLK`"]
pub struct ALTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYNCNT`"]
pub type SYNCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCNT`"]
pub struct SYNCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CHAIN`"]
pub type CHAIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAIN`"]
pub struct CHAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 6 - 6:6\\] Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
    #[inline(always)]
    pub fn altclk(&self) -> ALTCLK_R {
        ALTCLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
    #[inline(always)]
    pub fn syncnt(&self) -> SYNCNT_R {
        SYNCNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\] Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
    #[inline(always)]
    pub fn altclk(&mut self) -> ALTCLK_W {
        ALTCLK_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
    #[inline(always)]
    pub fn syncnt(&mut self) -> SYNCNT_W {
        SYNCNT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
    #[inline(always)]
    pub fn chain(&mut self) -> CHAIN_W {
        CHAIN_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}
