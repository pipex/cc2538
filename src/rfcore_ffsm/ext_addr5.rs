#[doc = "Reader of register EXT_ADDR5"]
pub type R = crate::R<u32, super::EXT_ADDR5>;
#[doc = "Writer for register EXT_ADDR5"]
pub type W = crate::W<u32, super::EXT_ADDR5>;
#[doc = "Register EXT_ADDR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_ADDR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT_ADDR5`"]
pub type EXT_ADDR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT_ADDR5`"]
pub struct EXT_ADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] EXT_ADDR\\[47:40\\] The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr5(&self) -> EXT_ADDR5_R {
        EXT_ADDR5_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] EXT_ADDR\\[47:40\\] The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr5(&mut self) -> EXT_ADDR5_W {
        EXT_ADDR5_W { w: self }
    }
}
