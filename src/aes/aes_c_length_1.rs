#[doc = "Reader of register AES_C_LENGTH_1"]
pub type R = crate::R<u32, super::AES_C_LENGTH_1>;
#[doc = "Writer for register AES_C_LENGTH_1"]
pub type W = crate::W<u32, super::AES_C_LENGTH_1>;
#[doc = "Register AES_C_LENGTH_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_C_LENGTH_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `C_LENGTH`"]
pub type C_LENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `C_LENGTH`"]
pub struct C_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> C_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\] Bits should be written with a value of 0. and ignored on a read."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 0:28 - 28:0\\] C_LENGTH\\[60:32\\] Bits \\[60:0\\] of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn c_length(&self) -> C_LENGTH_R {
        C_LENGTH_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\] Bits should be written with a value of 0. and ignored on a read."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:28 - 28:0\\] C_LENGTH\\[60:32\\] Bits \\[60:0\\] of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn c_length(&mut self) -> C_LENGTH_W {
        C_LENGTH_W { w: self }
    }
}
