#[doc = "Reader of register FSCTRL"]
pub type R = crate::R<u32, super::FSCTRL>;
#[doc = "Writer for register FSCTRL"]
pub type W = crate::W<u32, super::FSCTRL>;
#[doc = "Register FSCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FSCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRE_CURRENT`"]
pub type PRE_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_CURRENT`"]
pub struct PRE_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LODIV_BUF_CURRENT_TX`"]
pub type LODIV_BUF_CURRENT_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LODIV_BUF_CURRENT_TX`"]
pub struct LODIV_BUF_CURRENT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BUF_CURRENT_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LODIV_BUF_CURRENT_RX`"]
pub type LODIV_BUF_CURRENT_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LODIV_BUF_CURRENT_RX`"]
pub struct LODIV_BUF_CURRENT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BUF_CURRENT_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `LODIV_CURRENT`"]
pub type LODIV_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LODIV_CURRENT`"]
pub struct LODIV_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\] Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&self) -> PRE_CURRENT_R {
        PRE_CURRENT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\] Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&self) -> LODIV_BUF_CURRENT_TX_R {
        LODIV_BUF_CURRENT_TX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&self) -> LODIV_BUF_CURRENT_RX_R {
        LODIV_BUF_CURRENT_RX_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&self) -> LODIV_CURRENT_R {
        LODIV_CURRENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&mut self) -> PRE_CURRENT_W {
        PRE_CURRENT_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&mut self) -> LODIV_BUF_CURRENT_TX_W {
        LODIV_BUF_CURRENT_TX_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&mut self) -> LODIV_BUF_CURRENT_RX_W {
        LODIV_BUF_CURRENT_RX_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&mut self) -> LODIV_CURRENT_W {
        LODIV_CURRENT_W { w: self }
    }
}
