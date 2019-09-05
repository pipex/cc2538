#[doc = "Reader of register AES_AUTH_LENGTH"]
pub type R = crate::R<u32, super::AES_AUTH_LENGTH>;
#[doc = "Writer for register AES_AUTH_LENGTH"]
pub type W = crate::W<u32, super::AES_AUTH_LENGTH>;
#[doc = "Register AES_AUTH_LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_AUTH_LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTH_LENGTH`"]
pub type AUTH_LENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AUTH_LENGTH`"]
pub struct AUTH_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTH_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Bits \\[31:0\\] of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn auth_length(&self) -> AUTH_LENGTH_R {
        AUTH_LENGTH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Bits \\[31:0\\] of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn auth_length(&mut self) -> AUTH_LENGTH_W {
        AUTH_LENGTH_W { w: self }
    }
}
