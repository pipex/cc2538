#[doc = "Reader of register TBPMR"]
pub type R = crate::R<u32, super::TBPMR>;
#[doc = "Writer for register TBPMR"]
pub type W = crate::W<u32, super::TBPMR>;
#[doc = "Register TBPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPMR {
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
#[doc = "Reader of field `TBPSR`"]
pub type TBPSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSR`"]
pub struct TBPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&mut self) -> TBPSR_W {
        TBPSR_W { w: self }
    }
}
