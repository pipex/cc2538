#[doc = "Reader of register IO"]
pub type R = crate::R<u32, super::IO>;
#[doc = "Writer for register IO"]
pub type W = crate::W<u32, super::IO>;
#[doc = "Register IO `reset()`'s with value 0"]
impl crate::ResetValue for super::IO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SC`"]
pub type SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SC`"]
pub struct SC_W<'a> {
    w: &'a mut W,
}
impl<'a> SC_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\] I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\] I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    pub fn sc(&mut self) -> SC_W {
        SC_W { w: self }
    }
}
