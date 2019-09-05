#[doc = "Reader of register HASH_LENGTH_IN_L"]
pub type R = crate::R<u32, super::HASH_LENGTH_IN_L>;
#[doc = "Writer for register HASH_LENGTH_IN_L"]
pub type W = crate::W<u32, super::HASH_LENGTH_IN_L>;
#[doc = "Register HASH_LENGTH_IN_L `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_LENGTH_IN_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENGTH_IN`"]
pub type LENGTH_IN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LENGTH_IN`"]
pub struct LENGTH_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] LENGTH_IN\\[31:0\\] Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASH_LENGTH_IN_L needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the rfd_in bit of the HASH_IO_BUF_CTRL is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    pub fn length_in(&self) -> LENGTH_IN_R {
        LENGTH_IN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] LENGTH_IN\\[31:0\\] Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASH_LENGTH_IN_L needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the rfd_in bit of the HASH_IO_BUF_CTRL is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    pub fn length_in(&mut self) -> LENGTH_IN_W {
        LENGTH_IN_W { w: self }
    }
}
