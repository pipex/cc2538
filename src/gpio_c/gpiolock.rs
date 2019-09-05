#[doc = "Reader of register GPIOLOCK"]
pub type R = crate::R<u32, super::GPIOLOCK>;
#[doc = "Writer for register GPIOLOCK"]
pub type W = crate::W<u32, super::GPIOLOCK>;
#[doc = "Register GPIOLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
