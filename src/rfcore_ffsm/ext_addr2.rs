#[doc = "Reader of register EXT_ADDR2"]
pub type R = crate::R<u32, super::EXT_ADDR2>;
#[doc = "Writer for register EXT_ADDR2"]
pub type W = crate::W<u32, super::EXT_ADDR2>;
#[doc = "Register EXT_ADDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_ADDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT_ADDR2`"]
pub type EXT_ADDR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT_ADDR2`"]
pub struct EXT_ADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] EXT_ADDR\\[23:16\\] The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr2(&self) -> EXT_ADDR2_R {
        EXT_ADDR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] EXT_ADDR\\[23:16\\] The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr2(&mut self) -> EXT_ADDR2_W {
        EXT_ADDR2_W { w: self }
    }
}
