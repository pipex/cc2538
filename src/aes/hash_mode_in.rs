#[doc = "Reader of register HASH_MODE_IN"]
pub type R = crate::R<u32, super::HASH_MODE_IN>;
#[doc = "Writer for register HASH_MODE_IN"]
pub type W = crate::W<u32, super::HASH_MODE_IN>;
#[doc = "Register HASH_MODE_IN `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_MODE_IN {
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
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `SHA256_MODE`"]
pub type SHA256_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA256_MODE`"]
pub struct SHA256_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA256_MODE_W<'a> {
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
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `NEW_HASH`"]
pub type NEW_HASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEW_HASH`"]
pub struct NEW_HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> NEW_HASH_W<'a> {
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
    #[doc = "Bits 4:31 - 31:4\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\] The host must write this bit with 1 before processing a hash session."]
    #[inline(always)]
    pub fn sha256_mode(&self) -> SHA256_MODE_R {
        SHA256_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&self) -> NEW_HASH_R {
        NEW_HASH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] The host must write this bit with 1 before processing a hash session."]
    #[inline(always)]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W {
        SHA256_MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NEW_HASH_W {
        NEW_HASH_W { w: self }
    }
}
