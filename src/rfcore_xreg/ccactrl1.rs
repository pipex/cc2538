#[doc = "Reader of register CCACTRL1"]
pub type R = crate::R<u32, super::CCACTRL1>;
#[doc = "Writer for register CCACTRL1"]
pub type W = crate::W<u32, super::CCACTRL1>;
#[doc = "Register CCACTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCACTRL1 {
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
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `CCA_MODE`"]
pub type CCA_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCA_MODE`"]
pub struct CCA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `CCA_HYST`"]
pub type CCA_HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCA_HYST`"]
pub struct CCA_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\] 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CCA_MODE_R {
        CCA_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&self) -> CCA_HYST_R {
        CCA_HYST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&mut self) -> CCA_MODE_W {
        CCA_MODE_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&mut self) -> CCA_HYST_W {
        CCA_HYST_W { w: self }
    }
}
