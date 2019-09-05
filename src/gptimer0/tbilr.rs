#[doc = "Reader of register TBILR"]
pub type R = crate::R<u32, super::TBILR>;
#[doc = "Writer for register TBILR"]
pub type W = crate::W<u32, super::TBILR>;
#[doc = "Register TBILR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBILR {
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
#[doc = "Reader of field `TBILR`"]
pub type TBILR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBILR`"]
pub struct TBILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILR_W<'a> {
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
    #[doc = "Bits 0:15 - 15:0\\] GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&mut self) -> TBILR_W {
        TBILR_W { w: self }
    }
}
