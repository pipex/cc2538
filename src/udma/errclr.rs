#[doc = "Reader of register ERRCLR"]
pub type R = crate::R<u32, super::ERRCLR>;
#[doc = "Writer for register ERRCLR"]
pub type W = crate::W<u32, super::ERRCLR>;
#[doc = "Register ERRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved31`"]
pub type RESERVED31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved31`"]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `ERRCLR`"]
pub type ERRCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRCLR`"]
pub struct ERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:31 - 31:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\] uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&self) -> ERRCLR_R {
        ERRCLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ERRCLR_W {
        ERRCLR_W { w: self }
    }
}
