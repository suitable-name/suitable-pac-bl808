#[doc = "Register `PORTSC` reader"]
pub type R = crate::R<PortscSpec>;
#[doc = "Register `PORTSC` writer"]
pub type W = crate::W<PortscSpec>;
#[doc = "Field `CONN_STS` reader - Current Connect Status"]
pub type ConnStsR = crate::BitReader;
#[doc = "Field `CONN_CHG` reader - Connect Status Change (W1C)"]
pub type ConnChgR = crate::BitReader;
#[doc = "Field `CONN_CHG` writer - Connect Status Change (W1C)"]
pub type ConnChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PO_EN` reader - Port Enable/Disable"]
pub type PoEnR = crate::BitReader;
#[doc = "Field `PO_EN` writer - Port Enable/Disable"]
pub type PoEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PO_EN_CHG` reader - Port Enable/Disable Change (W1C)"]
pub type PoEnChgR = crate::BitReader;
#[doc = "Field `PO_EN_CHG` writer - Port Enable/Disable Change (W1C)"]
pub type PoEnChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `F_PO_RESM` reader - Force Port Resume"]
pub type FPoResmR = crate::BitReader;
#[doc = "Field `F_PO_RESM` writer - Force Port Resume"]
pub type FPoResmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PO_SUSP` reader - Port Suspend"]
pub type PoSuspR = crate::BitReader;
#[doc = "Field `PO_SUSP` writer - Port Suspend"]
pub type PoSuspW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PO_RESET` reader - Port Reset"]
pub type PoResetR = crate::BitReader;
#[doc = "Field `PO_RESET` writer - Port Reset"]
pub type PoResetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LINE_STS` reader - Line Status"]
pub type LineStsR = crate::FieldReader;
#[doc = "Field `PORT_TEST` reader - Port Test Control"]
pub type PortTestR = crate::FieldReader;
#[doc = "Field `PORT_TEST` writer - Port Test Control"]
pub type PortTestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HC_TST_PKDONE` reader - Host Controller Test Packet Done Status"]
pub type HcTstPkdoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn conn_sts(&self) -> ConnStsR {
        ConnStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change (W1C)"]
    #[inline(always)]
    pub fn conn_chg(&self) -> ConnChgR {
        ConnChgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable/Disable"]
    #[inline(always)]
    pub fn po_en(&self) -> PoEnR {
        PoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (W1C)"]
    #[inline(always)]
    pub fn po_en_chg(&self) -> PoEnChgR {
        PoEnChgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn f_po_resm(&self) -> FPoResmR {
        FPoResmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn po_susp(&self) -> PoSuspR {
        PoSuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn po_reset(&self) -> PoResetR {
        PoResetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline(always)]
    pub fn line_sts(&self) -> LineStsR {
        LineStsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn port_test(&self) -> PortTestR {
        PortTestR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Host Controller Test Packet Done Status"]
    #[inline(always)]
    pub fn hc_tst_pkdone(&self) -> HcTstPkdoneR {
        HcTstPkdoneR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change (W1C)"]
    #[inline(always)]
    pub fn conn_chg(&mut self) -> ConnChgW<'_, PortscSpec> {
        ConnChgW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable/Disable"]
    #[inline(always)]
    pub fn po_en(&mut self) -> PoEnW<'_, PortscSpec> {
        PoEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (W1C)"]
    #[inline(always)]
    pub fn po_en_chg(&mut self) -> PoEnChgW<'_, PortscSpec> {
        PoEnChgW::new(self, 3)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn f_po_resm(&mut self) -> FPoResmW<'_, PortscSpec> {
        FPoResmW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn po_susp(&mut self) -> PoSuspW<'_, PortscSpec> {
        PoSuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn po_reset(&mut self) -> PoResetW<'_, PortscSpec> {
        PoResetW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn port_test(&mut self) -> PortTestW<'_, PortscSpec> {
        PortTestW::new(self, 16)
    }
}
#[doc = "Port Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`portsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortscSpec;
impl crate::RegisterSpec for PortscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portsc::R`](R) reader structure"]
impl crate::Readable for PortscSpec {}
#[doc = "`write(|w| ..)` method takes [`portsc::W`](W) writer structure"]
impl crate::Writable for PortscSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_01ce;
}
#[doc = "`reset()` method sets PORTSC to value 0"]
impl crate::Resettable for PortscSpec {}
