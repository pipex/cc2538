#[doc = "Reader of register ADCCON1"]
pub type R = crate::R<u32, super::ADCCON1>;
#[doc = "Writer for register ADCCON1"]
pub type W = crate::W<u32, super::ADCCON1>;
#[doc = "Register ADCCON1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCON1 {
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
#[doc = "Reader of field `EOC`"]
pub type EOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOC`"]
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
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
#[doc = "Reader of field `STSEL`"]
pub type STSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEL`"]
pub struct STSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RCTRL`"]
pub type RCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCTRL`"]
pub struct RCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\] Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&self) -> RCTRL_R {
        RCTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Reserved. Always set to 11."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&mut self) -> STSEL_W {
        STSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&mut self) -> RCTRL_W {
        RCTRL_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Reserved. Always set to 11."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
}
