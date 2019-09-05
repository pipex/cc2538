#[doc = "Reader of register AES_KEY2_0"]
pub type R = crate::R<u32, super::AES_KEY2_0>;
#[doc = "Writer for register AES_KEY2_0"]
pub type W = crate::W<u32, super::AES_KEY2_0>;
#[doc = "Register AES_KEY2_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_KEY2_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_KEY2`"]
pub type AES_KEY2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_KEY2`"]
pub struct AES_KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] AES_KEY2/AES_GHASH_H\\[31:0\\] For GCM: -\\[127:0\\] - GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\] - This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\] - This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\] - ZEROES - This register must remain 0."]
    #[inline(always)]
    pub fn aes_key2(&self) -> AES_KEY2_R {
        AES_KEY2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] AES_KEY2/AES_GHASH_H\\[31:0\\] For GCM: -\\[127:0\\] - GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\] - This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\] - This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\] - ZEROES - This register must remain 0."]
    #[inline(always)]
    pub fn aes_key2(&mut self) -> AES_KEY2_W {
        AES_KEY2_W { w: self }
    }
}
