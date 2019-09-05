#[doc = "Reader of register HASH_DIGEST_F"]
pub type R = crate::R<u32, super::HASH_DIGEST_F>;
#[doc = "Writer for register HASH_DIGEST_F"]
pub type W = crate::W<u32, super::HASH_DIGEST_F>;
#[doc = "Register HASH_DIGEST_F `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_DIGEST_F {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HASH_DIGEST`"]
pub type HASH_DIGEST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HASH_DIGEST`"]
pub struct HASH_DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_DIGEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] HASH_DIGEST\\[191:160\\] Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&self) -> HASH_DIGEST_R {
        HASH_DIGEST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] HASH_DIGEST\\[191:160\\] Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&mut self) -> HASH_DIGEST_W {
        HASH_DIGEST_W { w: self }
    }
}
