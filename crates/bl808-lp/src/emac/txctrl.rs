#[doc = "Register `TXCTRL` reader"]
pub type R = crate::R<TxctrlSpec>;
#[doc = "Register `TXCTRL` writer"]
pub type W = crate::W<TxctrlSpec>;
#[doc = "Field `TXPAUSETV` reader - Pause Timer Value"]
pub type TxpausetvR = crate::FieldReader<u16>;
#[doc = "Field `TXPAUSETV` writer - Pause Timer Value"]
pub type TxpausetvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXPAUSERQ` reader - Pause Request"]
pub type TxpauserqR = crate::BitReader;
#[doc = "Field `TXPAUSERQ` writer - Pause Request"]
pub type TxpauserqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Pause Timer Value"]
    #[inline(always)]
    pub fn txpausetv(&self) -> TxpausetvR {
        TxpausetvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Pause Request"]
    #[inline(always)]
    pub fn txpauserq(&self) -> TxpauserqR {
        TxpauserqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Timer Value"]
    #[inline(always)]
    pub fn txpausetv(&mut self) -> TxpausetvW<'_, TxctrlSpec> {
        TxpausetvW::new(self, 0)
    }
    #[doc = "Bit 16 - Pause Request"]
    #[inline(always)]
    pub fn txpauserq(&mut self) -> TxpauserqW<'_, TxctrlSpec> {
        TxpauserqW::new(self, 16)
    }
}
#[doc = "Transmit Control (Flow Control) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctrlSpec;
impl crate::RegisterSpec for TxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TxctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TxctrlSpec {}
