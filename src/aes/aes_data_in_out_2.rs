#[doc = "Reader of register AES_DATA_IN_OUT_2"]
pub type R = crate::R<u32, super::AES_DATA_IN_OUT_2>;
#[doc = "Writer for register AES_DATA_IN_OUT_2"]
pub type W = crate::W<u32, super::AES_DATA_IN_OUT_2>;
#[doc = "Register AES_DATA_IN_OUT_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_DATA_IN_OUT_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_DATA_IN_OUT`"]
pub type AES_DATA_IN_OUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_DATA_IN_OUT`"]
pub struct AES_DATA_IN_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_DATA_IN_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] AES input data\\[95:64\\] / AES output data\\[95:64\\] Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub fn aes_data_in_out(&self) -> AES_DATA_IN_OUT_R {
        AES_DATA_IN_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] AES input data\\[95:64\\] / AES output data\\[95:64\\] Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub fn aes_data_in_out(&mut self) -> AES_DATA_IN_OUT_W {
        AES_DATA_IN_OUT_W { w: self }
    }
}
