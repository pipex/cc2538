#[doc = "Reader of register CC"]
pub type R = crate::R<u32, super::CC>;
#[doc = "Writer for register CC"]
pub type W = crate::W<u32, super::CC>;
#[doc = "Register CC `reset()`'s with value 0"]
impl crate::ResetValue for super::CC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved29`"]
pub type RESERVED29_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\] UART baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the UART. bit0 (PIOSC): 1: The UART baud clock is determined by the IO DIV setting in the system controller. 0: The UART baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The UART system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The UART system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] UART baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the UART. bit0 (PIOSC): 1: The UART baud clock is determined by the IO DIV setting in the system controller. 0: The UART baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The UART system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The UART system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
}
