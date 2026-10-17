#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aud_top: AudTop,
    aud_itf: AudItf,
    adc_0: Adc0,
    adc_1: Adc1,
    dac_0: Dac0,
    dac_1: Dac1,
    aec_0: Aec0,
    pdm_0: Pdm0,
    rsvd0: Rsvd0,
    dbg_0: Dbg0,
    dbg_1: Dbg1,
    dbg_2: Dbg2,
    dbg_3: Dbg3,
    dbg_4: Dbg4,
    _reserved14: [u8; 0x48],
    rx_fifo_ctrl: RxFifoCtrl,
    rx_fifo_status: RxFifoStatus,
    rx_fifo_data: RxFifoData,
    tx_fifo_ctrl: TxFifoCtrl,
    tx_fifo_status: TxFifoStatus,
    tx_fifo_data: TxFifoData,
    _reserved20: [u8; 0x0168],
    peq_: [Peq_; 25],
    _reserved21: [u8; 0x0c],
    adc_h: AdcH,
    adc_h_misc: AdcHMisc,
    adc_s0: AdcS0,
    adc_s0_misc: AdcS0Misc,
    adc_s1: AdcS1,
    adc_s1_misc: AdcS1Misc,
    adc_s2: AdcS2,
    adc_s2_misc: AdcS2Misc,
    adc_agc_0: AdcAgc0,
    adc_agc_1: AdcAgc1,
    adc_agc_2: AdcAgc2,
    adc_agc_3: AdcAgc3,
    audio_status: AudioStatus,
    dac_s0: DacS0,
    dac_s0_misc: DacS0Misc,
    dac_s1: DacS1,
    dac_s1_misc: DacS1Misc,
    dac_drc_0: DacDrc0,
    dac_drc_1: DacDrc1,
    dac_ngt_0: DacNgt0,
    dac_zd_0: DacZd0,
    _reserved42: [u8; 0x01ac],
    vad_ctrl_0: VadCtrl0,
    vad_ctrl_1: VadCtrl1,
    vad_ctrl_2: VadCtrl2,
    vad_ctrl_3: VadCtrl3,
    vad_ctrl_4: VadCtrl4,
    vad_ctrl_5: VadCtrl5,
    _reserved48: [u8; 0xe8],
    codec_power1: CodecPower1,
    codec_power2: CodecPower2,
    codec_dac: CodecDac,
    codec_dac_ramp: CodecDacRamp,
    codec_adc0: CodecAdc0,
    codec_adc1: CodecAdc1,
    codec_adc2: CodecAdc2,
    _reserved55: [u8; 0xe4],
    padc_ana_cfg1: PadcAnaCfg1,
    padc_ana_cfg2: PadcAnaCfg2,
    padc_ana_cmd: PadcAnaCmd,
    padc_out_cfg: PadcOutCfg,
    padc_calib_cmd: PadcCalibCmd,
    padc_status: PadcStatus,
    padc_data: PadcData,
    padc_calib_cfg1: PadcCalibCfg1,
    padc_calib_cfg2: PadcCalibCfg2,
    padc_calib_cfg3: PadcCalibCfg3,
    padc_calib_cfg4: PadcCalibCfg4,
    padc_calib_cfg5: PadcCalibCfg5,
    padc_calib_cfg6: PadcCalibCfg6,
    padc_calib_cfg7: PadcCalibCfg7,
    padc_oec_w1p: PadcOecW1p,
    padc_gec_w1p: PadcGecW1p,
    padc_oec_cfg0: PadcOecCfg0,
    padc_oec_cfg1: PadcOecCfg1,
    padc_oec_cfg2: PadcOecCfg2,
    padc_oec_cfg3: PadcOecCfg3,
    padc_oec_cfg4: PadcOecCfg4,
    padc_oec_cfg5: PadcOecCfg5,
    padc_oec_cfg6: PadcOecCfg6,
    padc_oec_cfg7: PadcOecCfg7,
    padc_gec_cfg0: PadcGecCfg0,
    padc_gec_cfg1: PadcGecCfg1,
    padc_gec_cfg2: PadcGecCfg2,
    padc_gec_cfg3: PadcGecCfg3,
    padc_gec_cfg4: PadcGecCfg4,
    padc_gec_cfg5: PadcGecCfg5,
    padc_gec_cfg6: PadcGecCfg6,
    padc_gec_cfg7: PadcGecCfg7,
    _reserved87: [u8; 0x80],
    padc_rx_fifo_ctrl: PadcRxFifoCtrl,
    padc_rx_fifo_status: PadcRxFifoStatus,
    padc_rx_fifo_data: PadcRxFifoData,
}
impl RegisterBlock {
    #[doc = "0x00 - Audio Top Level Configuration: Clock gating and sample rate management"]
    #[inline(always)]
    pub const fn aud_top(&self) -> &AudTop {
        &self.aud_top
    }
    #[doc = "0x04 - Audio Interface Configuration: Channel routing and enable control"]
    #[inline(always)]
    pub const fn aud_itf(&self) -> &AudItf {
        &self.aud_itf
    }
    #[doc = "0x08 - ADC Filter and Scaling Configuration 0"]
    #[inline(always)]
    pub const fn adc_0(&self) -> &Adc0 {
        &self.adc_0
    }
    #[doc = "0x0c - ADC Analog Gain and Control Loop Config 1"]
    #[inline(always)]
    pub const fn adc_1(&self) -> &Adc1 {
        &self.adc_1
    }
    #[doc = "0x10 - DAC Configuration 0: Digital-to-Analog conversion parameters"]
    #[inline(always)]
    pub const fn dac_0(&self) -> &Dac0 {
        &self.dac_0
    }
    #[doc = "0x14 - DAC Configuration 1: Modulation and order settings"]
    #[inline(always)]
    pub const fn dac_1(&self) -> &Dac1 {
        &self.dac_1
    }
    #[doc = "0x18 - AEC Configuration 0: Acoustic Echo Cancellation control"]
    #[inline(always)]
    pub const fn aec_0(&self) -> &Aec0 {
        &self.aec_0
    }
    #[doc = "0x1c - PDM Configuration 0: Pulse Density Modulation interface setup"]
    #[inline(always)]
    pub const fn pdm_0(&self) -> &Pdm0 {
        &self.pdm_0
    }
    #[doc = "0x20 - Reserved: Do not modify"]
    #[inline(always)]
    pub const fn rsvd0(&self) -> &Rsvd0 {
        &self.rsvd0
    }
    #[doc = "0x24 - Audio System Debug Control: Sine generator and test point routing"]
    #[inline(always)]
    pub const fn dbg_0(&self) -> &Dbg0 {
        &self.dbg_0
    }
    #[doc = "0x28 - Debug Register 1: Audio internal data read port"]
    #[inline(always)]
    pub const fn dbg_1(&self) -> &Dbg1 {
        &self.dbg_1
    }
    #[doc = "0x2c - Debug Register 2: FIR filter and ADC test selection"]
    #[inline(always)]
    pub const fn dbg_2(&self) -> &Dbg2 {
        &self.dbg_2
    }
    #[doc = "0x30 - Debug Register 3: DAC DWA and input test selection"]
    #[inline(always)]
    pub const fn dbg_3(&self) -> &Dbg3 {
        &self.dbg_3
    }
    #[doc = "0x34 - Debug Register 4: Audio synchronization ratio monitoring"]
    #[inline(always)]
    pub const fn dbg_4(&self) -> &Dbg4 {
        &self.dbg_4
    }
    #[doc = "0x80 - RX FIFO Control: Configuration for audio data reception and DMA thresholds"]
    #[inline(always)]
    pub const fn rx_fifo_ctrl(&self) -> &RxFifoCtrl {
        &self.rx_fifo_ctrl
    }
    #[doc = "0x84 - RX FIFO Status: Overflow, underflow, and item count"]
    #[inline(always)]
    pub const fn rx_fifo_status(&self) -> &RxFifoStatus {
        &self.rx_fifo_status
    }
    #[doc = "0x88 - RX FIFO Data: Read portal for incoming audio streams"]
    #[inline(always)]
    pub const fn rx_fifo_data(&self) -> &RxFifoData {
        &self.rx_fifo_data
    }
    #[doc = "0x8c - TX FIFO Control: Configuration for audio data transmission and DMA thresholds"]
    #[inline(always)]
    pub const fn tx_fifo_ctrl(&self) -> &TxFifoCtrl {
        &self.tx_fifo_ctrl
    }
    #[doc = "0x90 - TX FIFO Status: Request status and available space count"]
    #[inline(always)]
    pub const fn tx_fifo_status(&self) -> &TxFifoStatus {
        &self.tx_fifo_status
    }
    #[doc = "0x94 - TX FIFO Data: Write portal for outgoing audio streams"]
    #[inline(always)]
    pub const fn tx_fifo_data(&self) -> &TxFifoData {
        &self.tx_fifo_data
    }
    #[doc = "0x200..0x3f4 - Parametric Equalizer Bank %s: Sets the IIR biquad coefficients for the audio DSP chain"]
    #[inline(always)]
    pub const fn peq_(&self, n: usize) -> &Peq_ {
        &self.peq_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x3f4 - Parametric Equalizer Bank %s: Sets the IIR biquad coefficients for the audio DSP chain"]
    #[inline(always)]
    pub fn peq__iter(&self) -> impl Iterator<Item = &Peq_> {
        self.peq_.iter()
    }
    #[doc = "0x400 - ADC High-rate path: Main audio volume and mute control"]
    #[inline(always)]
    pub const fn adc_h(&self) -> &AdcH {
        &self.adc_h
    }
    #[doc = "0x404 - ADC High-rate miscellaneous: Zero-crossing timeouts"]
    #[inline(always)]
    pub const fn adc_h_misc(&self) -> &AdcHMisc {
        &self.adc_h_misc
    }
    #[doc = "0x408 - ADC Sideband 0: Volume and mute control for secondary audio path"]
    #[inline(always)]
    pub const fn adc_s0(&self) -> &AdcS0 {
        &self.adc_s0
    }
    #[doc = "0x40c - ADC Sideband 0 misc: Global mute and update synchronization"]
    #[inline(always)]
    pub const fn adc_s0_misc(&self) -> &AdcS0Misc {
        &self.adc_s0_misc
    }
    #[doc = "0x410 - ADC Sideband 1: Volume and mute control for secondary audio path"]
    #[inline(always)]
    pub const fn adc_s1(&self) -> &AdcS1 {
        &self.adc_s1
    }
    #[doc = "0x414 - ADC Sideband 1 misc: Zero-crossing timeout settings"]
    #[inline(always)]
    pub const fn adc_s1_misc(&self) -> &AdcS1Misc {
        &self.adc_s1_misc
    }
    #[doc = "0x418 - ADC Sideband 2: Volume and mute control for secondary audio path"]
    #[inline(always)]
    pub const fn adc_s2(&self) -> &AdcS2 {
        &self.adc_s2
    }
    #[doc = "0x41c - ADC Sideband 2 misc: Zero-crossing timeout settings"]
    #[inline(always)]
    pub const fn adc_s2_misc(&self) -> &AdcS2Misc {
        &self.adc_s2_misc
    }
    #[doc = "0x420 - ADC Automatic Gain Control 0: Thresholds and forced gain management"]
    #[inline(always)]
    pub const fn adc_agc_0(&self) -> &AdcAgc0 {
        &self.adc_agc_0
    }
    #[doc = "0x424 - ADC Automatic Gain Control 1: Attack, decay, and hold timing"]
    #[inline(always)]
    pub const fn adc_agc_1(&self) -> &AdcAgc1 {
        &self.adc_agc_1
    }
    #[doc = "0x428 - ADC Automatic Gain Control 2: Noise gate and silence gain"]
    #[inline(always)]
    pub const fn adc_agc_2(&self) -> &AdcAgc2 {
        &self.adc_agc_2
    }
    #[doc = "0x42c - ADC Automatic Gain Control 3: Detection mode and noise exit timing"]
    #[inline(always)]
    pub const fn adc_agc_3(&self) -> &AdcAgc3 {
        &self.adc_agc_3
    }
    #[doc = "0x430 - Audio Master Status and Interrupt: Clearing bits for all ADC/DAC events"]
    #[inline(always)]
    pub const fn audio_status(&self) -> &AudioStatus {
        &self.audio_status
    }
    #[doc = "0x434 - DAC Sideband 0: Main control for secondary audio output channel"]
    #[inline(always)]
    pub const fn dac_s0(&self) -> &DacS0 {
        &self.dac_s0
    }
    #[doc = "0x438 - DAC Sideband 0 misc: Global volume sync and ZCD timeout"]
    #[inline(always)]
    pub const fn dac_s0_misc(&self) -> &DacS0Misc {
        &self.dac_s0_misc
    }
    #[doc = "0x43c - DAC Sideband 1: Main control for second secondary audio output channel"]
    #[inline(always)]
    pub const fn dac_s1(&self) -> &DacS1 {
        &self.dac_s1
    }
    #[doc = "0x440 - DAC Sideband 1 misc: Second channel zero-crossing timeout"]
    #[inline(always)]
    pub const fn dac_s1_misc(&self) -> &DacS1Misc {
        &self.dac_s1_misc
    }
    #[doc = "0x444 - DAC Dynamic Range Compression 0: Peak and average timing settings"]
    #[inline(always)]
    pub const fn dac_drc_0(&self) -> &DacDrc0 {
        &self.dac_drc_0
    }
    #[doc = "0x448 - DAC Dynamic Range Compression 1: Slope and threshold configuration"]
    #[inline(always)]
    pub const fn dac_drc_1(&self) -> &DacDrc1 {
        &self.dac_drc_1
    }
    #[doc = "0x44c - DAC Noise Gate 0: Hysteresis and source selection"]
    #[inline(always)]
    pub const fn dac_ngt_0(&self) -> &DacNgt0 {
        &self.dac_ngt_0
    }
    #[doc = "0x450 - DAC Zero Detection 0: Automatic mute during silence detection"]
    #[inline(always)]
    pub const fn dac_zd_0(&self) -> &DacZd0 {
        &self.dac_zd_0
    }
    #[doc = "0x600 - Voice Activity Detection Control 0"]
    #[inline(always)]
    pub const fn vad_ctrl_0(&self) -> &VadCtrl0 {
        &self.vad_ctrl_0
    }
    #[doc = "0x604 - Voice Activity Detection Control 1: Scaling and bias"]
    #[inline(always)]
    pub const fn vad_ctrl_1(&self) -> &VadCtrl1 {
        &self.vad_ctrl_1
    }
    #[doc = "0x608 - Voice Activity Detection Control 2: Threshold limits"]
    #[inline(always)]
    pub const fn vad_ctrl_2(&self) -> &VadCtrl2 {
        &self.vad_ctrl_2
    }
    #[doc = "0x60c - Voice Activity Detection Control 3: Smoothing factors"]
    #[inline(always)]
    pub const fn vad_ctrl_3(&self) -> &VadCtrl3 {
        &self.vad_ctrl_3
    }
    #[doc = "0x610 - Voice Activity Detection Control 4: EK Estimation status"]
    #[inline(always)]
    pub const fn vad_ctrl_4(&self) -> &VadCtrl4 {
        &self.vad_ctrl_4
    }
    #[doc = "0x614 - Voice Activity Detection Control 5: Noise threshold status"]
    #[inline(always)]
    pub const fn vad_ctrl_5(&self) -> &VadCtrl5 {
        &self.vad_ctrl_5
    }
    #[doc = "0x700 - Analog Codec Power Control 1: MicBias and PGA power"]
    #[inline(always)]
    pub const fn codec_power1(&self) -> &CodecPower1 {
        &self.codec_power1
    }
    #[doc = "0x704 - Analog Codec Power Control 2: Bandgap and LDO settings"]
    #[inline(always)]
    pub const fn codec_power2(&self) -> &CodecPower2 {
        &self.codec_power2
    }
    #[doc = "0x708 - Analog DAC Control: Line-out gain and source selection"]
    #[inline(always)]
    pub const fn codec_dac(&self) -> &CodecDac {
        &self.codec_dac
    }
    #[doc = "0x70c - Analog DAC Ramp Control: Anti-pop hardware ramping for line-out"]
    #[inline(always)]
    pub const fn codec_dac_ramp(&self) -> &CodecDacRamp {
        &self.codec_dac_ramp
    }
    #[doc = "0x710 - Analog ADC Channel 0 Control: PGA gain and analog front-end power"]
    #[inline(always)]
    pub const fn codec_adc0(&self) -> &CodecAdc0 {
        &self.codec_adc0
    }
    #[doc = "0x714 - Analog ADC Channel 1 Control: PGA gain and analog front-end power"]
    #[inline(always)]
    pub const fn codec_adc1(&self) -> &CodecAdc1 {
        &self.codec_adc1
    }
    #[doc = "0x718 - Analog ADC Channel 2 Control: PGA gain and analog front-end power"]
    #[inline(always)]
    pub const fn codec_adc2(&self) -> &CodecAdc2 {
        &self.codec_adc2
    }
    #[doc = "0x800 - Passthrough ADC Analog Config 1: Chopping and bias configuration"]
    #[inline(always)]
    pub const fn padc_ana_cfg1(&self) -> &PadcAnaCfg1 {
        &self.padc_ana_cfg1
    }
    #[doc = "0x804 - Passthrough ADC Analog Config 2: Dither and modulation settings"]
    #[inline(always)]
    pub const fn padc_ana_cfg2(&self) -> &PadcAnaCfg2 {
        &self.padc_ana_cfg2
    }
    #[doc = "0x808 - Passthrough ADC Analog Command: Start conversion and power triggers"]
    #[inline(always)]
    pub const fn padc_ana_cmd(&self) -> &PadcAnaCmd {
        &self.padc_ana_cmd
    }
    #[doc = "0x80c - Passthrough ADC Output Config: Data rate and filter types"]
    #[inline(always)]
    pub const fn padc_out_cfg(&self) -> &PadcOutCfg {
        &self.padc_out_cfg
    }
    #[doc = "0x810 - PADC Calibration Command"]
    #[inline(always)]
    pub const fn padc_calib_cmd(&self) -> &PadcCalibCmd {
        &self.padc_calib_cmd
    }
    #[doc = "0x814 - Passthrough ADC Status: Calibration and interrupt flags"]
    #[inline(always)]
    pub const fn padc_status(&self) -> &PadcStatus {
        &self.padc_status
    }
    #[doc = "0x818 - Passthrough ADC Data: 24-bit raw conversion result"]
    #[inline(always)]
    pub const fn padc_data(&self) -> &PadcData {
        &self.padc_data
    }
    #[doc = "0x81c - PADC Calibration Config 1: Offset Error Correction result"]
    #[inline(always)]
    pub const fn padc_calib_cfg1(&self) -> &PadcCalibCfg1 {
        &self.padc_calib_cfg1
    }
    #[doc = "0x820 - PADC Calibration Config 2: Gain Error Correction result"]
    #[inline(always)]
    pub const fn padc_calib_cfg2(&self) -> &PadcCalibCfg2 {
        &self.padc_calib_cfg2
    }
    #[doc = "0x824 - PADC Calibration Config 3: Reserved"]
    #[inline(always)]
    pub const fn padc_calib_cfg3(&self) -> &PadcCalibCfg3 {
        &self.padc_calib_cfg3
    }
    #[doc = "0x828 - PADC Calibration Config 4: Reserved"]
    #[inline(always)]
    pub const fn padc_calib_cfg4(&self) -> &PadcCalibCfg4 {
        &self.padc_calib_cfg4
    }
    #[doc = "0x82c - PADC Calibration Config 5: Gain calibration reference"]
    #[inline(always)]
    pub const fn padc_calib_cfg5(&self) -> &PadcCalibCfg5 {
        &self.padc_calib_cfg5
    }
    #[doc = "0x830 - PADC Calibration Config 6: Reserved"]
    #[inline(always)]
    pub const fn padc_calib_cfg6(&self) -> &PadcCalibCfg6 {
        &self.padc_calib_cfg6
    }
    #[doc = "0x834 - PADC Calibration Config 7: Reserved"]
    #[inline(always)]
    pub const fn padc_calib_cfg7(&self) -> &PadcCalibCfg7 {
        &self.padc_calib_cfg7
    }
    #[doc = "0x838 - PADC Offset Error Correction W1P Trigger"]
    #[inline(always)]
    pub const fn padc_oec_w1p(&self) -> &PadcOecW1p {
        &self.padc_oec_w1p
    }
    #[doc = "0x83c - PADC Gain Error Correction W1P Trigger"]
    #[inline(always)]
    pub const fn padc_gec_w1p(&self) -> &PadcGecW1p {
        &self.padc_gec_w1p
    }
    #[doc = "0x840 - PADC Offset Error Correction Config 0"]
    #[inline(always)]
    pub const fn padc_oec_cfg0(&self) -> &PadcOecCfg0 {
        &self.padc_oec_cfg0
    }
    #[doc = "0x844 - PADC Offset Error Correction Config 1"]
    #[inline(always)]
    pub const fn padc_oec_cfg1(&self) -> &PadcOecCfg1 {
        &self.padc_oec_cfg1
    }
    #[doc = "0x848 - PADC Offset Error Correction Config 2"]
    #[inline(always)]
    pub const fn padc_oec_cfg2(&self) -> &PadcOecCfg2 {
        &self.padc_oec_cfg2
    }
    #[doc = "0x84c - PADC Offset Error Correction Config 3"]
    #[inline(always)]
    pub const fn padc_oec_cfg3(&self) -> &PadcOecCfg3 {
        &self.padc_oec_cfg3
    }
    #[doc = "0x850 - PADC Offset Error Correction Config 4"]
    #[inline(always)]
    pub const fn padc_oec_cfg4(&self) -> &PadcOecCfg4 {
        &self.padc_oec_cfg4
    }
    #[doc = "0x854 - PADC Offset Error Correction Config 5"]
    #[inline(always)]
    pub const fn padc_oec_cfg5(&self) -> &PadcOecCfg5 {
        &self.padc_oec_cfg5
    }
    #[doc = "0x858 - PADC Offset Error Correction Config 6"]
    #[inline(always)]
    pub const fn padc_oec_cfg6(&self) -> &PadcOecCfg6 {
        &self.padc_oec_cfg6
    }
    #[doc = "0x85c - PADC Offset Error Correction Config 7"]
    #[inline(always)]
    pub const fn padc_oec_cfg7(&self) -> &PadcOecCfg7 {
        &self.padc_oec_cfg7
    }
    #[doc = "0x860 - PADC Gain Error Correction Config 0"]
    #[inline(always)]
    pub const fn padc_gec_cfg0(&self) -> &PadcGecCfg0 {
        &self.padc_gec_cfg0
    }
    #[doc = "0x864 - PADC Gain Error Correction Config 1"]
    #[inline(always)]
    pub const fn padc_gec_cfg1(&self) -> &PadcGecCfg1 {
        &self.padc_gec_cfg1
    }
    #[doc = "0x868 - PADC Gain Error Correction Config 2"]
    #[inline(always)]
    pub const fn padc_gec_cfg2(&self) -> &PadcGecCfg2 {
        &self.padc_gec_cfg2
    }
    #[doc = "0x86c - PADC Gain Error Correction Config 3"]
    #[inline(always)]
    pub const fn padc_gec_cfg3(&self) -> &PadcGecCfg3 {
        &self.padc_gec_cfg3
    }
    #[doc = "0x870 - PADC Gain Error Correction Config 4"]
    #[inline(always)]
    pub const fn padc_gec_cfg4(&self) -> &PadcGecCfg4 {
        &self.padc_gec_cfg4
    }
    #[doc = "0x874 - PADC Gain Error Correction Config 5"]
    #[inline(always)]
    pub const fn padc_gec_cfg5(&self) -> &PadcGecCfg5 {
        &self.padc_gec_cfg5
    }
    #[doc = "0x878 - PADC Gain Error Correction Config 6"]
    #[inline(always)]
    pub const fn padc_gec_cfg6(&self) -> &PadcGecCfg6 {
        &self.padc_gec_cfg6
    }
    #[doc = "0x87c - PADC Gain Error Correction Config 7"]
    #[inline(always)]
    pub const fn padc_gec_cfg7(&self) -> &PadcGecCfg7 {
        &self.padc_gec_cfg7
    }
    #[doc = "0x900 - Passthrough ADC RX FIFO Control: DMA and interrupt triggers"]
    #[inline(always)]
    pub const fn padc_rx_fifo_ctrl(&self) -> &PadcRxFifoCtrl {
        &self.padc_rx_fifo_ctrl
    }
    #[doc = "0x904 - Passthrough ADC RX FIFO Status: Occupancy and error status"]
    #[inline(always)]
    pub const fn padc_rx_fifo_status(&self) -> &PadcRxFifoStatus {
        &self.padc_rx_fifo_status
    }
    #[doc = "0x908 - Passthrough ADC RX FIFO Data Port: Read portal for conversion streams"]
    #[inline(always)]
    pub const fn padc_rx_fifo_data(&self) -> &PadcRxFifoData {
        &self.padc_rx_fifo_data
    }
}
#[doc = "aud_top (rw) register accessor: Audio Top Level Configuration: Clock gating and sample rate management\n\nYou can [`read`](crate::Reg::read) this register and get [`aud_top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aud_top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_top`] module"]
#[doc(alias = "aud_top")]
pub type AudTop = crate::Reg<aud_top::AudTopSpec>;
#[doc = "Audio Top Level Configuration: Clock gating and sample rate management"]
pub mod aud_top;
#[doc = "aud_itf (rw) register accessor: Audio Interface Configuration: Channel routing and enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`aud_itf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aud_itf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aud_itf`] module"]
#[doc(alias = "aud_itf")]
pub type AudItf = crate::Reg<aud_itf::AudItfSpec>;
#[doc = "Audio Interface Configuration: Channel routing and enable control"]
pub mod aud_itf;
#[doc = "adc_0 (rw) register accessor: ADC Filter and Scaling Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_0`] module"]
#[doc(alias = "adc_0")]
pub type Adc0 = crate::Reg<adc_0::Adc0Spec>;
#[doc = "ADC Filter and Scaling Configuration 0"]
pub mod adc_0;
#[doc = "adc_1 (rw) register accessor: ADC Analog Gain and Control Loop Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_1`] module"]
#[doc(alias = "adc_1")]
pub type Adc1 = crate::Reg<adc_1::Adc1Spec>;
#[doc = "ADC Analog Gain and Control Loop Config 1"]
pub mod adc_1;
#[doc = "dac_0 (rw) register accessor: DAC Configuration 0: Digital-to-Analog conversion parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_0`] module"]
#[doc(alias = "dac_0")]
pub type Dac0 = crate::Reg<dac_0::Dac0Spec>;
#[doc = "DAC Configuration 0: Digital-to-Analog conversion parameters"]
pub mod dac_0;
#[doc = "dac_1 (rw) register accessor: DAC Configuration 1: Modulation and order settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_1`] module"]
#[doc(alias = "dac_1")]
pub type Dac1 = crate::Reg<dac_1::Dac1Spec>;
#[doc = "DAC Configuration 1: Modulation and order settings"]
pub mod dac_1;
#[doc = "aec_0 (rw) register accessor: AEC Configuration 0: Acoustic Echo Cancellation control\n\nYou can [`read`](crate::Reg::read) this register and get [`aec_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aec_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aec_0`] module"]
#[doc(alias = "aec_0")]
pub type Aec0 = crate::Reg<aec_0::Aec0Spec>;
#[doc = "AEC Configuration 0: Acoustic Echo Cancellation control"]
pub mod aec_0;
#[doc = "pdm_0 (rw) register accessor: PDM Configuration 0: Pulse Density Modulation interface setup\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdm_0`] module"]
#[doc(alias = "pdm_0")]
pub type Pdm0 = crate::Reg<pdm_0::Pdm0Spec>;
#[doc = "PDM Configuration 0: Pulse Density Modulation interface setup"]
pub mod pdm_0;
#[doc = "rsvd0 (rw) register accessor: Reserved: Do not modify\n\nYou can [`read`](crate::Reg::read) this register and get [`rsvd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsvd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsvd0`] module"]
#[doc(alias = "rsvd0")]
pub type Rsvd0 = crate::Reg<rsvd0::Rsvd0Spec>;
#[doc = "Reserved: Do not modify"]
pub mod rsvd0;
#[doc = "dbg_0 (rw) register accessor: Audio System Debug Control: Sine generator and test point routing\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_0`] module"]
#[doc(alias = "dbg_0")]
pub type Dbg0 = crate::Reg<dbg_0::Dbg0Spec>;
#[doc = "Audio System Debug Control: Sine generator and test point routing"]
pub mod dbg_0;
#[doc = "dbg_1 (r) register accessor: Debug Register 1: Audio internal data read port\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_1`] module"]
#[doc(alias = "dbg_1")]
pub type Dbg1 = crate::Reg<dbg_1::Dbg1Spec>;
#[doc = "Debug Register 1: Audio internal data read port"]
pub mod dbg_1;
#[doc = "dbg_2 (rw) register accessor: Debug Register 2: FIR filter and ADC test selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_2`] module"]
#[doc(alias = "dbg_2")]
pub type Dbg2 = crate::Reg<dbg_2::Dbg2Spec>;
#[doc = "Debug Register 2: FIR filter and ADC test selection"]
pub mod dbg_2;
#[doc = "dbg_3 (rw) register accessor: Debug Register 3: DAC DWA and input test selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_3`] module"]
#[doc(alias = "dbg_3")]
pub type Dbg3 = crate::Reg<dbg_3::Dbg3Spec>;
#[doc = "Debug Register 3: DAC DWA and input test selection"]
pub mod dbg_3;
#[doc = "dbg_4 (rw) register accessor: Debug Register 4: Audio synchronization ratio monitoring\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_4`] module"]
#[doc(alias = "dbg_4")]
pub type Dbg4 = crate::Reg<dbg_4::Dbg4Spec>;
#[doc = "Debug Register 4: Audio synchronization ratio monitoring"]
pub mod dbg_4;
#[doc = "rx_fifo_ctrl (rw) register accessor: RX FIFO Control: Configuration for audio data reception and DMA thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_ctrl`] module"]
#[doc(alias = "rx_fifo_ctrl")]
pub type RxFifoCtrl = crate::Reg<rx_fifo_ctrl::RxFifoCtrlSpec>;
#[doc = "RX FIFO Control: Configuration for audio data reception and DMA thresholds"]
pub mod rx_fifo_ctrl;
#[doc = "rx_fifo_status (r) register accessor: RX FIFO Status: Overflow, underflow, and item count\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_status`] module"]
#[doc(alias = "rx_fifo_status")]
pub type RxFifoStatus = crate::Reg<rx_fifo_status::RxFifoStatusSpec>;
#[doc = "RX FIFO Status: Overflow, underflow, and item count"]
pub mod rx_fifo_status;
#[doc = "rx_fifo_data (r) register accessor: RX FIFO Data: Read portal for incoming audio streams\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_data`] module"]
#[doc(alias = "rx_fifo_data")]
pub type RxFifoData = crate::Reg<rx_fifo_data::RxFifoDataSpec>;
#[doc = "RX FIFO Data: Read portal for incoming audio streams"]
pub mod rx_fifo_data;
#[doc = "tx_fifo_ctrl (rw) register accessor: TX FIFO Control: Configuration for audio data transmission and DMA thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_ctrl`] module"]
#[doc(alias = "tx_fifo_ctrl")]
pub type TxFifoCtrl = crate::Reg<tx_fifo_ctrl::TxFifoCtrlSpec>;
#[doc = "TX FIFO Control: Configuration for audio data transmission and DMA thresholds"]
pub mod tx_fifo_ctrl;
#[doc = "tx_fifo_status (r) register accessor: TX FIFO Status: Request status and available space count\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_status`] module"]
#[doc(alias = "tx_fifo_status")]
pub type TxFifoStatus = crate::Reg<tx_fifo_status::TxFifoStatusSpec>;
#[doc = "TX FIFO Status: Request status and available space count"]
pub mod tx_fifo_status;
#[doc = "tx_fifo_data (w) register accessor: TX FIFO Data: Write portal for outgoing audio streams\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_data`] module"]
#[doc(alias = "tx_fifo_data")]
pub type TxFifoData = crate::Reg<tx_fifo_data::TxFifoDataSpec>;
#[doc = "TX FIFO Data: Write portal for outgoing audio streams"]
pub mod tx_fifo_data;
#[doc = "Parametric Equalizer Bank %s: Sets the IIR biquad coefficients for the audio DSP chain"]
pub use self::peq_::Peq_;
#[doc = r"Cluster"]
#[doc = "Parametric Equalizer Bank %s: Sets the IIR biquad coefficients for the audio DSP chain"]
pub mod peq_;
#[doc = "adc_h (rw) register accessor: ADC High-rate path: Main audio volume and mute control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_h`] module"]
#[doc(alias = "adc_h")]
pub type AdcH = crate::Reg<adc_h::AdcHSpec>;
#[doc = "ADC High-rate path: Main audio volume and mute control"]
pub mod adc_h;
#[doc = "adc_h_misc (rw) register accessor: ADC High-rate miscellaneous: Zero-crossing timeouts\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_h_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_h_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_h_misc`] module"]
#[doc(alias = "adc_h_misc")]
pub type AdcHMisc = crate::Reg<adc_h_misc::AdcHMiscSpec>;
#[doc = "ADC High-rate miscellaneous: Zero-crossing timeouts"]
pub mod adc_h_misc;
#[doc = "adc_s0 (rw) register accessor: ADC Sideband 0: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s0`] module"]
#[doc(alias = "adc_s0")]
pub type AdcS0 = crate::Reg<adc_s0::AdcS0Spec>;
#[doc = "ADC Sideband 0: Volume and mute control for secondary audio path"]
pub mod adc_s0;
#[doc = "adc_s0_misc (rw) register accessor: ADC Sideband 0 misc: Global mute and update synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s0_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s0_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s0_misc`] module"]
#[doc(alias = "adc_s0_misc")]
pub type AdcS0Misc = crate::Reg<adc_s0_misc::AdcS0MiscSpec>;
#[doc = "ADC Sideband 0 misc: Global mute and update synchronization"]
pub mod adc_s0_misc;
#[doc = "adc_s1 (rw) register accessor: ADC Sideband 1: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s1`] module"]
#[doc(alias = "adc_s1")]
pub type AdcS1 = crate::Reg<adc_s1::AdcS1Spec>;
#[doc = "ADC Sideband 1: Volume and mute control for secondary audio path"]
pub mod adc_s1;
#[doc = "adc_s1_misc (rw) register accessor: ADC Sideband 1 misc: Zero-crossing timeout settings\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s1_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s1_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s1_misc`] module"]
#[doc(alias = "adc_s1_misc")]
pub type AdcS1Misc = crate::Reg<adc_s1_misc::AdcS1MiscSpec>;
#[doc = "ADC Sideband 1 misc: Zero-crossing timeout settings"]
pub mod adc_s1_misc;
#[doc = "adc_s2 (rw) register accessor: ADC Sideband 2: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s2`] module"]
#[doc(alias = "adc_s2")]
pub type AdcS2 = crate::Reg<adc_s2::AdcS2Spec>;
#[doc = "ADC Sideband 2: Volume and mute control for secondary audio path"]
pub mod adc_s2;
#[doc = "adc_s2_misc (rw) register accessor: ADC Sideband 2 misc: Zero-crossing timeout settings\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s2_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s2_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_s2_misc`] module"]
#[doc(alias = "adc_s2_misc")]
pub type AdcS2Misc = crate::Reg<adc_s2_misc::AdcS2MiscSpec>;
#[doc = "ADC Sideband 2 misc: Zero-crossing timeout settings"]
pub mod adc_s2_misc;
#[doc = "adc_agc_0 (rw) register accessor: ADC Automatic Gain Control 0: Thresholds and forced gain management\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_agc_0`] module"]
#[doc(alias = "adc_agc_0")]
pub type AdcAgc0 = crate::Reg<adc_agc_0::AdcAgc0Spec>;
#[doc = "ADC Automatic Gain Control 0: Thresholds and forced gain management"]
pub mod adc_agc_0;
#[doc = "adc_agc_1 (rw) register accessor: ADC Automatic Gain Control 1: Attack, decay, and hold timing\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_agc_1`] module"]
#[doc(alias = "adc_agc_1")]
pub type AdcAgc1 = crate::Reg<adc_agc_1::AdcAgc1Spec>;
#[doc = "ADC Automatic Gain Control 1: Attack, decay, and hold timing"]
pub mod adc_agc_1;
#[doc = "adc_agc_2 (rw) register accessor: ADC Automatic Gain Control 2: Noise gate and silence gain\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_agc_2`] module"]
#[doc(alias = "adc_agc_2")]
pub type AdcAgc2 = crate::Reg<adc_agc_2::AdcAgc2Spec>;
#[doc = "ADC Automatic Gain Control 2: Noise gate and silence gain"]
pub mod adc_agc_2;
#[doc = "adc_agc_3 (rw) register accessor: ADC Automatic Gain Control 3: Detection mode and noise exit timing\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_agc_3`] module"]
#[doc(alias = "adc_agc_3")]
pub type AdcAgc3 = crate::Reg<adc_agc_3::AdcAgc3Spec>;
#[doc = "ADC Automatic Gain Control 3: Detection mode and noise exit timing"]
pub mod adc_agc_3;
#[doc = "audio_status (rw) register accessor: Audio Master Status and Interrupt: Clearing bits for all ADC/DAC events\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_status`] module"]
#[doc(alias = "audio_status")]
pub type AudioStatus = crate::Reg<audio_status::AudioStatusSpec>;
#[doc = "Audio Master Status and Interrupt: Clearing bits for all ADC/DAC events"]
pub mod audio_status;
#[doc = "dac_s0 (rw) register accessor: DAC Sideband 0: Main control for secondary audio output channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_s0`] module"]
#[doc(alias = "dac_s0")]
pub type DacS0 = crate::Reg<dac_s0::DacS0Spec>;
#[doc = "DAC Sideband 0: Main control for secondary audio output channel"]
pub mod dac_s0;
#[doc = "dac_s0_misc (rw) register accessor: DAC Sideband 0 misc: Global volume sync and ZCD timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s0_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s0_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_s0_misc`] module"]
#[doc(alias = "dac_s0_misc")]
pub type DacS0Misc = crate::Reg<dac_s0_misc::DacS0MiscSpec>;
#[doc = "DAC Sideband 0 misc: Global volume sync and ZCD timeout"]
pub mod dac_s0_misc;
#[doc = "dac_s1 (rw) register accessor: DAC Sideband 1: Main control for second secondary audio output channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_s1`] module"]
#[doc(alias = "dac_s1")]
pub type DacS1 = crate::Reg<dac_s1::DacS1Spec>;
#[doc = "DAC Sideband 1: Main control for second secondary audio output channel"]
pub mod dac_s1;
#[doc = "dac_s1_misc (rw) register accessor: DAC Sideband 1 misc: Second channel zero-crossing timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s1_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s1_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_s1_misc`] module"]
#[doc(alias = "dac_s1_misc")]
pub type DacS1Misc = crate::Reg<dac_s1_misc::DacS1MiscSpec>;
#[doc = "DAC Sideband 1 misc: Second channel zero-crossing timeout"]
pub mod dac_s1_misc;
#[doc = "dac_drc_0 (rw) register accessor: DAC Dynamic Range Compression 0: Peak and average timing settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_drc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_drc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_drc_0`] module"]
#[doc(alias = "dac_drc_0")]
pub type DacDrc0 = crate::Reg<dac_drc_0::DacDrc0Spec>;
#[doc = "DAC Dynamic Range Compression 0: Peak and average timing settings"]
pub mod dac_drc_0;
#[doc = "dac_drc_1 (rw) register accessor: DAC Dynamic Range Compression 1: Slope and threshold configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_drc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_drc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_drc_1`] module"]
#[doc(alias = "dac_drc_1")]
pub type DacDrc1 = crate::Reg<dac_drc_1::DacDrc1Spec>;
#[doc = "DAC Dynamic Range Compression 1: Slope and threshold configuration"]
pub mod dac_drc_1;
#[doc = "dac_ngt_0 (rw) register accessor: DAC Noise Gate 0: Hysteresis and source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_ngt_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ngt_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_ngt_0`] module"]
#[doc(alias = "dac_ngt_0")]
pub type DacNgt0 = crate::Reg<dac_ngt_0::DacNgt0Spec>;
#[doc = "DAC Noise Gate 0: Hysteresis and source selection"]
pub mod dac_ngt_0;
#[doc = "dac_zd_0 (rw) register accessor: DAC Zero Detection 0: Automatic mute during silence detection\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_zd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_zd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_zd_0`] module"]
#[doc(alias = "dac_zd_0")]
pub type DacZd0 = crate::Reg<dac_zd_0::DacZd0Spec>;
#[doc = "DAC Zero Detection 0: Automatic mute during silence detection"]
pub mod dac_zd_0;
#[doc = "vad_ctrl_0 (rw) register accessor: Voice Activity Detection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_0`] module"]
#[doc(alias = "vad_ctrl_0")]
pub type VadCtrl0 = crate::Reg<vad_ctrl_0::VadCtrl0Spec>;
#[doc = "Voice Activity Detection Control 0"]
pub mod vad_ctrl_0;
#[doc = "vad_ctrl_1 (rw) register accessor: Voice Activity Detection Control 1: Scaling and bias\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_1`] module"]
#[doc(alias = "vad_ctrl_1")]
pub type VadCtrl1 = crate::Reg<vad_ctrl_1::VadCtrl1Spec>;
#[doc = "Voice Activity Detection Control 1: Scaling and bias"]
pub mod vad_ctrl_1;
#[doc = "vad_ctrl_2 (rw) register accessor: Voice Activity Detection Control 2: Threshold limits\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_2`] module"]
#[doc(alias = "vad_ctrl_2")]
pub type VadCtrl2 = crate::Reg<vad_ctrl_2::VadCtrl2Spec>;
#[doc = "Voice Activity Detection Control 2: Threshold limits"]
pub mod vad_ctrl_2;
#[doc = "vad_ctrl_3 (rw) register accessor: Voice Activity Detection Control 3: Smoothing factors\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_3`] module"]
#[doc(alias = "vad_ctrl_3")]
pub type VadCtrl3 = crate::Reg<vad_ctrl_3::VadCtrl3Spec>;
#[doc = "Voice Activity Detection Control 3: Smoothing factors"]
pub mod vad_ctrl_3;
#[doc = "vad_ctrl_4 (r) register accessor: Voice Activity Detection Control 4: EK Estimation status\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_4`] module"]
#[doc(alias = "vad_ctrl_4")]
pub type VadCtrl4 = crate::Reg<vad_ctrl_4::VadCtrl4Spec>;
#[doc = "Voice Activity Detection Control 4: EK Estimation status"]
pub mod vad_ctrl_4;
#[doc = "vad_ctrl_5 (r) register accessor: Voice Activity Detection Control 5: Noise threshold status\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vad_ctrl_5`] module"]
#[doc(alias = "vad_ctrl_5")]
pub type VadCtrl5 = crate::Reg<vad_ctrl_5::VadCtrl5Spec>;
#[doc = "Voice Activity Detection Control 5: Noise threshold status"]
pub mod vad_ctrl_5;
#[doc = "codec_power1 (rw) register accessor: Analog Codec Power Control 1: MicBias and PGA power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_power1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_power1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_power1`] module"]
#[doc(alias = "codec_power1")]
pub type CodecPower1 = crate::Reg<codec_power1::CodecPower1Spec>;
#[doc = "Analog Codec Power Control 1: MicBias and PGA power"]
pub mod codec_power1;
#[doc = "codec_power2 (rw) register accessor: Analog Codec Power Control 2: Bandgap and LDO settings\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_power2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_power2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_power2`] module"]
#[doc(alias = "codec_power2")]
pub type CodecPower2 = crate::Reg<codec_power2::CodecPower2Spec>;
#[doc = "Analog Codec Power Control 2: Bandgap and LDO settings"]
pub mod codec_power2;
#[doc = "codec_dac (rw) register accessor: Analog DAC Control: Line-out gain and source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_dac`] module"]
#[doc(alias = "codec_dac")]
pub type CodecDac = crate::Reg<codec_dac::CodecDacSpec>;
#[doc = "Analog DAC Control: Line-out gain and source selection"]
pub mod codec_dac;
#[doc = "codec_dac_ramp (rw) register accessor: Analog DAC Ramp Control: Anti-pop hardware ramping for line-out\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_dac_ramp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_dac_ramp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_dac_ramp`] module"]
#[doc(alias = "codec_dac_ramp")]
pub type CodecDacRamp = crate::Reg<codec_dac_ramp::CodecDacRampSpec>;
#[doc = "Analog DAC Ramp Control: Anti-pop hardware ramping for line-out"]
pub mod codec_dac_ramp;
#[doc = "codec_adc0 (rw) register accessor: Analog ADC Channel 0 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_adc0`] module"]
#[doc(alias = "codec_adc0")]
pub type CodecAdc0 = crate::Reg<codec_adc0::CodecAdc0Spec>;
#[doc = "Analog ADC Channel 0 Control: PGA gain and analog front-end power"]
pub mod codec_adc0;
#[doc = "codec_adc1 (rw) register accessor: Analog ADC Channel 1 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_adc1`] module"]
#[doc(alias = "codec_adc1")]
pub type CodecAdc1 = crate::Reg<codec_adc1::CodecAdc1Spec>;
#[doc = "Analog ADC Channel 1 Control: PGA gain and analog front-end power"]
pub mod codec_adc1;
#[doc = "codec_adc2 (rw) register accessor: Analog ADC Channel 2 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_adc2`] module"]
#[doc(alias = "codec_adc2")]
pub type CodecAdc2 = crate::Reg<codec_adc2::CodecAdc2Spec>;
#[doc = "Analog ADC Channel 2 Control: PGA gain and analog front-end power"]
pub mod codec_adc2;
#[doc = "padc_ana_cfg1 (rw) register accessor: Passthrough ADC Analog Config 1: Chopping and bias configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_ana_cfg1`] module"]
#[doc(alias = "padc_ana_cfg1")]
pub type PadcAnaCfg1 = crate::Reg<padc_ana_cfg1::PadcAnaCfg1Spec>;
#[doc = "Passthrough ADC Analog Config 1: Chopping and bias configuration"]
pub mod padc_ana_cfg1;
#[doc = "padc_ana_cfg2 (rw) register accessor: Passthrough ADC Analog Config 2: Dither and modulation settings\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_ana_cfg2`] module"]
#[doc(alias = "padc_ana_cfg2")]
pub type PadcAnaCfg2 = crate::Reg<padc_ana_cfg2::PadcAnaCfg2Spec>;
#[doc = "Passthrough ADC Analog Config 2: Dither and modulation settings"]
pub mod padc_ana_cfg2;
#[doc = "padc_ana_cmd (rw) register accessor: Passthrough ADC Analog Command: Start conversion and power triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_ana_cmd`] module"]
#[doc(alias = "padc_ana_cmd")]
pub type PadcAnaCmd = crate::Reg<padc_ana_cmd::PadcAnaCmdSpec>;
#[doc = "Passthrough ADC Analog Command: Start conversion and power triggers"]
pub mod padc_ana_cmd;
#[doc = "padc_out_cfg (rw) register accessor: Passthrough ADC Output Config: Data rate and filter types\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_out_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_out_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_out_cfg`] module"]
#[doc(alias = "padc_out_cfg")]
pub type PadcOutCfg = crate::Reg<padc_out_cfg::PadcOutCfgSpec>;
#[doc = "Passthrough ADC Output Config: Data rate and filter types"]
pub mod padc_out_cfg;
#[doc = "padc_status (rw) register accessor: Passthrough ADC Status: Calibration and interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_status`] module"]
#[doc(alias = "padc_status")]
pub type PadcStatus = crate::Reg<padc_status::PadcStatusSpec>;
#[doc = "Passthrough ADC Status: Calibration and interrupt flags"]
pub mod padc_status;
#[doc = "padc_data (r) register accessor: Passthrough ADC Data: 24-bit raw conversion result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_data`] module"]
#[doc(alias = "padc_data")]
pub type PadcData = crate::Reg<padc_data::PadcDataSpec>;
#[doc = "Passthrough ADC Data: 24-bit raw conversion result"]
pub mod padc_data;
#[doc = "padc_calib_cmd (rw) register accessor: PADC Calibration Command\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cmd`] module"]
#[doc(alias = "padc_calib_cmd")]
pub type PadcCalibCmd = crate::Reg<padc_calib_cmd::PadcCalibCmdSpec>;
#[doc = "PADC Calibration Command"]
pub mod padc_calib_cmd;
#[doc = "padc_calib_cfg1 (r) register accessor: PADC Calibration Config 1: Offset Error Correction result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg1`] module"]
#[doc(alias = "padc_calib_cfg1")]
pub type PadcCalibCfg1 = crate::Reg<padc_calib_cfg1::PadcCalibCfg1Spec>;
#[doc = "PADC Calibration Config 1: Offset Error Correction result"]
pub mod padc_calib_cfg1;
#[doc = "padc_calib_cfg2 (r) register accessor: PADC Calibration Config 2: Gain Error Correction result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg2`] module"]
#[doc(alias = "padc_calib_cfg2")]
pub type PadcCalibCfg2 = crate::Reg<padc_calib_cfg2::PadcCalibCfg2Spec>;
#[doc = "PADC Calibration Config 2: Gain Error Correction result"]
pub mod padc_calib_cfg2;
#[doc = "padc_calib_cfg3 (rw) register accessor: PADC Calibration Config 3: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg3`] module"]
#[doc(alias = "padc_calib_cfg3")]
pub type PadcCalibCfg3 = crate::Reg<padc_calib_cfg3::PadcCalibCfg3Spec>;
#[doc = "PADC Calibration Config 3: Reserved"]
pub mod padc_calib_cfg3;
#[doc = "padc_calib_cfg4 (rw) register accessor: PADC Calibration Config 4: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg4`] module"]
#[doc(alias = "padc_calib_cfg4")]
pub type PadcCalibCfg4 = crate::Reg<padc_calib_cfg4::PadcCalibCfg4Spec>;
#[doc = "PADC Calibration Config 4: Reserved"]
pub mod padc_calib_cfg4;
#[doc = "padc_calib_cfg5 (rw) register accessor: PADC Calibration Config 5: Gain calibration reference\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg5`] module"]
#[doc(alias = "padc_calib_cfg5")]
pub type PadcCalibCfg5 = crate::Reg<padc_calib_cfg5::PadcCalibCfg5Spec>;
#[doc = "PADC Calibration Config 5: Gain calibration reference"]
pub mod padc_calib_cfg5;
#[doc = "padc_calib_cfg6 (rw) register accessor: PADC Calibration Config 6: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg6`] module"]
#[doc(alias = "padc_calib_cfg6")]
pub type PadcCalibCfg6 = crate::Reg<padc_calib_cfg6::PadcCalibCfg6Spec>;
#[doc = "PADC Calibration Config 6: Reserved"]
pub mod padc_calib_cfg6;
#[doc = "padc_calib_cfg7 (rw) register accessor: PADC Calibration Config 7: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_calib_cfg7`] module"]
#[doc(alias = "padc_calib_cfg7")]
pub type PadcCalibCfg7 = crate::Reg<padc_calib_cfg7::PadcCalibCfg7Spec>;
#[doc = "PADC Calibration Config 7: Reserved"]
pub mod padc_calib_cfg7;
#[doc = "padc_oec_w1p (rw) register accessor: PADC Offset Error Correction W1P Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_w1p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_oec_w1p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_w1p`] module"]
#[doc(alias = "padc_oec_w1p")]
pub type PadcOecW1p = crate::Reg<padc_oec_w1p::PadcOecW1pSpec>;
#[doc = "PADC Offset Error Correction W1P Trigger"]
pub mod padc_oec_w1p;
#[doc = "padc_gec_w1p (rw) register accessor: PADC Gain Error Correction W1P Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_w1p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_gec_w1p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_w1p`] module"]
#[doc(alias = "padc_gec_w1p")]
pub type PadcGecW1p = crate::Reg<padc_gec_w1p::PadcGecW1pSpec>;
#[doc = "PADC Gain Error Correction W1P Trigger"]
pub mod padc_gec_w1p;
#[doc = "padc_oec_cfg0 (r) register accessor: PADC Offset Error Correction Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg0`] module"]
#[doc(alias = "padc_oec_cfg0")]
pub type PadcOecCfg0 = crate::Reg<padc_oec_cfg0::PadcOecCfg0Spec>;
#[doc = "PADC Offset Error Correction Config 0"]
pub mod padc_oec_cfg0;
#[doc = "padc_oec_cfg1 (r) register accessor: PADC Offset Error Correction Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg1`] module"]
#[doc(alias = "padc_oec_cfg1")]
pub type PadcOecCfg1 = crate::Reg<padc_oec_cfg1::PadcOecCfg1Spec>;
#[doc = "PADC Offset Error Correction Config 1"]
pub mod padc_oec_cfg1;
#[doc = "padc_oec_cfg2 (r) register accessor: PADC Offset Error Correction Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg2`] module"]
#[doc(alias = "padc_oec_cfg2")]
pub type PadcOecCfg2 = crate::Reg<padc_oec_cfg2::PadcOecCfg2Spec>;
#[doc = "PADC Offset Error Correction Config 2"]
pub mod padc_oec_cfg2;
#[doc = "padc_oec_cfg3 (r) register accessor: PADC Offset Error Correction Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg3`] module"]
#[doc(alias = "padc_oec_cfg3")]
pub type PadcOecCfg3 = crate::Reg<padc_oec_cfg3::PadcOecCfg3Spec>;
#[doc = "PADC Offset Error Correction Config 3"]
pub mod padc_oec_cfg3;
#[doc = "padc_oec_cfg4 (r) register accessor: PADC Offset Error Correction Config 4\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg4`] module"]
#[doc(alias = "padc_oec_cfg4")]
pub type PadcOecCfg4 = crate::Reg<padc_oec_cfg4::PadcOecCfg4Spec>;
#[doc = "PADC Offset Error Correction Config 4"]
pub mod padc_oec_cfg4;
#[doc = "padc_oec_cfg5 (r) register accessor: PADC Offset Error Correction Config 5\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg5`] module"]
#[doc(alias = "padc_oec_cfg5")]
pub type PadcOecCfg5 = crate::Reg<padc_oec_cfg5::PadcOecCfg5Spec>;
#[doc = "PADC Offset Error Correction Config 5"]
pub mod padc_oec_cfg5;
#[doc = "padc_oec_cfg6 (r) register accessor: PADC Offset Error Correction Config 6\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg6`] module"]
#[doc(alias = "padc_oec_cfg6")]
pub type PadcOecCfg6 = crate::Reg<padc_oec_cfg6::PadcOecCfg6Spec>;
#[doc = "PADC Offset Error Correction Config 6"]
pub mod padc_oec_cfg6;
#[doc = "padc_oec_cfg7 (r) register accessor: PADC Offset Error Correction Config 7\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_oec_cfg7`] module"]
#[doc(alias = "padc_oec_cfg7")]
pub type PadcOecCfg7 = crate::Reg<padc_oec_cfg7::PadcOecCfg7Spec>;
#[doc = "PADC Offset Error Correction Config 7"]
pub mod padc_oec_cfg7;
#[doc = "padc_gec_cfg0 (r) register accessor: PADC Gain Error Correction Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg0`] module"]
#[doc(alias = "padc_gec_cfg0")]
pub type PadcGecCfg0 = crate::Reg<padc_gec_cfg0::PadcGecCfg0Spec>;
#[doc = "PADC Gain Error Correction Config 0"]
pub mod padc_gec_cfg0;
#[doc = "padc_gec_cfg1 (r) register accessor: PADC Gain Error Correction Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg1`] module"]
#[doc(alias = "padc_gec_cfg1")]
pub type PadcGecCfg1 = crate::Reg<padc_gec_cfg1::PadcGecCfg1Spec>;
#[doc = "PADC Gain Error Correction Config 1"]
pub mod padc_gec_cfg1;
#[doc = "padc_gec_cfg2 (r) register accessor: PADC Gain Error Correction Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg2`] module"]
#[doc(alias = "padc_gec_cfg2")]
pub type PadcGecCfg2 = crate::Reg<padc_gec_cfg2::PadcGecCfg2Spec>;
#[doc = "PADC Gain Error Correction Config 2"]
pub mod padc_gec_cfg2;
#[doc = "padc_gec_cfg3 (r) register accessor: PADC Gain Error Correction Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg3`] module"]
#[doc(alias = "padc_gec_cfg3")]
pub type PadcGecCfg3 = crate::Reg<padc_gec_cfg3::PadcGecCfg3Spec>;
#[doc = "PADC Gain Error Correction Config 3"]
pub mod padc_gec_cfg3;
#[doc = "padc_gec_cfg4 (r) register accessor: PADC Gain Error Correction Config 4\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg4`] module"]
#[doc(alias = "padc_gec_cfg4")]
pub type PadcGecCfg4 = crate::Reg<padc_gec_cfg4::PadcGecCfg4Spec>;
#[doc = "PADC Gain Error Correction Config 4"]
pub mod padc_gec_cfg4;
#[doc = "padc_gec_cfg5 (r) register accessor: PADC Gain Error Correction Config 5\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg5`] module"]
#[doc(alias = "padc_gec_cfg5")]
pub type PadcGecCfg5 = crate::Reg<padc_gec_cfg5::PadcGecCfg5Spec>;
#[doc = "PADC Gain Error Correction Config 5"]
pub mod padc_gec_cfg5;
#[doc = "padc_gec_cfg6 (r) register accessor: PADC Gain Error Correction Config 6\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg6`] module"]
#[doc(alias = "padc_gec_cfg6")]
pub type PadcGecCfg6 = crate::Reg<padc_gec_cfg6::PadcGecCfg6Spec>;
#[doc = "PADC Gain Error Correction Config 6"]
pub mod padc_gec_cfg6;
#[doc = "padc_gec_cfg7 (r) register accessor: PADC Gain Error Correction Config 7\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_gec_cfg7`] module"]
#[doc(alias = "padc_gec_cfg7")]
pub type PadcGecCfg7 = crate::Reg<padc_gec_cfg7::PadcGecCfg7Spec>;
#[doc = "PADC Gain Error Correction Config 7"]
pub mod padc_gec_cfg7;
#[doc = "padc_rx_fifo_ctrl (rw) register accessor: Passthrough ADC RX FIFO Control: DMA and interrupt triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_rx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_rx_fifo_ctrl`] module"]
#[doc(alias = "padc_rx_fifo_ctrl")]
pub type PadcRxFifoCtrl = crate::Reg<padc_rx_fifo_ctrl::PadcRxFifoCtrlSpec>;
#[doc = "Passthrough ADC RX FIFO Control: DMA and interrupt triggers"]
pub mod padc_rx_fifo_ctrl;
#[doc = "padc_rx_fifo_status (r) register accessor: Passthrough ADC RX FIFO Status: Occupancy and error status\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_rx_fifo_status`] module"]
#[doc(alias = "padc_rx_fifo_status")]
pub type PadcRxFifoStatus = crate::Reg<padc_rx_fifo_status::PadcRxFifoStatusSpec>;
#[doc = "Passthrough ADC RX FIFO Status: Occupancy and error status"]
pub mod padc_rx_fifo_status;
#[doc = "padc_rx_fifo_data (r) register accessor: Passthrough ADC RX FIFO Data Port: Read portal for conversion streams\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padc_rx_fifo_data`] module"]
#[doc(alias = "padc_rx_fifo_data")]
pub type PadcRxFifoData = crate::Reg<padc_rx_fifo_data::PadcRxFifoDataSpec>;
#[doc = "Passthrough ADC RX FIFO Data Port: Read portal for conversion streams"]
pub mod padc_rx_fifo_data;
