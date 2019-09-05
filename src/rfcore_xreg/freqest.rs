#[doc = "Reader of register FREQEST"]
pub type R = crate::R<u32, super::FREQEST>;
#[doc = "Writer for register FREQEST"]
pub type W = crate::W<u32, super::FREQEST>;
#[doc = "Register FREQEST `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQEST {
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
#[doc = "Reader of field `FREQEST`"]
pub type FREQEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQEST`"]
pub struct FREQEST_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
    #[inline(always)]
    pub fn freqest(&self) -> FREQEST_R {
        FREQEST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
    #[inline(always)]
    pub fn freqest(&mut self) -> FREQEST_W {
        FREQEST_W { w: self }
    }
}
