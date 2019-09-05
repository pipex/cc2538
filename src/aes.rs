#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmac_ch0_ctrl: DMAC_CH0_CTRL,
    #[doc = "0x04 - Channel external address"]
    pub dmac_ch0_extaddr: DMAC_CH0_EXTADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Channel DMA length"]
    pub dmac_ch0_dmalength: DMAC_CH0_DMALENGTH,
    _reserved3: [u8; 8usize],
    #[doc = "0x18 - DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    pub dmac_status: DMAC_STATUS,
    #[doc = "0x1c - DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
    pub dmac_swres: DMAC_SWRES,
    #[doc = "0x20 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmac_ch1_ctrl: DMAC_CH1_CTRL,
    #[doc = "0x24 - Channel external address"]
    pub dmac_ch1_extaddr: DMAC_CH1_EXTADDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - Channel DMA length"]
    pub dmac_ch1_dmalength: DMAC_CH1_DMALENGTH,
    _reserved8: [u8; 72usize],
    #[doc = "0x78 - DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    pub dmac_mst_runparams: DMAC_MST_RUNPARAMS,
    #[doc = "0x7c - DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
    pub dmac_persr: DMAC_PERSR,
    _reserved10: [u8; 120usize],
    #[doc = "0xf8 - DMAC options register These registers contain information regarding the different options configured in this DMAC."]
    pub dmac_options: DMAC_OPTIONS,
    #[doc = "0xfc - DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    pub dmac_version: DMAC_VERSION,
    _reserved12: [u8; 768usize],
    #[doc = "0x400 - Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    pub key_store_write_area: KEY_STORE_WRITE_AREA,
    #[doc = "0x404 - Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    pub key_store_written_area: KEY_STORE_WRITTEN_AREA,
    #[doc = "0x408 - Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    pub key_store_size: KEY_STORE_SIZE,
    #[doc = "0x40c - Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    pub key_store_read_area: KEY_STORE_READ_AREA,
    _reserved16: [u8; 240usize],
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_0: AES_KEY2_0,
    #[doc = "0x504 - AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_1: AES_KEY2_1,
    #[doc = "0x508 - AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_2: AES_KEY2_2,
    #[doc = "0x50c - AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_3: AES_KEY2_3,
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_0: AES_KEY3_0,
    #[doc = "0x514 - AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_1: AES_KEY3_1,
    #[doc = "0x518 - AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_2: AES_KEY3_2,
    #[doc = "0x51c - AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_3: AES_KEY3_3,
    _reserved24: [u8; 32usize],
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_0: AES_IV_0,
    #[doc = "0x544 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_1: AES_IV_1,
    #[doc = "0x548 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_2: AES_IV_2,
    #[doc = "0x54c - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_3: AES_IV_3,
    #[doc = "0x550 - AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0."]
    pub aes_ctrl: AES_CTRL,
    #[doc = "0x554 - AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aes_c_length_0: AES_C_LENGTH_0,
    #[doc = "0x558 - AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aes_c_length_1: AES_C_LENGTH_1,
    #[doc = "0x55c - Authentication length register"]
    pub aes_auth_length: AES_AUTH_LENGTH,
    #[doc = "0x560 - Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_0: AES_DATA_IN_OUT_0,
    #[doc = "0x564 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_1: AES_DATA_IN_OUT_1,
    #[doc = "0x568 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_2: AES_DATA_IN_OUT_2,
    #[doc = "0x56c - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_3: AES_DATA_IN_OUT_3,
    #[doc = "0x570 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    pub aes_tag_out_0: AES_TAG_OUT_0,
    #[doc = "0x574 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_1: AES_TAG_OUT_1,
    #[doc = "0x578 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_2: AES_TAG_OUT_2,
    #[doc = "0x57c - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_3: AES_TAG_OUT_3,
    _reserved40: [u8; 128usize],
    #[doc = "0x600 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_0: HASH_DATA_IN_0,
    #[doc = "0x604 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_1: HASH_DATA_IN_1,
    #[doc = "0x608 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_2: HASH_DATA_IN_2,
    #[doc = "0x60c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_3: HASH_DATA_IN_3,
    #[doc = "0x610 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_4: HASH_DATA_IN_4,
    #[doc = "0x614 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_5: HASH_DATA_IN_5,
    #[doc = "0x618 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_6: HASH_DATA_IN_6,
    #[doc = "0x61c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_7: HASH_DATA_IN_7,
    #[doc = "0x620 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_8: HASH_DATA_IN_8,
    #[doc = "0x624 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_9: HASH_DATA_IN_9,
    #[doc = "0x628 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_10: HASH_DATA_IN_10,
    #[doc = "0x62c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_11: HASH_DATA_IN_11,
    #[doc = "0x630 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_12: HASH_DATA_IN_12,
    #[doc = "0x634 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_13: HASH_DATA_IN_13,
    #[doc = "0x638 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_14: HASH_DATA_IN_14,
    #[doc = "0x63c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_15: HASH_DATA_IN_15,
    #[doc = "0x640 - Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    pub hash_io_buf_ctrl: HASH_IO_BUF_CTRL,
    #[doc = "0x644 - Hash mode register"]
    pub hash_mode_in: HASH_MODE_IN,
    #[doc = "0x648 - Hash length register"]
    pub hash_length_in_l: HASH_LENGTH_IN_L,
    #[doc = "0x64c - Hash length register"]
    pub hash_length_in_h: HASH_LENGTH_IN_H,
    #[doc = "0x650 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_a: HASH_DIGEST_A,
    #[doc = "0x654 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_b: HASH_DIGEST_B,
    #[doc = "0x658 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_c: HASH_DIGEST_C,
    #[doc = "0x65c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_d: HASH_DIGEST_D,
    #[doc = "0x660 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_e: HASH_DIGEST_E,
    #[doc = "0x664 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_f: HASH_DIGEST_F,
    #[doc = "0x668 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_g: HASH_DIGEST_G,
    #[doc = "0x66c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_h: HASH_DIGEST_H,
    _reserved68: [u8; 144usize],
    #[doc = "0x700 - Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
    pub ctrl_alg_sel: CTRL_ALG_SEL,
    #[doc = "0x704 - Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    pub ctrl_prot_en: CTRL_PROT_EN,
    _reserved70: [u8; 56usize],
    #[doc = "0x740 - Software reset"]
    pub ctrl_sw_reset: CTRL_SW_RESET,
    _reserved71: [u8; 60usize],
    #[doc = "0x780 - Interrupt configuration"]
    pub ctrl_int_cfg: CTRL_INT_CFG,
    #[doc = "0x784 - Interrupt enable"]
    pub ctrl_int_en: CTRL_INT_EN,
    #[doc = "0x788 - Interrupt clear"]
    pub ctrl_int_clr: CTRL_INT_CLR,
    #[doc = "0x78c - Interrupt set"]
    pub ctrl_int_set: CTRL_INT_SET,
    #[doc = "0x790 - Interrupt status"]
    pub ctrl_int_stat: CTRL_INT_STAT,
    _reserved76: [u8; 100usize],
    #[doc = "0x7f8 - Options register"]
    pub ctrl_options: CTRL_OPTIONS,
    #[doc = "0x7fc - Version register"]
    pub ctrl_version: CTRL_VERSION,
}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch0_ctrl](dmac_ch0_ctrl) module"]
pub type DMAC_CH0_CTRL = crate::Reg<u32, _DMAC_CH0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH0_CTRL;
#[doc = "`read()` method returns [dmac_ch0_ctrl::R](dmac_ch0_ctrl::R) reader structure"]
impl crate::Readable for DMAC_CH0_CTRL {}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_ctrl::W](dmac_ch0_ctrl::W) writer structure"]
impl crate::Writable for DMAC_CH0_CTRL {}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch0_ctrl;
#[doc = "Channel external address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch0_extaddr](dmac_ch0_extaddr) module"]
pub type DMAC_CH0_EXTADDR = crate::Reg<u32, _DMAC_CH0_EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH0_EXTADDR;
#[doc = "`read()` method returns [dmac_ch0_extaddr::R](dmac_ch0_extaddr::R) reader structure"]
impl crate::Readable for DMAC_CH0_EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_extaddr::W](dmac_ch0_extaddr::W) writer structure"]
impl crate::Writable for DMAC_CH0_EXTADDR {}
#[doc = "Channel external address"]
pub mod dmac_ch0_extaddr;
#[doc = "Channel DMA length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch0_dmalength](dmac_ch0_dmalength) module"]
pub type DMAC_CH0_DMALENGTH = crate::Reg<u32, _DMAC_CH0_DMALENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH0_DMALENGTH;
#[doc = "`read()` method returns [dmac_ch0_dmalength::R](dmac_ch0_dmalength::R) reader structure"]
impl crate::Readable for DMAC_CH0_DMALENGTH {}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_dmalength::W](dmac_ch0_dmalength::W) writer structure"]
impl crate::Writable for DMAC_CH0_DMALENGTH {}
#[doc = "Channel DMA length"]
pub mod dmac_ch0_dmalength;
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_status](dmac_status) module"]
pub type DMAC_STATUS = crate::Reg<u32, _DMAC_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_STATUS;
#[doc = "`read()` method returns [dmac_status::R](dmac_status::R) reader structure"]
impl crate::Readable for DMAC_STATUS {}
#[doc = "`write(|w| ..)` method takes [dmac_status::W](dmac_status::W) writer structure"]
impl crate::Writable for DMAC_STATUS {}
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmac_status;
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_swres](dmac_swres) module"]
pub type DMAC_SWRES = crate::Reg<u32, _DMAC_SWRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_SWRES;
#[doc = "`read()` method returns [dmac_swres::R](dmac_swres::R) reader structure"]
impl crate::Readable for DMAC_SWRES {}
#[doc = "`write(|w| ..)` method takes [dmac_swres::W](dmac_swres::W) writer structure"]
impl crate::Writable for DMAC_SWRES {}
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
pub mod dmac_swres;
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch1_ctrl](dmac_ch1_ctrl) module"]
pub type DMAC_CH1_CTRL = crate::Reg<u32, _DMAC_CH1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH1_CTRL;
#[doc = "`read()` method returns [dmac_ch1_ctrl::R](dmac_ch1_ctrl::R) reader structure"]
impl crate::Readable for DMAC_CH1_CTRL {}
#[doc = "`write(|w| ..)` method takes [dmac_ch1_ctrl::W](dmac_ch1_ctrl::W) writer structure"]
impl crate::Writable for DMAC_CH1_CTRL {}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch1_ctrl;
#[doc = "Channel external address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch1_extaddr](dmac_ch1_extaddr) module"]
pub type DMAC_CH1_EXTADDR = crate::Reg<u32, _DMAC_CH1_EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH1_EXTADDR;
#[doc = "`read()` method returns [dmac_ch1_extaddr::R](dmac_ch1_extaddr::R) reader structure"]
impl crate::Readable for DMAC_CH1_EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmac_ch1_extaddr::W](dmac_ch1_extaddr::W) writer structure"]
impl crate::Writable for DMAC_CH1_EXTADDR {}
#[doc = "Channel external address"]
pub mod dmac_ch1_extaddr;
#[doc = "Channel DMA length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_ch1_dmalength](dmac_ch1_dmalength) module"]
pub type DMAC_CH1_DMALENGTH = crate::Reg<u32, _DMAC_CH1_DMALENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_CH1_DMALENGTH;
#[doc = "`read()` method returns [dmac_ch1_dmalength::R](dmac_ch1_dmalength::R) reader structure"]
impl crate::Readable for DMAC_CH1_DMALENGTH {}
#[doc = "`write(|w| ..)` method takes [dmac_ch1_dmalength::W](dmac_ch1_dmalength::W) writer structure"]
impl crate::Writable for DMAC_CH1_DMALENGTH {}
#[doc = "Channel DMA length"]
pub mod dmac_ch1_dmalength;
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_mst_runparams](dmac_mst_runparams) module"]
pub type DMAC_MST_RUNPARAMS = crate::Reg<u32, _DMAC_MST_RUNPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_MST_RUNPARAMS;
#[doc = "`read()` method returns [dmac_mst_runparams::R](dmac_mst_runparams::R) reader structure"]
impl crate::Readable for DMAC_MST_RUNPARAMS {}
#[doc = "`write(|w| ..)` method takes [dmac_mst_runparams::W](dmac_mst_runparams::W) writer structure"]
impl crate::Writable for DMAC_MST_RUNPARAMS {}
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmac_mst_runparams;
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_persr](dmac_persr) module"]
pub type DMAC_PERSR = crate::Reg<u32, _DMAC_PERSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_PERSR;
#[doc = "`read()` method returns [dmac_persr::R](dmac_persr::R) reader structure"]
impl crate::Readable for DMAC_PERSR {}
#[doc = "`write(|w| ..)` method takes [dmac_persr::W](dmac_persr::W) writer structure"]
impl crate::Writable for DMAC_PERSR {}
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
pub mod dmac_persr;
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_options](dmac_options) module"]
pub type DMAC_OPTIONS = crate::Reg<u32, _DMAC_OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_OPTIONS;
#[doc = "`read()` method returns [dmac_options::R](dmac_options::R) reader structure"]
impl crate::Readable for DMAC_OPTIONS {}
#[doc = "`write(|w| ..)` method takes [dmac_options::W](dmac_options::W) writer structure"]
impl crate::Writable for DMAC_OPTIONS {}
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC."]
pub mod dmac_options;
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmac_version](dmac_version) module"]
pub type DMAC_VERSION = crate::Reg<u32, _DMAC_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_VERSION;
#[doc = "`read()` method returns [dmac_version::R](dmac_version::R) reader structure"]
impl crate::Readable for DMAC_VERSION {}
#[doc = "`write(|w| ..)` method takes [dmac_version::W](dmac_version::W) writer structure"]
impl crate::Writable for DMAC_VERSION {}
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmac_version;
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_store_write_area](key_store_write_area) module"]
pub type KEY_STORE_WRITE_AREA = crate::Reg<u32, _KEY_STORE_WRITE_AREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_STORE_WRITE_AREA;
#[doc = "`read()` method returns [key_store_write_area::R](key_store_write_area::R) reader structure"]
impl crate::Readable for KEY_STORE_WRITE_AREA {}
#[doc = "`write(|w| ..)` method takes [key_store_write_area::W](key_store_write_area::W) writer structure"]
impl crate::Writable for KEY_STORE_WRITE_AREA {}
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod key_store_write_area;
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_store_written_area](key_store_written_area) module"]
pub type KEY_STORE_WRITTEN_AREA = crate::Reg<u32, _KEY_STORE_WRITTEN_AREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_STORE_WRITTEN_AREA;
#[doc = "`read()` method returns [key_store_written_area::R](key_store_written_area::R) reader structure"]
impl crate::Readable for KEY_STORE_WRITTEN_AREA {}
#[doc = "`write(|w| ..)` method takes [key_store_written_area::W](key_store_written_area::W) writer structure"]
impl crate::Writable for KEY_STORE_WRITTEN_AREA {}
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod key_store_written_area;
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_store_size](key_store_size) module"]
pub type KEY_STORE_SIZE = crate::Reg<u32, _KEY_STORE_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_STORE_SIZE;
#[doc = "`read()` method returns [key_store_size::R](key_store_size::R) reader structure"]
impl crate::Readable for KEY_STORE_SIZE {}
#[doc = "`write(|w| ..)` method takes [key_store_size::W](key_store_size::W) writer structure"]
impl crate::Writable for KEY_STORE_SIZE {}
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod key_store_size;
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_store_read_area](key_store_read_area) module"]
pub type KEY_STORE_READ_AREA = crate::Reg<u32, _KEY_STORE_READ_AREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_STORE_READ_AREA;
#[doc = "`read()` method returns [key_store_read_area::R](key_store_read_area::R) reader structure"]
impl crate::Readable for KEY_STORE_READ_AREA {}
#[doc = "`write(|w| ..)` method takes [key_store_read_area::W](key_store_read_area::W) writer structure"]
impl crate::Writable for KEY_STORE_READ_AREA {}
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod key_store_read_area;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key2_0](aes_key2_0) module"]
pub type AES_KEY2_0 = crate::Reg<u32, _AES_KEY2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY2_0;
#[doc = "`read()` method returns [aes_key2_0::R](aes_key2_0::R) reader structure"]
impl crate::Readable for AES_KEY2_0 {}
#[doc = "`write(|w| ..)` method takes [aes_key2_0::W](aes_key2_0::W) writer structure"]
impl crate::Writable for AES_KEY2_0 {}
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_0;
#[doc = "AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key2_1](aes_key2_1) module"]
pub type AES_KEY2_1 = crate::Reg<u32, _AES_KEY2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY2_1;
#[doc = "`read()` method returns [aes_key2_1::R](aes_key2_1::R) reader structure"]
impl crate::Readable for AES_KEY2_1 {}
#[doc = "`write(|w| ..)` method takes [aes_key2_1::W](aes_key2_1::W) writer structure"]
impl crate::Writable for AES_KEY2_1 {}
#[doc = "AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_1;
#[doc = "AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key2_2](aes_key2_2) module"]
pub type AES_KEY2_2 = crate::Reg<u32, _AES_KEY2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY2_2;
#[doc = "`read()` method returns [aes_key2_2::R](aes_key2_2::R) reader structure"]
impl crate::Readable for AES_KEY2_2 {}
#[doc = "`write(|w| ..)` method takes [aes_key2_2::W](aes_key2_2::W) writer structure"]
impl crate::Writable for AES_KEY2_2 {}
#[doc = "AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_2;
#[doc = "AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key2_3](aes_key2_3) module"]
pub type AES_KEY2_3 = crate::Reg<u32, _AES_KEY2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY2_3;
#[doc = "`read()` method returns [aes_key2_3::R](aes_key2_3::R) reader structure"]
impl crate::Readable for AES_KEY2_3 {}
#[doc = "`write(|w| ..)` method takes [aes_key2_3::W](aes_key2_3::W) writer structure"]
impl crate::Writable for AES_KEY2_3 {}
#[doc = "AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_3;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key3_0](aes_key3_0) module"]
pub type AES_KEY3_0 = crate::Reg<u32, _AES_KEY3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY3_0;
#[doc = "`read()` method returns [aes_key3_0::R](aes_key3_0::R) reader structure"]
impl crate::Readable for AES_KEY3_0 {}
#[doc = "`write(|w| ..)` method takes [aes_key3_0::W](aes_key3_0::W) writer structure"]
impl crate::Writable for AES_KEY3_0 {}
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_0;
#[doc = "AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key3_1](aes_key3_1) module"]
pub type AES_KEY3_1 = crate::Reg<u32, _AES_KEY3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY3_1;
#[doc = "`read()` method returns [aes_key3_1::R](aes_key3_1::R) reader structure"]
impl crate::Readable for AES_KEY3_1 {}
#[doc = "`write(|w| ..)` method takes [aes_key3_1::W](aes_key3_1::W) writer structure"]
impl crate::Writable for AES_KEY3_1 {}
#[doc = "AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_1;
#[doc = "AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key3_2](aes_key3_2) module"]
pub type AES_KEY3_2 = crate::Reg<u32, _AES_KEY3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY3_2;
#[doc = "`read()` method returns [aes_key3_2::R](aes_key3_2::R) reader structure"]
impl crate::Readable for AES_KEY3_2 {}
#[doc = "`write(|w| ..)` method takes [aes_key3_2::W](aes_key3_2::W) writer structure"]
impl crate::Writable for AES_KEY3_2 {}
#[doc = "AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_2;
#[doc = "AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_key3_3](aes_key3_3) module"]
pub type AES_KEY3_3 = crate::Reg<u32, _AES_KEY3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY3_3;
#[doc = "`read()` method returns [aes_key3_3::R](aes_key3_3::R) reader structure"]
impl crate::Readable for AES_KEY3_3 {}
#[doc = "`write(|w| ..)` method takes [aes_key3_3::W](aes_key3_3::W) writer structure"]
impl crate::Writable for AES_KEY3_3 {}
#[doc = "AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_3;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_iv_0](aes_iv_0) module"]
pub type AES_IV_0 = crate::Reg<u32, _AES_IV_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IV_0;
#[doc = "`read()` method returns [aes_iv_0::R](aes_iv_0::R) reader structure"]
impl crate::Readable for AES_IV_0 {}
#[doc = "`write(|w| ..)` method takes [aes_iv_0::W](aes_iv_0::W) writer structure"]
impl crate::Writable for AES_IV_0 {}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_0;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_iv_1](aes_iv_1) module"]
pub type AES_IV_1 = crate::Reg<u32, _AES_IV_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IV_1;
#[doc = "`read()` method returns [aes_iv_1::R](aes_iv_1::R) reader structure"]
impl crate::Readable for AES_IV_1 {}
#[doc = "`write(|w| ..)` method takes [aes_iv_1::W](aes_iv_1::W) writer structure"]
impl crate::Writable for AES_IV_1 {}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_1;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_iv_2](aes_iv_2) module"]
pub type AES_IV_2 = crate::Reg<u32, _AES_IV_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IV_2;
#[doc = "`read()` method returns [aes_iv_2::R](aes_iv_2::R) reader structure"]
impl crate::Readable for AES_IV_2 {}
#[doc = "`write(|w| ..)` method takes [aes_iv_2::W](aes_iv_2::W) writer structure"]
impl crate::Writable for AES_IV_2 {}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_2;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_iv_3](aes_iv_3) module"]
pub type AES_IV_3 = crate::Reg<u32, _AES_IV_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IV_3;
#[doc = "`read()` method returns [aes_iv_3::R](aes_iv_3::R) reader structure"]
impl crate::Readable for AES_IV_3 {}
#[doc = "`write(|w| ..)` method takes [aes_iv_3::W](aes_iv_3::W) writer structure"]
impl crate::Writable for AES_IV_3 {}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_3;
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_ctrl](aes_ctrl) module"]
pub type AES_CTRL = crate::Reg<u32, _AES_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_CTRL;
#[doc = "`read()` method returns [aes_ctrl::R](aes_ctrl::R) reader structure"]
impl crate::Readable for AES_CTRL {}
#[doc = "`write(|w| ..)` method takes [aes_ctrl::W](aes_ctrl::W) writer structure"]
impl crate::Writable for AES_CTRL {}
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0."]
pub mod aes_ctrl;
#[doc = "AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_c_length_0](aes_c_length_0) module"]
pub type AES_C_LENGTH_0 = crate::Reg<u32, _AES_C_LENGTH_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_C_LENGTH_0;
#[doc = "`read()` method returns [aes_c_length_0::R](aes_c_length_0::R) reader structure"]
impl crate::Readable for AES_C_LENGTH_0 {}
#[doc = "`write(|w| ..)` method takes [aes_c_length_0::W](aes_c_length_0::W) writer structure"]
impl crate::Writable for AES_C_LENGTH_0 {}
#[doc = "AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_0;
#[doc = "AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_c_length_1](aes_c_length_1) module"]
pub type AES_C_LENGTH_1 = crate::Reg<u32, _AES_C_LENGTH_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_C_LENGTH_1;
#[doc = "`read()` method returns [aes_c_length_1::R](aes_c_length_1::R) reader structure"]
impl crate::Readable for AES_C_LENGTH_1 {}
#[doc = "`write(|w| ..)` method takes [aes_c_length_1::W](aes_c_length_1::W) writer structure"]
impl crate::Writable for AES_C_LENGTH_1 {}
#[doc = "AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_1;
#[doc = "Authentication length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_auth_length](aes_auth_length) module"]
pub type AES_AUTH_LENGTH = crate::Reg<u32, _AES_AUTH_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_AUTH_LENGTH;
#[doc = "`read()` method returns [aes_auth_length::R](aes_auth_length::R) reader structure"]
impl crate::Readable for AES_AUTH_LENGTH {}
#[doc = "`write(|w| ..)` method takes [aes_auth_length::W](aes_auth_length::W) writer structure"]
impl crate::Writable for AES_AUTH_LENGTH {}
#[doc = "Authentication length register"]
pub mod aes_auth_length;
#[doc = "Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_data_in_out_0](aes_data_in_out_0) module"]
pub type AES_DATA_IN_OUT_0 = crate::Reg<u32, _AES_DATA_IN_OUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DATA_IN_OUT_0;
#[doc = "`read()` method returns [aes_data_in_out_0::R](aes_data_in_out_0::R) reader structure"]
impl crate::Readable for AES_DATA_IN_OUT_0 {}
#[doc = "`write(|w| ..)` method takes [aes_data_in_out_0::W](aes_data_in_out_0::W) writer structure"]
impl crate::Writable for AES_DATA_IN_OUT_0 {}
#[doc = "Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_0;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_data_in_out_1](aes_data_in_out_1) module"]
pub type AES_DATA_IN_OUT_1 = crate::Reg<u32, _AES_DATA_IN_OUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DATA_IN_OUT_1;
#[doc = "`read()` method returns [aes_data_in_out_1::R](aes_data_in_out_1::R) reader structure"]
impl crate::Readable for AES_DATA_IN_OUT_1 {}
#[doc = "`write(|w| ..)` method takes [aes_data_in_out_1::W](aes_data_in_out_1::W) writer structure"]
impl crate::Writable for AES_DATA_IN_OUT_1 {}
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_1;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_data_in_out_2](aes_data_in_out_2) module"]
pub type AES_DATA_IN_OUT_2 = crate::Reg<u32, _AES_DATA_IN_OUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DATA_IN_OUT_2;
#[doc = "`read()` method returns [aes_data_in_out_2::R](aes_data_in_out_2::R) reader structure"]
impl crate::Readable for AES_DATA_IN_OUT_2 {}
#[doc = "`write(|w| ..)` method takes [aes_data_in_out_2::W](aes_data_in_out_2::W) writer structure"]
impl crate::Writable for AES_DATA_IN_OUT_2 {}
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_2;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_data_in_out_3](aes_data_in_out_3) module"]
pub type AES_DATA_IN_OUT_3 = crate::Reg<u32, _AES_DATA_IN_OUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_DATA_IN_OUT_3;
#[doc = "`read()` method returns [aes_data_in_out_3::R](aes_data_in_out_3::R) reader structure"]
impl crate::Readable for AES_DATA_IN_OUT_3 {}
#[doc = "`write(|w| ..)` method takes [aes_data_in_out_3::W](aes_data_in_out_3::W) writer structure"]
impl crate::Writable for AES_DATA_IN_OUT_3 {}
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_3;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_tag_out_0](aes_tag_out_0) module"]
pub type AES_TAG_OUT_0 = crate::Reg<u32, _AES_TAG_OUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_TAG_OUT_0;
#[doc = "`read()` method returns [aes_tag_out_0::R](aes_tag_out_0::R) reader structure"]
impl crate::Readable for AES_TAG_OUT_0 {}
#[doc = "`write(|w| ..)` method takes [aes_tag_out_0::W](aes_tag_out_0::W) writer structure"]
impl crate::Writable for AES_TAG_OUT_0 {}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aes_tag_out_0;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_tag_out_1](aes_tag_out_1) module"]
pub type AES_TAG_OUT_1 = crate::Reg<u32, _AES_TAG_OUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_TAG_OUT_1;
#[doc = "`read()` method returns [aes_tag_out_1::R](aes_tag_out_1::R) reader structure"]
impl crate::Readable for AES_TAG_OUT_1 {}
#[doc = "`write(|w| ..)` method takes [aes_tag_out_1::W](aes_tag_out_1::W) writer structure"]
impl crate::Writable for AES_TAG_OUT_1 {}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_1;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_tag_out_2](aes_tag_out_2) module"]
pub type AES_TAG_OUT_2 = crate::Reg<u32, _AES_TAG_OUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_TAG_OUT_2;
#[doc = "`read()` method returns [aes_tag_out_2::R](aes_tag_out_2::R) reader structure"]
impl crate::Readable for AES_TAG_OUT_2 {}
#[doc = "`write(|w| ..)` method takes [aes_tag_out_2::W](aes_tag_out_2::W) writer structure"]
impl crate::Writable for AES_TAG_OUT_2 {}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_2;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aes_tag_out_3](aes_tag_out_3) module"]
pub type AES_TAG_OUT_3 = crate::Reg<u32, _AES_TAG_OUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_TAG_OUT_3;
#[doc = "`read()` method returns [aes_tag_out_3::R](aes_tag_out_3::R) reader structure"]
impl crate::Readable for AES_TAG_OUT_3 {}
#[doc = "`write(|w| ..)` method takes [aes_tag_out_3::W](aes_tag_out_3::W) writer structure"]
impl crate::Writable for AES_TAG_OUT_3 {}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_3;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_0](hash_data_in_0) module"]
pub type HASH_DATA_IN_0 = crate::Reg<u32, _HASH_DATA_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_0;
#[doc = "`read()` method returns [hash_data_in_0::R](hash_data_in_0::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_0 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_0::W](hash_data_in_0::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_0 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_0;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_1](hash_data_in_1) module"]
pub type HASH_DATA_IN_1 = crate::Reg<u32, _HASH_DATA_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_1;
#[doc = "`read()` method returns [hash_data_in_1::R](hash_data_in_1::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_1 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_1::W](hash_data_in_1::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_1 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_1;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_2](hash_data_in_2) module"]
pub type HASH_DATA_IN_2 = crate::Reg<u32, _HASH_DATA_IN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_2;
#[doc = "`read()` method returns [hash_data_in_2::R](hash_data_in_2::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_2 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_2::W](hash_data_in_2::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_2 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_2;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_3](hash_data_in_3) module"]
pub type HASH_DATA_IN_3 = crate::Reg<u32, _HASH_DATA_IN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_3;
#[doc = "`read()` method returns [hash_data_in_3::R](hash_data_in_3::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_3 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_3::W](hash_data_in_3::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_3 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_3;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_4](hash_data_in_4) module"]
pub type HASH_DATA_IN_4 = crate::Reg<u32, _HASH_DATA_IN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_4;
#[doc = "`read()` method returns [hash_data_in_4::R](hash_data_in_4::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_4 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_4::W](hash_data_in_4::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_4 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_4;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_5](hash_data_in_5) module"]
pub type HASH_DATA_IN_5 = crate::Reg<u32, _HASH_DATA_IN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_5;
#[doc = "`read()` method returns [hash_data_in_5::R](hash_data_in_5::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_5 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_5::W](hash_data_in_5::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_5 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_5;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_6](hash_data_in_6) module"]
pub type HASH_DATA_IN_6 = crate::Reg<u32, _HASH_DATA_IN_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_6;
#[doc = "`read()` method returns [hash_data_in_6::R](hash_data_in_6::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_6 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_6::W](hash_data_in_6::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_6 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_6;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_7](hash_data_in_7) module"]
pub type HASH_DATA_IN_7 = crate::Reg<u32, _HASH_DATA_IN_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_7;
#[doc = "`read()` method returns [hash_data_in_7::R](hash_data_in_7::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_7 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_7::W](hash_data_in_7::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_7 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_7;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_8](hash_data_in_8) module"]
pub type HASH_DATA_IN_8 = crate::Reg<u32, _HASH_DATA_IN_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_8;
#[doc = "`read()` method returns [hash_data_in_8::R](hash_data_in_8::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_8 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_8::W](hash_data_in_8::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_8 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_8;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_9](hash_data_in_9) module"]
pub type HASH_DATA_IN_9 = crate::Reg<u32, _HASH_DATA_IN_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_9;
#[doc = "`read()` method returns [hash_data_in_9::R](hash_data_in_9::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_9 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_9::W](hash_data_in_9::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_9 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_9;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_10](hash_data_in_10) module"]
pub type HASH_DATA_IN_10 = crate::Reg<u32, _HASH_DATA_IN_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_10;
#[doc = "`read()` method returns [hash_data_in_10::R](hash_data_in_10::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_10 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_10::W](hash_data_in_10::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_10 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_10;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_11](hash_data_in_11) module"]
pub type HASH_DATA_IN_11 = crate::Reg<u32, _HASH_DATA_IN_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_11;
#[doc = "`read()` method returns [hash_data_in_11::R](hash_data_in_11::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_11 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_11::W](hash_data_in_11::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_11 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_11;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_12](hash_data_in_12) module"]
pub type HASH_DATA_IN_12 = crate::Reg<u32, _HASH_DATA_IN_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_12;
#[doc = "`read()` method returns [hash_data_in_12::R](hash_data_in_12::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_12 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_12::W](hash_data_in_12::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_12 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_12;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_13](hash_data_in_13) module"]
pub type HASH_DATA_IN_13 = crate::Reg<u32, _HASH_DATA_IN_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_13;
#[doc = "`read()` method returns [hash_data_in_13::R](hash_data_in_13::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_13 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_13::W](hash_data_in_13::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_13 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_13;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_14](hash_data_in_14) module"]
pub type HASH_DATA_IN_14 = crate::Reg<u32, _HASH_DATA_IN_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_14;
#[doc = "`read()` method returns [hash_data_in_14::R](hash_data_in_14::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_14 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_14::W](hash_data_in_14::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_14 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_14;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_data_in_15](hash_data_in_15) module"]
pub type HASH_DATA_IN_15 = crate::Reg<u32, _HASH_DATA_IN_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DATA_IN_15;
#[doc = "`read()` method returns [hash_data_in_15::R](hash_data_in_15::R) reader structure"]
impl crate::Readable for HASH_DATA_IN_15 {}
#[doc = "`write(|w| ..)` method takes [hash_data_in_15::W](hash_data_in_15::W) writer structure"]
impl crate::Writable for HASH_DATA_IN_15 {}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_15;
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_io_buf_ctrl](hash_io_buf_ctrl) module"]
pub type HASH_IO_BUF_CTRL = crate::Reg<u32, _HASH_IO_BUF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_IO_BUF_CTRL;
#[doc = "`read()` method returns [hash_io_buf_ctrl::R](hash_io_buf_ctrl::R) reader structure"]
impl crate::Readable for HASH_IO_BUF_CTRL {}
#[doc = "`write(|w| ..)` method takes [hash_io_buf_ctrl::W](hash_io_buf_ctrl::W) writer structure"]
impl crate::Writable for HASH_IO_BUF_CTRL {}
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hash_io_buf_ctrl;
#[doc = "Hash mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_mode_in](hash_mode_in) module"]
pub type HASH_MODE_IN = crate::Reg<u32, _HASH_MODE_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_MODE_IN;
#[doc = "`read()` method returns [hash_mode_in::R](hash_mode_in::R) reader structure"]
impl crate::Readable for HASH_MODE_IN {}
#[doc = "`write(|w| ..)` method takes [hash_mode_in::W](hash_mode_in::W) writer structure"]
impl crate::Writable for HASH_MODE_IN {}
#[doc = "Hash mode register"]
pub mod hash_mode_in;
#[doc = "Hash length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_length_in_l](hash_length_in_l) module"]
pub type HASH_LENGTH_IN_L = crate::Reg<u32, _HASH_LENGTH_IN_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_LENGTH_IN_L;
#[doc = "`read()` method returns [hash_length_in_l::R](hash_length_in_l::R) reader structure"]
impl crate::Readable for HASH_LENGTH_IN_L {}
#[doc = "`write(|w| ..)` method takes [hash_length_in_l::W](hash_length_in_l::W) writer structure"]
impl crate::Writable for HASH_LENGTH_IN_L {}
#[doc = "Hash length register"]
pub mod hash_length_in_l;
#[doc = "Hash length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_length_in_h](hash_length_in_h) module"]
pub type HASH_LENGTH_IN_H = crate::Reg<u32, _HASH_LENGTH_IN_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_LENGTH_IN_H;
#[doc = "`read()` method returns [hash_length_in_h::R](hash_length_in_h::R) reader structure"]
impl crate::Readable for HASH_LENGTH_IN_H {}
#[doc = "`write(|w| ..)` method takes [hash_length_in_h::W](hash_length_in_h::W) writer structure"]
impl crate::Writable for HASH_LENGTH_IN_H {}
#[doc = "Hash length register"]
pub mod hash_length_in_h;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_a](hash_digest_a) module"]
pub type HASH_DIGEST_A = crate::Reg<u32, _HASH_DIGEST_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_A;
#[doc = "`read()` method returns [hash_digest_a::R](hash_digest_a::R) reader structure"]
impl crate::Readable for HASH_DIGEST_A {}
#[doc = "`write(|w| ..)` method takes [hash_digest_a::W](hash_digest_a::W) writer structure"]
impl crate::Writable for HASH_DIGEST_A {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_a;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_b](hash_digest_b) module"]
pub type HASH_DIGEST_B = crate::Reg<u32, _HASH_DIGEST_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_B;
#[doc = "`read()` method returns [hash_digest_b::R](hash_digest_b::R) reader structure"]
impl crate::Readable for HASH_DIGEST_B {}
#[doc = "`write(|w| ..)` method takes [hash_digest_b::W](hash_digest_b::W) writer structure"]
impl crate::Writable for HASH_DIGEST_B {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_b;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_c](hash_digest_c) module"]
pub type HASH_DIGEST_C = crate::Reg<u32, _HASH_DIGEST_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_C;
#[doc = "`read()` method returns [hash_digest_c::R](hash_digest_c::R) reader structure"]
impl crate::Readable for HASH_DIGEST_C {}
#[doc = "`write(|w| ..)` method takes [hash_digest_c::W](hash_digest_c::W) writer structure"]
impl crate::Writable for HASH_DIGEST_C {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_c;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_d](hash_digest_d) module"]
pub type HASH_DIGEST_D = crate::Reg<u32, _HASH_DIGEST_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_D;
#[doc = "`read()` method returns [hash_digest_d::R](hash_digest_d::R) reader structure"]
impl crate::Readable for HASH_DIGEST_D {}
#[doc = "`write(|w| ..)` method takes [hash_digest_d::W](hash_digest_d::W) writer structure"]
impl crate::Writable for HASH_DIGEST_D {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_d;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_e](hash_digest_e) module"]
pub type HASH_DIGEST_E = crate::Reg<u32, _HASH_DIGEST_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_E;
#[doc = "`read()` method returns [hash_digest_e::R](hash_digest_e::R) reader structure"]
impl crate::Readable for HASH_DIGEST_E {}
#[doc = "`write(|w| ..)` method takes [hash_digest_e::W](hash_digest_e::W) writer structure"]
impl crate::Writable for HASH_DIGEST_E {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_e;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_f](hash_digest_f) module"]
pub type HASH_DIGEST_F = crate::Reg<u32, _HASH_DIGEST_F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_F;
#[doc = "`read()` method returns [hash_digest_f::R](hash_digest_f::R) reader structure"]
impl crate::Readable for HASH_DIGEST_F {}
#[doc = "`write(|w| ..)` method takes [hash_digest_f::W](hash_digest_f::W) writer structure"]
impl crate::Writable for HASH_DIGEST_F {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_f;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_g](hash_digest_g) module"]
pub type HASH_DIGEST_G = crate::Reg<u32, _HASH_DIGEST_G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_G;
#[doc = "`read()` method returns [hash_digest_g::R](hash_digest_g::R) reader structure"]
impl crate::Readable for HASH_DIGEST_G {}
#[doc = "`write(|w| ..)` method takes [hash_digest_g::W](hash_digest_g::W) writer structure"]
impl crate::Writable for HASH_DIGEST_G {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_g;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_digest_h](hash_digest_h) module"]
pub type HASH_DIGEST_H = crate::Reg<u32, _HASH_DIGEST_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST_H;
#[doc = "`read()` method returns [hash_digest_h::R](hash_digest_h::R) reader structure"]
impl crate::Readable for HASH_DIGEST_H {}
#[doc = "`write(|w| ..)` method takes [hash_digest_h::W](hash_digest_h::W) writer structure"]
impl crate::Writable for HASH_DIGEST_H {}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_h;
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_alg_sel](ctrl_alg_sel) module"]
pub type CTRL_ALG_SEL = crate::Reg<u32, _CTRL_ALG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_ALG_SEL;
#[doc = "`read()` method returns [ctrl_alg_sel::R](ctrl_alg_sel::R) reader structure"]
impl crate::Readable for CTRL_ALG_SEL {}
#[doc = "`write(|w| ..)` method takes [ctrl_alg_sel::W](ctrl_alg_sel::W) writer structure"]
impl crate::Writable for CTRL_ALG_SEL {}
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod ctrl_alg_sel;
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_prot_en](ctrl_prot_en) module"]
pub type CTRL_PROT_EN = crate::Reg<u32, _CTRL_PROT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_PROT_EN;
#[doc = "`read()` method returns [ctrl_prot_en::R](ctrl_prot_en::R) reader structure"]
impl crate::Readable for CTRL_PROT_EN {}
#[doc = "`write(|w| ..)` method takes [ctrl_prot_en::W](ctrl_prot_en::W) writer structure"]
impl crate::Writable for CTRL_PROT_EN {}
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod ctrl_prot_en;
#[doc = "Software reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_sw_reset](ctrl_sw_reset) module"]
pub type CTRL_SW_RESET = crate::Reg<u32, _CTRL_SW_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SW_RESET;
#[doc = "`read()` method returns [ctrl_sw_reset::R](ctrl_sw_reset::R) reader structure"]
impl crate::Readable for CTRL_SW_RESET {}
#[doc = "`write(|w| ..)` method takes [ctrl_sw_reset::W](ctrl_sw_reset::W) writer structure"]
impl crate::Writable for CTRL_SW_RESET {}
#[doc = "Software reset"]
pub mod ctrl_sw_reset;
#[doc = "Interrupt configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_int_cfg](ctrl_int_cfg) module"]
pub type CTRL_INT_CFG = crate::Reg<u32, _CTRL_INT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_INT_CFG;
#[doc = "`read()` method returns [ctrl_int_cfg::R](ctrl_int_cfg::R) reader structure"]
impl crate::Readable for CTRL_INT_CFG {}
#[doc = "`write(|w| ..)` method takes [ctrl_int_cfg::W](ctrl_int_cfg::W) writer structure"]
impl crate::Writable for CTRL_INT_CFG {}
#[doc = "Interrupt configuration"]
pub mod ctrl_int_cfg;
#[doc = "Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_int_en](ctrl_int_en) module"]
pub type CTRL_INT_EN = crate::Reg<u32, _CTRL_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_INT_EN;
#[doc = "`read()` method returns [ctrl_int_en::R](ctrl_int_en::R) reader structure"]
impl crate::Readable for CTRL_INT_EN {}
#[doc = "`write(|w| ..)` method takes [ctrl_int_en::W](ctrl_int_en::W) writer structure"]
impl crate::Writable for CTRL_INT_EN {}
#[doc = "Interrupt enable"]
pub mod ctrl_int_en;
#[doc = "Interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_int_clr](ctrl_int_clr) module"]
pub type CTRL_INT_CLR = crate::Reg<u32, _CTRL_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_INT_CLR;
#[doc = "`read()` method returns [ctrl_int_clr::R](ctrl_int_clr::R) reader structure"]
impl crate::Readable for CTRL_INT_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_int_clr::W](ctrl_int_clr::W) writer structure"]
impl crate::Writable for CTRL_INT_CLR {}
#[doc = "Interrupt clear"]
pub mod ctrl_int_clr;
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_int_set](ctrl_int_set) module"]
pub type CTRL_INT_SET = crate::Reg<u32, _CTRL_INT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_INT_SET;
#[doc = "`read()` method returns [ctrl_int_set::R](ctrl_int_set::R) reader structure"]
impl crate::Readable for CTRL_INT_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_int_set::W](ctrl_int_set::W) writer structure"]
impl crate::Writable for CTRL_INT_SET {}
#[doc = "Interrupt set"]
pub mod ctrl_int_set;
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_int_stat](ctrl_int_stat) module"]
pub type CTRL_INT_STAT = crate::Reg<u32, _CTRL_INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_INT_STAT;
#[doc = "`read()` method returns [ctrl_int_stat::R](ctrl_int_stat::R) reader structure"]
impl crate::Readable for CTRL_INT_STAT {}
#[doc = "`write(|w| ..)` method takes [ctrl_int_stat::W](ctrl_int_stat::W) writer structure"]
impl crate::Writable for CTRL_INT_STAT {}
#[doc = "Interrupt status"]
pub mod ctrl_int_stat;
#[doc = "Options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_options](ctrl_options) module"]
pub type CTRL_OPTIONS = crate::Reg<u32, _CTRL_OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_OPTIONS;
#[doc = "`read()` method returns [ctrl_options::R](ctrl_options::R) reader structure"]
impl crate::Readable for CTRL_OPTIONS {}
#[doc = "`write(|w| ..)` method takes [ctrl_options::W](ctrl_options::W) writer structure"]
impl crate::Writable for CTRL_OPTIONS {}
#[doc = "Options register"]
pub mod ctrl_options;
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_version](ctrl_version) module"]
pub type CTRL_VERSION = crate::Reg<u32, _CTRL_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_VERSION;
#[doc = "`read()` method returns [ctrl_version::R](ctrl_version::R) reader structure"]
impl crate::Readable for CTRL_VERSION {}
#[doc = "`write(|w| ..)` method takes [ctrl_version::W](ctrl_version::W) writer structure"]
impl crate::Writable for CTRL_VERSION {}
#[doc = "Version register"]
pub mod ctrl_version;
