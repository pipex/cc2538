#[doc = "Reader of register PAN_ID0"]
pub type R = crate::R<u32, super::PAN_ID0>;
#[doc = "Writer for register PAN_ID0"]
pub type W = crate::W<u32, super::PAN_ID0>;
#[doc = "Register PAN_ID0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PAN_ID0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAN_ID0`"]
pub type PAN_ID0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAN_ID0`"]
pub struct PAN_ID0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAN_ID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] PAN_ID\\[7:0\\] The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id0(&self) -> PAN_ID0_R {
        PAN_ID0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] PAN_ID\\[7:0\\] The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id0(&mut self) -> PAN_ID0_W {
        PAN_ID0_W { w: self }
    }
}
