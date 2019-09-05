#[doc = "Reader of register LCTL"]
pub type R = crate::R<u32, super::LCTL>;
#[doc = "Writer for register LCTL"]
pub type W = crate::W<u32, super::LCTL>;
#[doc = "Register LCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved26`"]
pub type RESERVED26_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `BLEN`"]
pub type BLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEN`"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 4:5 - 5:4\\] Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - 3:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
}
