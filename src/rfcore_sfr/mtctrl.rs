#[doc = "Reader of register MTCTRL"]
pub type R = crate::R<u32, super::MTCTRL>;
#[doc = "Writer for register MTCTRL"]
pub type W = crate::W<u32, super::MTCTRL>;
#[doc = "Register MTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MTCTRL {
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
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `LATCH_MODE`"]
pub type LATCH_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATCH_MODE`"]
pub struct LATCH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCH_MODE_W<'a> {
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
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
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
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
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
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 4:7 - 7:4\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
    #[inline(always)]
    pub fn latch_mode(&self) -> LATCH_MODE_R {
        LATCH_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] State of MAC Timer 0: Timer idle 1: Timer running"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
    #[inline(always)]
    pub fn latch_mode(&mut self) -> LATCH_MODE_W {
        LATCH_MODE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] State of MAC Timer 0: Timer idle 1: Timer running"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
}
