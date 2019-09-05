#[doc = "Reader of register CTRL_OPTIONS"]
pub type R = crate::R<u32, super::CTRL_OPTIONS>;
#[doc = "Writer for register CTRL_OPTIONS"]
pub type W = crate::W<u32, super::CTRL_OPTIONS>;
#[doc = "Register CTRL_OPTIONS `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
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
        self.w.bits = (self.w.bits & !(0x7f << 17)) | (((value as u32) & 0x7f) << 17);
        self.w
    }
}
#[doc = "Reader of field `AHBINTERFACE`"]
pub type AHBINTERFACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBINTERFACE`"]
pub struct AHBINTERFACE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBINTERFACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `SHA_256`"]
pub type SHA_256_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA_256`"]
pub struct SHA_256_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_256_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AES_CCM`"]
pub type AES_CCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_CCM`"]
pub struct AES_CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_CCM_W<'a> {
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
#[doc = "Reader of field `AES_GCM`"]
pub type AES_GCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_GCM`"]
pub struct AES_GCM_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_GCM_W<'a> {
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
#[doc = "Reader of field `AES_256`"]
pub type AES_256_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_256`"]
pub struct AES_256_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_256_W<'a> {
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
#[doc = "Reader of field `AES_128`"]
pub type AES_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_128`"]
pub struct AES_128_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_128_W<'a> {
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
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] This field is 0x01 for the TYPE1 device."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 17:23 - 23:17\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\] AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
    #[inline(always)]
    pub fn ahbinterface(&self) -> AHBINTERFACE_R {
        AHBINTERFACE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\] The HASH core supports SHA-256."]
    #[inline(always)]
    pub fn sha_256(&self) -> SHA_256_R {
        SHA_256_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] AES-CCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_ccm(&self) -> AES_CCM_R {
        AES_CCM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] AES-GCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_gcm(&self) -> AES_GCM_R {
        AES_GCM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
    #[inline(always)]
    pub fn aes_256(&self) -> AES_256_R {
        AES_256_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] AES core supports 128-bit keys."]
    #[inline(always)]
    pub fn aes_128(&self) -> AES_128_R {
        AES_128_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Bit should be ignored"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] HASH Core is available."]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] AES core is available."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] KEY STORE is available."]
    #[inline(always)]
    pub fn keystore(&self) -> KEYSTORE_R {
        KEYSTORE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\] This field is 0x01 for the TYPE1 device."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 17:23 - 23:17\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
    #[inline(always)]
    pub fn ahbinterface(&mut self) -> AHBINTERFACE_W {
        AHBINTERFACE_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] The HASH core supports SHA-256."]
    #[inline(always)]
    pub fn sha_256(&mut self) -> SHA_256_W {
        SHA_256_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] AES-CCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_ccm(&mut self) -> AES_CCM_W {
        AES_CCM_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] AES-GCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_gcm(&mut self) -> AES_GCM_W {
        AES_GCM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
    #[inline(always)]
    pub fn aes_256(&mut self) -> AES_256_W {
        AES_256_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] AES core supports 128-bit keys."]
    #[inline(always)]
    pub fn aes_128(&mut self) -> AES_128_W {
        AES_128_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Bit should be ignored"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] HASH Core is available."]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W {
        HASH_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] AES core is available."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] KEY STORE is available."]
    #[inline(always)]
    pub fn keystore(&mut self) -> KEYSTORE_W {
        KEYSTORE_W { w: self }
    }
}
