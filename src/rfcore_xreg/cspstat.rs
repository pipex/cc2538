#[doc = "Reader of register CSPSTAT"]
pub type R = crate::R<u32, super::CSPSTAT>;
#[doc = "Writer for register CSPSTAT"]
pub type W = crate::W<u32, super::CSPSTAT>;
#[doc = "Register CSPSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CSPSTAT {
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
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CSP_RUNNING`"]
pub type CSP_RUNNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSP_RUNNING`"]
pub struct CSP_RUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_RUNNING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CSP_PC`"]
pub type CSP_PC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSP_PC`"]
pub struct CSP_PC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] 1: CSP is running. 0: CSP is idle."]
    #[inline(always)]
    pub fn csp_running(&self) -> CSP_RUNNING_R {
        CSP_RUNNING_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - 4:0\\] CSP program counter"]
    #[inline(always)]
    pub fn csp_pc(&self) -> CSP_PC_R {
        CSP_PC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 1: CSP is running. 0: CSP is idle."]
    #[inline(always)]
    pub fn csp_running(&mut self) -> CSP_RUNNING_W {
        CSP_RUNNING_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] CSP program counter"]
    #[inline(always)]
    pub fn csp_pc(&mut self) -> CSP_PC_W {
        CSP_PC_W { w: self }
    }
}
