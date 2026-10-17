#[doc = "Register `OTG_CSR` reader"]
pub type R = crate::R<OtgCsrSpec>;
#[doc = "Register `OTG_CSR` writer"]
pub type W = crate::W<OtgCsrSpec>;
#[doc = "Field `B_BUS_REQ` reader - B-device Bus Request"]
pub type BBusReqR = crate::BitReader;
#[doc = "Field `B_BUS_REQ` writer - B-device Bus Request"]
pub type BBusReqW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `B_HNP_EN` reader - B-device HNP Enable"]
pub type BHnpEnR = crate::BitReader;
#[doc = "Field `B_HNP_EN` writer - B-device HNP Enable"]
pub type BHnpEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `B_DSCHRG_VBUS` reader - B-device Discharge VBUS"]
pub type BDschrgVbusR = crate::BitReader;
#[doc = "Field `B_DSCHRG_VBUS` writer - B-device Discharge VBUS"]
pub type BDschrgVbusW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_BUS_REQ_HOV` reader - A-device Bus Request (Host Overwrite)"]
pub type ABusReqHovR = crate::BitReader;
#[doc = "Field `A_BUS_REQ_HOV` writer - A-device Bus Request (Host Overwrite)"]
pub type ABusReqHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_BUS_DROP_HOV` reader - A-device Bus Drop (Host Overwrite)"]
pub type ABusDropHovR = crate::BitReader;
#[doc = "Field `A_BUS_DROP_HOV` writer - A-device Bus Drop (Host Overwrite)"]
pub type ABusDropHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_SET_B_HNP_EN` reader - A-device Set B-HNP Enable"]
pub type ASetBHnpEnR = crate::BitReader;
#[doc = "Field `A_SET_B_HNP_EN` writer - A-device Set B-HNP Enable"]
pub type ASetBHnpEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_SRP_DET_EN` reader - A-device SRP Detect Enable"]
pub type ASrpDetEnR = crate::BitReader;
#[doc = "Field `A_SRP_DET_EN` writer - A-device SRP Detect Enable"]
pub type ASrpDetEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_SRP_RESP_TYP` reader - A-device SRP Response Type"]
pub type ASrpRespTypR = crate::BitReader;
#[doc = "Field `A_SRP_RESP_TYP` writer - A-device SRP Response Type"]
pub type ASrpRespTypW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ID_FLT_SEL` reader - ID Filter Select"]
pub type IdFltSelR = crate::BitReader;
#[doc = "Field `ID_FLT_SEL` writer - ID Filter Select"]
pub type IdFltSelW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VBUS_FLT_SEL_HOV_POV` reader - VBUS Filter Select (Host/Peripheral Overwrite)"]
pub type VbusFltSelHovPovR = crate::BitReader;
#[doc = "Field `VBUS_FLT_SEL_HOV_POV` writer - VBUS Filter Select (Host/Peripheral Overwrite)"]
pub type VbusFltSelHovPovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HDISCON_FLT_SEL_HOV` reader - Host Disconnect Filter Select (Host Overwrite)"]
pub type HdisconFltSelHovR = crate::BitReader;
#[doc = "Field `HDISCON_FLT_SEL_HOV` writer - Host Disconnect Filter Select (Host Overwrite)"]
pub type HdisconFltSelHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IDPULUP_HOV_POV` reader - ID Pull-up (Host/Peripheral Overwrite)"]
pub type IdpulupHovPovR = crate::BitReader;
#[doc = "Field `IDPULUP_HOV_POV` writer - ID Pull-up (Host/Peripheral Overwrite)"]
pub type IdpulupHovPovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `B_SESS_END_POV` reader - B-Session End (Peripheral Overwrite)"]
pub type BSessEndPovR = crate::BitReader;
#[doc = "Field `B_SESS_VLD_POV` reader - B-Session Valid (Peripheral Overwrite)"]
pub type BSessVldPovR = crate::BitReader;
#[doc = "Field `A_SESS_VLD` reader - A-Session Valid"]
pub type ASessVldR = crate::BitReader;
#[doc = "Field `VBUS_VLD_HOV` reader - VBUS Valid (Host Overwrite)"]
pub type VbusVldHovR = crate::BitReader;
#[doc = "Field `CROLE_HOV_POV` reader - Current Role (Host/Peripheral Overwrite)"]
pub type CroleHovPovR = crate::BitReader;
#[doc = "Field `ID_HOV_POV` reader - ID Status (Host/Peripheral Overwrite)"]
pub type IdHovPovR = crate::BitReader;
#[doc = "Field `SPD_TYP_HOV_POV` reader - Speed Type (Host/Peripheral Overwrite)"]
pub type SpdTypHovPovR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - B-device Bus Request"]
    #[inline(always)]
    pub fn b_bus_req(&self) -> BBusReqR {
        BBusReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - B-device HNP Enable"]
    #[inline(always)]
    pub fn b_hnp_en(&self) -> BHnpEnR {
        BHnpEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - B-device Discharge VBUS"]
    #[inline(always)]
    pub fn b_dschrg_vbus(&self) -> BDschrgVbusR {
        BDschrgVbusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - A-device Bus Request (Host Overwrite)"]
    #[inline(always)]
    pub fn a_bus_req_hov(&self) -> ABusReqHovR {
        ABusReqHovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-device Bus Drop (Host Overwrite)"]
    #[inline(always)]
    pub fn a_bus_drop_hov(&self) -> ABusDropHovR {
        ABusDropHovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A-device Set B-HNP Enable"]
    #[inline(always)]
    pub fn a_set_b_hnp_en(&self) -> ASetBHnpEnR {
        ASetBHnpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A-device SRP Detect Enable"]
    #[inline(always)]
    pub fn a_srp_det_en(&self) -> ASrpDetEnR {
        ASrpDetEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A-device SRP Response Type"]
    #[inline(always)]
    pub fn a_srp_resp_typ(&self) -> ASrpRespTypR {
        ASrpRespTypR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ID Filter Select"]
    #[inline(always)]
    pub fn id_flt_sel(&self) -> IdFltSelR {
        IdFltSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBUS Filter Select (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn vbus_flt_sel_hov_pov(&self) -> VbusFltSelHovPovR {
        VbusFltSelHovPovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host Disconnect Filter Select (Host Overwrite)"]
    #[inline(always)]
    pub fn hdiscon_flt_sel_hov(&self) -> HdisconFltSelHovR {
        HdisconFltSelHovR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - ID Pull-up (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn idpulup_hov_pov(&self) -> IdpulupHovPovR {
        IdpulupHovPovR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - B-Session End (Peripheral Overwrite)"]
    #[inline(always)]
    pub fn b_sess_end_pov(&self) -> BSessEndPovR {
        BSessEndPovR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - B-Session Valid (Peripheral Overwrite)"]
    #[inline(always)]
    pub fn b_sess_vld_pov(&self) -> BSessVldPovR {
        BSessVldPovR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn a_sess_vld(&self) -> ASessVldR {
        ASessVldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VBUS Valid (Host Overwrite)"]
    #[inline(always)]
    pub fn vbus_vld_hov(&self) -> VbusVldHovR {
        VbusVldHovR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Current Role (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn crole_hov_pov(&self) -> CroleHovPovR {
        CroleHovPovR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ID Status (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn id_hov_pov(&self) -> IdHovPovR {
        IdHovPovR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Speed Type (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn spd_typ_hov_pov(&self) -> SpdTypHovPovR {
        SpdTypHovPovR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - B-device Bus Request"]
    #[inline(always)]
    pub fn b_bus_req(&mut self) -> BBusReqW<'_, OtgCsrSpec> {
        BBusReqW::new(self, 0)
    }
    #[doc = "Bit 1 - B-device HNP Enable"]
    #[inline(always)]
    pub fn b_hnp_en(&mut self) -> BHnpEnW<'_, OtgCsrSpec> {
        BHnpEnW::new(self, 1)
    }
    #[doc = "Bit 2 - B-device Discharge VBUS"]
    #[inline(always)]
    pub fn b_dschrg_vbus(&mut self) -> BDschrgVbusW<'_, OtgCsrSpec> {
        BDschrgVbusW::new(self, 2)
    }
    #[doc = "Bit 4 - A-device Bus Request (Host Overwrite)"]
    #[inline(always)]
    pub fn a_bus_req_hov(&mut self) -> ABusReqHovW<'_, OtgCsrSpec> {
        ABusReqHovW::new(self, 4)
    }
    #[doc = "Bit 5 - A-device Bus Drop (Host Overwrite)"]
    #[inline(always)]
    pub fn a_bus_drop_hov(&mut self) -> ABusDropHovW<'_, OtgCsrSpec> {
        ABusDropHovW::new(self, 5)
    }
    #[doc = "Bit 6 - A-device Set B-HNP Enable"]
    #[inline(always)]
    pub fn a_set_b_hnp_en(&mut self) -> ASetBHnpEnW<'_, OtgCsrSpec> {
        ASetBHnpEnW::new(self, 6)
    }
    #[doc = "Bit 7 - A-device SRP Detect Enable"]
    #[inline(always)]
    pub fn a_srp_det_en(&mut self) -> ASrpDetEnW<'_, OtgCsrSpec> {
        ASrpDetEnW::new(self, 7)
    }
    #[doc = "Bit 8 - A-device SRP Response Type"]
    #[inline(always)]
    pub fn a_srp_resp_typ(&mut self) -> ASrpRespTypW<'_, OtgCsrSpec> {
        ASrpRespTypW::new(self, 8)
    }
    #[doc = "Bit 9 - ID Filter Select"]
    #[inline(always)]
    pub fn id_flt_sel(&mut self) -> IdFltSelW<'_, OtgCsrSpec> {
        IdFltSelW::new(self, 9)
    }
    #[doc = "Bit 10 - VBUS Filter Select (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn vbus_flt_sel_hov_pov(&mut self) -> VbusFltSelHovPovW<'_, OtgCsrSpec> {
        VbusFltSelHovPovW::new(self, 10)
    }
    #[doc = "Bit 11 - Host Disconnect Filter Select (Host Overwrite)"]
    #[inline(always)]
    pub fn hdiscon_flt_sel_hov(&mut self) -> HdisconFltSelHovW<'_, OtgCsrSpec> {
        HdisconFltSelHovW::new(self, 11)
    }
    #[doc = "Bit 13 - ID Pull-up (Host/Peripheral Overwrite)"]
    #[inline(always)]
    pub fn idpulup_hov_pov(&mut self) -> IdpulupHovPovW<'_, OtgCsrSpec> {
        IdpulupHovPovW::new(self, 13)
    }
}
#[doc = "OTG Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgCsrSpec;
impl crate::RegisterSpec for OtgCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_csr::R`](R) reader structure"]
impl crate::Readable for OtgCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_csr::W`](W) writer structure"]
impl crate::Writable for OtgCsrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2ff7;
}
#[doc = "`reset()` method sets OTG_CSR to value 0"]
impl crate::Resettable for OtgCsrSpec {}
