#[doc = "Reader of register NINEBITADDR"]
pub type R = crate::R<u32, super::NINEBITADDR>;
#[doc = "Writer for register NINEBITADDR"]
pub type W = crate::W<u32, super::NINEBITADDR>;
#[doc = "Register NINEBITADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::NINEBITADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NINEBITEN`"]
pub type NINEBITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINEBITEN`"]
pub struct NINEBITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `Reserved7`"]
pub type RESERVED7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\] Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&self) -> NINEBITEN_R {
        NINEBITEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - 14:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\] Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&mut self) -> NINEBITEN_W {
        NINEBITEN_W { w: self }
    }
    #[doc = "Bits 8:14 - 14:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
