#[doc = "Reader of register RCGCGPT"]
pub type R = crate::R<u32, super::RCGCGPT>;
#[doc = "Writer for register RCGCGPT"]
pub type W = crate::W<u32, super::RCGCGPT>;
#[doc = "Register RCGCGPT `reset()`'s with value 0"]
impl crate::ResetValue for super::RCGCGPT {
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
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `GPT3`"]
pub type GPT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT3`"]
pub struct GPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `GPT2`"]
pub type GPT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT2`"]
pub struct GPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `GPT1`"]
pub type GPT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT1`"]
pub struct GPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `GPT0`"]
pub type GPT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT0`"]
pub struct GPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT0_W<'a> {
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
    #[doc = "Bits 4:31 - 31:4\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\] 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&self) -> GPT3_R {
        GPT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&self) -> GPT2_R {
        GPT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&self) -> GPT1_R {
        GPT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    pub fn gpt0(&self) -> GPT0_R {
        GPT0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&mut self) -> GPT3_W {
        GPT3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&mut self) -> GPT2_W {
        GPT2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&mut self) -> GPT1_W {
        GPT1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    pub fn gpt0(&mut self) -> GPT0_W {
        GPT0_W { w: self }
    }
}
