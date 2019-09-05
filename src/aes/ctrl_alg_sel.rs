#[doc = "Reader of register CTRL_ALG_SEL"]
pub type R = crate::R<u32, super::CTRL_ALG_SEL>;
#[doc = "Writer for register CTRL_ALG_SEL"]
pub type W = crate::W<u32, super::CTRL_ALG_SEL>;
#[doc = "Register CTRL_ALG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_ALG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
#[doc = "Reader of field `Reserved28`"]
pub type RESERVED28_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved28`"]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 3)) | (((value as u32) & 0x0fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `HASH`"]
pub type HASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASH`"]
pub struct HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_W<'a> {
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
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Reader of field `KEYSTORE`"]
pub type KEYSTORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYSTORE`"]
pub struct KEYSTORE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSTORE_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 3:30 - 30:3\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 3) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\] If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn keystore(&self) -> KEYSTORE_R {
        KEYSTORE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Bits 3:30 - 30:3\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W {
        HASH_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn keystore(&mut self) -> KEYSTORE_W {
        KEYSTORE_W { w: self }
    }
}
