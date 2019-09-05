#[doc = "Reader of register SHIFT"]
pub type R = crate::R<u32, super::SHIFT>;
#[doc = "Writer for register SHIFT"]
pub type W = crate::W<u32, super::SHIFT>;
#[doc = "Register SHIFT `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NUM_BITS_TO_SHIFT`"]
pub type NUM_BITS_TO_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_BITS_TO_SHIFT`"]
pub struct NUM_BITS_TO_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_BITS_TO_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\] This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&self) -> NUM_BITS_TO_SHIFT_R {
        NUM_BITS_TO_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\] This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&mut self) -> NUM_BITS_TO_SHIFT_W {
        NUM_BITS_TO_SHIFT_W { w: self }
    }
}
