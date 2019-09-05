#[doc = "Reader of register KEY_STORE_WRITTEN_AREA"]
pub type R = crate::R<u32, super::KEY_STORE_WRITTEN_AREA>;
#[doc = "Writer for register KEY_STORE_WRITTEN_AREA"]
pub type W = crate::W<u32, super::KEY_STORE_WRITTEN_AREA>;
#[doc = "Register KEY_STORE_WRITTEN_AREA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY_STORE_WRITTEN_AREA {
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
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA_WRITTEN7`"]
pub type RAM_AREA_WRITTEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN7`"]
pub struct RAM_AREA_WRITTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA_WRITTEN6`"]
pub type RAM_AREA_WRITTEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN6`"]
pub struct RAM_AREA_WRITTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA_WRITTEN5`"]
pub type RAM_AREA_WRITTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN5`"]
pub struct RAM_AREA_WRITTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN5_W<'a> {
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
#[doc = "Reader of field `RAM_AREA_WRITTEN4`"]
pub type RAM_AREA_WRITTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN4`"]
pub struct RAM_AREA_WRITTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA_WRITTEN3`"]
pub type RAM_AREA_WRITTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN3`"]
pub struct RAM_AREA_WRITTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN3_W<'a> {
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
#[doc = "Reader of field `RAM_AREA_WRITTEN2`"]
pub type RAM_AREA_WRITTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN2`"]
pub struct RAM_AREA_WRITTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN2_W<'a> {
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
#[doc = "Reader of field `RAM_AREA_WRITTEN1`"]
pub type RAM_AREA_WRITTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN1`"]
pub struct RAM_AREA_WRITTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN1_W<'a> {
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
#[doc = "Reader of field `RAM_AREA_WRITTEN0`"]
pub type RAM_AREA_WRITTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA_WRITTEN0`"]
pub struct RAM_AREA_WRITTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN0_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&self) -> RAM_AREA_WRITTEN7_R {
        RAM_AREA_WRITTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&self) -> RAM_AREA_WRITTEN6_R {
        RAM_AREA_WRITTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&self) -> RAM_AREA_WRITTEN5_R {
        RAM_AREA_WRITTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&self) -> RAM_AREA_WRITTEN4_R {
        RAM_AREA_WRITTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&self) -> RAM_AREA_WRITTEN3_R {
        RAM_AREA_WRITTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&self) -> RAM_AREA_WRITTEN2_R {
        RAM_AREA_WRITTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&self) -> RAM_AREA_WRITTEN1_R {
        RAM_AREA_WRITTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&self) -> RAM_AREA_WRITTEN0_R {
        RAM_AREA_WRITTEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&mut self) -> RAM_AREA_WRITTEN7_W {
        RAM_AREA_WRITTEN7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&mut self) -> RAM_AREA_WRITTEN6_W {
        RAM_AREA_WRITTEN6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&mut self) -> RAM_AREA_WRITTEN5_W {
        RAM_AREA_WRITTEN5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&mut self) -> RAM_AREA_WRITTEN4_W {
        RAM_AREA_WRITTEN4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&mut self) -> RAM_AREA_WRITTEN3_W {
        RAM_AREA_WRITTEN3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&mut self) -> RAM_AREA_WRITTEN2_W {
        RAM_AREA_WRITTEN2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&mut self) -> RAM_AREA_WRITTEN1_W {
        RAM_AREA_WRITTEN1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&mut self) -> RAM_AREA_WRITTEN0_W {
        RAM_AREA_WRITTEN0_W { w: self }
    }
}
