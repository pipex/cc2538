#[doc = "Reader of register TAPS"]
pub type R = crate::R<u32, super::TAPS>;
#[doc = "Writer for register TAPS"]
pub type W = crate::W<u32, super::TAPS>;
#[doc = "Register TAPS `reset()`'s with value 0"]
impl crate::ResetValue for super::TAPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PSS`"]
pub type PSS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSS`"]
pub struct PSS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\] GPTM Timer A prescaler"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] GPTM Timer A prescaler"]
    #[inline(always)]
    pub fn pss(&mut self) -> PSS_W {
        PSS_W { w: self }
    }
}
