#[doc = "Reader of register KEY_STORE_READ_AREA"]
pub type R = crate::R<u32, super::KEY_STORE_READ_AREA>;
#[doc = "Writer for register KEY_STORE_READ_AREA"]
pub type W = crate::W<u32, super::KEY_STORE_READ_AREA>;
#[doc = "Register KEY_STORE_READ_AREA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY_STORE_READ_AREA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
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
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 4)) | (((value as u32) & 0x07ff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA`"]
pub type RAM_AREA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAM_AREA`"]
pub struct RAM_AREA_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\] Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 4:30 - 30:4\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 4) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 4:30 - 30:4\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&mut self) -> RAM_AREA_W {
        RAM_AREA_W { w: self }
    }
}
