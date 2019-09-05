#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Writer for register PP"]
pub type W = crate::W<u32, super::PP>;
#[doc = "Register PP `reset()`'s with value 0"]
impl crate::ResetValue for super::PP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved30`"]
pub type RESERVED30_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NB`"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\] 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
    #[inline(always)]
    pub fn sc(&mut self) -> SC_W {
        SC_W { w: self }
    }
}
