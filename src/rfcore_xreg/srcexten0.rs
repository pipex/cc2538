#[doc = "Reader of register SRCEXTEN0"]
pub type R = crate::R<u32, super::SRCEXTEN0>;
#[doc = "Writer for register SRCEXTEN0"]
pub type W = crate::W<u32, super::SRCEXTEN0>;
#[doc = "Register SRCEXTEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCEXTEN0 {
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
#[doc = "Reader of field `EXT_ADDR_EN`"]
pub type EXT_ADDR_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT_ADDR_EN`"]
pub struct EXT_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADDR_EN_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\] bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\] and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\] bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\] and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W {
        EXT_ADDR_EN_W { w: self }
    }
}
