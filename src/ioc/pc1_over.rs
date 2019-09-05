#[doc = "Reader of register PC1_OVER"]
pub type R = crate::R<u32, super::PC1_OVER>;
#[doc = "Writer for register PC1_OVER"]
pub type W = crate::W<u32, super::PC1_OVER>;
#[doc = "Register PC1_OVER `reset()`'s with value 0"]
impl crate::ResetValue for super::PC1_OVER {
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
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `PC1_over`"]
pub type PC1_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC1_over`"]
pub struct PC1_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\] 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc1_over(&self) -> PC1_OVER_R {
        PC1_OVER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc1_over(&mut self) -> PC1_OVER_W {
        PC1_OVER_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
}
