#[doc = "Reader of register RSSI"]
pub type R = crate::R<u32, super::RSSI>;
#[doc = "Writer for register RSSI"]
pub type W = crate::W<u32, super::RSSI>;
#[doc = "Register RSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::RSSI {
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
#[doc = "Reader of field `RSSI_VAL`"]
pub type RSSI_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSSI_VAL`"]
pub struct RSSI_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_VAL_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] RSSI estimate on a logarithmic scale, signed number on 2's complement Unit is 1 dB, offset is 73dB. The RSSI value is averaged over eight symbol periods. The RSSI_VALID status bit should be checked before reading RSSI_VAL for the first time. The reset value of -128 also indicates that the RSSI value is invalid."]
    #[inline(always)]
    pub fn rssi_val(&self) -> RSSI_VAL_R {
        RSSI_VAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] RSSI estimate on a logarithmic scale, signed number on 2's complement Unit is 1 dB, offset is 73dB. The RSSI value is averaged over eight symbol periods. The RSSI_VALID status bit should be checked before reading RSSI_VAL for the first time. The reset value of -128 also indicates that the RSSI value is invalid."]
    #[inline(always)]
    pub fn rssi_val(&mut self) -> RSSI_VAL_W {
        RSSI_VAL_W { w: self }
    }
}
