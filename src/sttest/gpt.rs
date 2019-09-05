#[doc = "Reader of register GPT"]
pub type R = crate::R<u32, super::GPT>;
#[doc = "Writer for register GPT"]
pub type W = crate::W<u32, super::GPT>;
#[doc = "Register GPT `reset()`'s with value 0"]
impl crate::ResetValue for super::GPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `GPTIDOE`"]
pub type GPTIDOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTIDOE`"]
pub struct GPTIDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTIDOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `GPTIDOV`"]
pub type GPTIDOV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTIDOV`"]
pub struct GPTIDOV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTIDOV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\] GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&self) -> GPTIDOE_R {
        GPTIDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\] GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&self) -> GPTIDOV_R {
        GPTIDOV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&mut self) -> GPTIDOE_W {
        GPTIDOE_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&mut self) -> GPTIDOV_W {
        GPTIDOV_W { w: self }
    }
}
