#[doc = "Register `HBN_BOR_CFG` reader"]
pub type R = crate::R<HbnBorCfgSpec>;
#[doc = "Register `HBN_BOR_CFG` writer"]
pub type W = crate::W<HbnBorCfgSpec>;
#[doc = "Field `bod_sel` reader - "]
pub type BodSelR = crate::BitReader;
#[doc = "Field `bod_sel` writer - "]
pub type BodSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bod_vth` reader - "]
pub type BodVthR = crate::FieldReader;
#[doc = "Field `bod_vth` writer - "]
pub type BodVthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pu_bod` reader - "]
pub type PuBodR = crate::BitReader;
#[doc = "Field `pu_bod` writer - "]
pub type PuBodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bod_out` reader - "]
pub type RBodOutR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bod_sel(&self) -> BodSelR {
        BodSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn bod_vth(&self) -> BodVthR {
        BodVthR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_bod(&self) -> PuBodR {
        PuBodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_bod_out(&self) -> RBodOutR {
        RBodOutR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bod_sel(&mut self) -> BodSelW<'_, HbnBorCfgSpec> {
        BodSelW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn bod_vth(&mut self) -> BodVthW<'_, HbnBorCfgSpec> {
        BodVthW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_bod(&mut self) -> PuBodW<'_, HbnBorCfgSpec> {
        PuBodW::new(self, 4)
    }
}
#[doc = "Brown-Out Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_bor_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_bor_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnBorCfgSpec;
impl crate::RegisterSpec for HbnBorCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_bor_cfg::R`](R) reader structure"]
impl crate::Readable for HbnBorCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_bor_cfg::W`](W) writer structure"]
impl crate::Writable for HbnBorCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_BOR_CFG to value 0x0a"]
impl crate::Resettable for HbnBorCfgSpec {
    const RESET_VALUE: u32 = 0x0a;
}
