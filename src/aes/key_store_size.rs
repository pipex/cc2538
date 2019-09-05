#[doc = "Reader of register KEY_STORE_SIZE"]
pub type R = crate::R<u32, super::KEY_STORE_SIZE>;
#[doc = "Writer for register KEY_STORE_SIZE"]
pub type W = crate::W<u32, super::KEY_STORE_SIZE>;
#[doc = "Register KEY_STORE_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY_STORE_SIZE {
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
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `KEY_SIZE`"]
pub type KEY_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_SIZE`"]
pub struct KEY_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\] Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W {
        KEY_SIZE_W { w: self }
    }
}
