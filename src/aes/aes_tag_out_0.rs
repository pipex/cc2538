#[doc = "Reader of register AES_TAG_OUT_0"]
pub type R = crate::R<u32, super::AES_TAG_OUT_0>;
#[doc = "Writer for register AES_TAG_OUT_0"]
pub type W = crate::W<u32, super::AES_TAG_OUT_0>;
#[doc = "Register AES_TAG_OUT_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_TAG_OUT_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_TAG`"]
pub type AES_TAG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_TAG`"]
pub struct AES_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] AES_TAG\\[31:0\\] Bits \\[31:0\\] of the AES_TAG registers store the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&self) -> AES_TAG_R {
        AES_TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] AES_TAG\\[31:0\\] Bits \\[31:0\\] of the AES_TAG registers store the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&mut self) -> AES_TAG_W {
        AES_TAG_W { w: self }
    }
}
