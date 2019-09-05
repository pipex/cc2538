#[doc = "Reader of register RXCTRL"]
pub type R = crate::R<u32, super::RXCTRL>;
#[doc = "Writer for register RXCTRL"]
pub type W = crate::W<u32, super::RXCTRL>;
#[doc = "Register RXCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCTRL {
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `GBIAS_LNA2_REF`"]
pub type GBIAS_LNA2_REF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GBIAS_LNA2_REF`"]
pub struct GBIAS_LNA2_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> GBIAS_LNA2_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `GBIAS_LNA_REF`"]
pub type GBIAS_LNA_REF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GBIAS_LNA_REF`"]
pub struct GBIAS_LNA_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> GBIAS_LNA_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MIX_CURRENT`"]
pub type MIX_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIX_CURRENT`"]
pub struct MIX_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_CURRENT_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\] Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&self) -> GBIAS_LNA2_REF_R {
        GBIAS_LNA2_REF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&self) -> GBIAS_LNA_REF_R {
        GBIAS_LNA_REF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&self) -> MIX_CURRENT_R {
        MIX_CURRENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&mut self) -> GBIAS_LNA2_REF_W {
        GBIAS_LNA2_REF_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&mut self) -> GBIAS_LNA_REF_W {
        GBIAS_LNA_REF_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&mut self) -> MIX_CURRENT_W {
        MIX_CURRENT_W { w: self }
    }
}
