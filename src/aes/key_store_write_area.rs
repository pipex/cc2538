#[doc = "Reader of register KEY_STORE_WRITE_AREA"]
pub type R = crate::R<u32, super::KEY_STORE_WRITE_AREA>;
#[doc = "Writer for register KEY_STORE_WRITE_AREA"]
pub type W = crate::W<u32, super::KEY_STORE_WRITE_AREA>;
#[doc = "Register KEY_STORE_WRITE_AREA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY_STORE_WRITE_AREA {
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
#[doc = "Reader of field `RAM_AREA7`"]
pub type RAM_AREA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA7`"]
pub struct RAM_AREA7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA7_W<'a> {
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
#[doc = "Reader of field `RAM_AREA6`"]
pub type RAM_AREA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA6`"]
pub struct RAM_AREA6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA6_W<'a> {
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
#[doc = "Reader of field `RAM_AREA5`"]
pub type RAM_AREA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA5`"]
pub struct RAM_AREA5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA5_W<'a> {
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
#[doc = "Reader of field `RAM_AREA4`"]
pub type RAM_AREA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA4`"]
pub struct RAM_AREA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA4_W<'a> {
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
#[doc = "Reader of field `RAM_AREA3`"]
pub type RAM_AREA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA3`"]
pub struct RAM_AREA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA3_W<'a> {
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
#[doc = "Reader of field `RAM_AREA2`"]
pub type RAM_AREA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA2`"]
pub struct RAM_AREA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA2_W<'a> {
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
#[doc = "Reader of field `RAM_AREA1`"]
pub type RAM_AREA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA1`"]
pub struct RAM_AREA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA1_W<'a> {
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
#[doc = "Reader of field `RAM_AREA0`"]
pub type RAM_AREA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA0`"]
pub struct RAM_AREA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA0_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W {
        RAM_AREA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W {
        RAM_AREA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W {
        RAM_AREA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W {
        RAM_AREA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W {
        RAM_AREA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W {
        RAM_AREA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W {
        RAM_AREA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W {
        RAM_AREA0_W { w: self }
    }
}
