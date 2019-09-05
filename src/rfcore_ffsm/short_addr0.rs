#[doc = "Reader of register SHORT_ADDR0"]
pub type R = crate::R<u32, super::SHORT_ADDR0>;
#[doc = "Writer for register SHORT_ADDR0"]
pub type W = crate::W<u32, super::SHORT_ADDR0>;
#[doc = "Register SHORT_ADDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORT_ADDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHORT_ADDR0`"]
pub type SHORT_ADDR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHORT_ADDR0`"]
pub struct SHORT_ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORT_ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] SHORT_ADDR\\[7:0\\] The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&self) -> SHORT_ADDR0_R {
        SHORT_ADDR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] SHORT_ADDR\\[7:0\\] The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&mut self) -> SHORT_ADDR0_W {
        SHORT_ADDR0_W { w: self }
    }
}
