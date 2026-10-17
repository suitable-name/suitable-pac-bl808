#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `TXB_M` reader - "]
pub type TxbMR = crate::BitReader;
#[doc = "Field `TXB_M` writer - "]
pub type TxbMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE_M` reader - "]
pub type TxeMR = crate::BitReader;
#[doc = "Field `TXE_M` writer - "]
pub type TxeMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB_M` reader - "]
pub type RxbMR = crate::BitReader;
#[doc = "Field `RXB_M` writer - "]
pub type RxbMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE_M` reader - "]
pub type RxeMR = crate::BitReader;
#[doc = "Field `RXE_M` writer - "]
pub type RxeMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_M` reader - "]
pub type BusyMR = crate::BitReader;
#[doc = "Field `BUSY_M` writer - "]
pub type BusyMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_M` reader - "]
pub type TxcMR = crate::BitReader;
#[doc = "Field `TXC_M` writer - "]
pub type TxcMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC_M` reader - "]
pub type RxcMR = crate::BitReader;
#[doc = "Field `RXC_M` writer - "]
pub type RxcMW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&self) -> TxbMR {
        TxbMR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&self) -> TxeMR {
        TxeMR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&self) -> RxbMR {
        RxbMR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&self) -> RxeMR {
        RxeMR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&self) -> BusyMR {
        BusyMR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&self) -> TxcMR {
        TxcMR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&self) -> RxcMR {
        RxcMR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&mut self) -> TxbMW<'_, IntMaskSpec> {
        TxbMW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&mut self) -> TxeMW<'_, IntMaskSpec> {
        TxeMW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&mut self) -> RxbMW<'_, IntMaskSpec> {
        RxbMW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&mut self) -> RxeMW<'_, IntMaskSpec> {
        RxeMW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&mut self) -> BusyMW<'_, IntMaskSpec> {
        BusyMW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&mut self) -> TxcMW<'_, IntMaskSpec> {
        TxcMW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&mut self) -> RxcMW<'_, IntMaskSpec> {
        RxcMW::new(self, 6)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MASK to value 0x7f"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0x7f;
}
