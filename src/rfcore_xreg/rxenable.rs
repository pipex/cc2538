#[doc = "Reader of register RXENABLE"]
pub type R = crate::R<u32, super::RXENABLE>;
#[doc = "Writer for register RXENABLE"]
pub type W = crate::W<u32, super::RXENABLE>;
#[doc = "Register RXENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::RXENABLE {
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
#[doc = "Reader of field `RXENMASK`"]
pub type RXENMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXENMASK`"]
pub struct RXENMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
    #[inline(always)]
    pub fn rxenmask(&self) -> RXENMASK_R {
        RXENMASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
    #[inline(always)]
    pub fn rxenmask(&mut self) -> RXENMASK_W {
        RXENMASK_W { w: self }
    }
}
