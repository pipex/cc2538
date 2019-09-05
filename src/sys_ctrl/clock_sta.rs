#[doc = "Reader of register CLOCK_STA"]
pub type R = crate::R<u32, super::CLOCK_STA>;
#[doc = "Writer for register CLOCK_STA"]
pub type W = crate::W<u32, super::CLOCK_STA>;
#[doc = "Register CLOCK_STA `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK_STA {
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
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `SYNC_32K`"]
pub type SYNC_32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC_32K`"]
pub struct SYNC_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
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
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `Reserved22`"]
pub type RESERVED22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved22`"]
pub struct RESERVED22_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED22_W<'a> {
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
#[doc = "Reader of field `SOURCE_CHANGE`"]
pub type SOURCE_CHANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOURCE_CHANGE`"]
pub struct SOURCE_CHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_CHANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `XOSC_STB`"]
pub type XOSC_STB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_STB`"]
pub struct XOSC_STB_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_STB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HSOSC_STB`"]
pub type HSOSC_STB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSOSC_STB`"]
pub struct HSOSC_STB_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOSC_STB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTCLK_FREQ`"]
pub type RTCLK_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCLK_FREQ`"]
pub struct RTCLK_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCLK_FREQ_W<'a> {
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
    #[doc = "Bits 27:31 - 31:27\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - 26:26\\] 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
    #[inline(always)]
    pub fn sync_32k(&self) -> SYNC_32K_R {
        SYNC_32K_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\] Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> OSC32K_CALDIS_R {
        OSC32K_CALDIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\] Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> OSC32K_R {
        OSC32K_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\] Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - 21:21\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\] 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
    #[inline(always)]
    pub fn source_change(&self) -> SOURCE_CHANGE_R {
        SOURCE_CHANGE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\] XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
    #[inline(always)]
    pub fn xosc_stb(&self) -> XOSC_STB_R {
        XOSC_STB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\] HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
    #[inline(always)]
    pub fn hsosc_stb(&self) -> HSOSC_STB_R {
        HSOSC_STB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\] 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OSC_PD_R {
        OSC_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\] Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IO_DIV_R {
        IO_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\] Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
    #[inline(always)]
    pub fn rtclk_freq(&self) -> RTCLK_FREQ_R {
        RTCLK_FREQ_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SYS_DIV_R {
        SYS_DIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - 31:27\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
    #[inline(always)]
    pub fn sync_32k(&mut self) -> SYNC_32K_W {
        SYNC_32K_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
    #[inline(always)]
    pub fn osc32k_caldis(&mut self) -> OSC32K_CALDIS_W {
        OSC32K_CALDIS_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&mut self) -> OSC32K_W {
        OSC32K_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved22(&mut self) -> RESERVED22_W {
        RESERVED22_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
    #[inline(always)]
    pub fn source_change(&mut self) -> SOURCE_CHANGE_W {
        SOURCE_CHANGE_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
    #[inline(always)]
    pub fn xosc_stb(&mut self) -> XOSC_STB_W {
        XOSC_STB_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
    #[inline(always)]
    pub fn hsosc_stb(&mut self) -> HSOSC_STB_W {
        HSOSC_STB_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
    #[inline(always)]
    pub fn osc_pd(&mut self) -> OSC_PD_W {
        OSC_PD_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&mut self) -> IO_DIV_W {
        IO_DIV_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
    #[inline(always)]
    pub fn rtclk_freq(&mut self) -> RTCLK_FREQ_W {
        RTCLK_FREQ_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&mut self) -> SYS_DIV_W {
        SYS_DIV_W { w: self }
    }
}
