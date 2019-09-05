#[doc = "Reader of register CLOCK_CTRL"]
pub type R = crate::R<u32, super::CLOCK_CTRL>;
#[doc = "Writer for register CLOCK_CTRL"]
pub type W = crate::W<u32, super::CLOCK_CTRL>;
#[doc = "Register CLOCK_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `OSC32K_CALDIS`"]
pub type OSC32K_CALDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32K_CALDIS`"]
pub struct OSC32K_CALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32K_CALDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `OSC32K`"]
pub type OSC32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32K`"]
pub struct OSC32K_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `Reserved24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `AMP_DET`"]
pub type AMP_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMP_DET`"]
pub struct AMP_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `Reserved21`"]
pub type RESERVED21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved21`"]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `OSC_PD`"]
pub type OSC_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC_PD`"]
pub struct OSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OSC`"]
pub type OSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC`"]
pub struct OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `IO_DIV`"]
pub type IO_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO_DIV`"]
pub struct IO_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `Reserved5`"]
pub type RESERVED5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
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
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `SYS_DIV`"]
pub type SYS_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYS_DIV`"]
pub struct SYS_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\] Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> OSC32K_CALDIS_R {
        OSC32K_CALDIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\] 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> OSC32K_R {
        OSC32K_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - 21:21\\] Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&self) -> AMP_DET_R {
        AMP_DET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - 20:18\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 17 - 17:17\\] 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OSC_PD_R {
        OSC_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\] I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IO_DIV_R {
        IO_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SYS_DIV_R {
        SYS_DIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&mut self) -> OSC32K_CALDIS_W {
        OSC32K_CALDIS_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&mut self) -> OSC32K_W {
        OSC32K_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&mut self) -> AMP_DET_W {
        AMP_DET_W { w: self }
    }
    #[doc = "Bits 18:20 - 20:18\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&mut self) -> OSC_PD_W {
        OSC_PD_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&mut self) -> IO_DIV_W {
        IO_DIV_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&mut self) -> SYS_DIV_W {
        SYS_DIV_W { w: self }
    }
}
