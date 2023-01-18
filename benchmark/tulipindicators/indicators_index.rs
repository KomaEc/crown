
extern "C" {
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stoch_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_abs_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_abs(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_acos_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_acos(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ad_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ad(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_add_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_add(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adosc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adosc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adx_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adx(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adxr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_adxr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ao_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ao(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_apo_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_apo(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_aroon_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_aroon(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_aroonosc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_aroonosc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_asin_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_asin(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_atan_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_atan(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_atr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_atr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_avgprice_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_avgprice(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_bbands_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_bbands(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_bop_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_bop(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cci_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cci(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ceil_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ceil(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cmo_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cmo(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cos_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cos(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cosh_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cosh(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_crossany_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_crossany(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_crossover_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_crossover(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                    options: *const std::os::raw::c_double,
                    outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cvi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_cvi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_decay_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_decay(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dema_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dema(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_di_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_di(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_div_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_div(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stderr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stderr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stddev(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stddev_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sqrt(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sqrt_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sma(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sma_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sinh(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sinh_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sin(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sin_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_rsi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_rsi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_round(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_round_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_rocr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_rocr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_roc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_roc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_qstick(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_qstick_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_pvi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_pvi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_psar(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_psar_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ppo(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ppo_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_obv(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_obv_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_nvi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_nvi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_natr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_natr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mul(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mul_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_msw(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_msw_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mom(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mom_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_min(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_min_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mfi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mfi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_medprice(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_medprice_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_md(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_md_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_max(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_max_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mass(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_mass_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_marketfi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_marketfi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_macd(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_macd_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_log10(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_log10_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ln(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ln_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linregslope(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                      options: *const std::os::raw::c_double,
                      outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linregslope_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linregintercept(size: std::os::raw::c_int,
                          inputs: *const *const std::os::raw::c_double,
                          options: *const std::os::raw::c_double,
                          outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linregintercept_start(options: *const std::os::raw::c_double)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linreg(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_linreg_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_lag(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_lag_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_kvo(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_kvo_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_kama(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_kama_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_hma(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_hma_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_fosc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_fosc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_floor(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_floor_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_fisher(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_fisher_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_exp(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_exp_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_emv(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_emv_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ema(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ema_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_edecay(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_edecay_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dx(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dx_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dpo(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dpo_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dm(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_dm_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trix(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stochrsi_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stochrsi(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sub_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sub(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sum_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sum(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tan_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tan(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tanh_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tanh(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tema_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tema(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_todeg_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_todeg(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_torad_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_torad(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
             options: *const std::os::raw::c_double,
             outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trima_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trima(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trix_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_stoch(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vwma_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trunc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tsf_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_tsf(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_typprice_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_typprice(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                   options: *const std::os::raw::c_double,
                   outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ultosc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_ultosc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                 options: *const std::os::raw::c_double,
                 outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_var_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_var(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vhf_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vhf(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vidya_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vosc(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_volatility_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_volatility(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                     options: *const std::os::raw::c_double,
                     outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vosc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vidya(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_trunc_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_vwma(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
               options: *const std::os::raw::c_double,
               outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wad_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wad(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wcprice_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wcprice(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                  options: *const std::os::raw::c_double,
                  outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wilders_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wilders(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                  options: *const std::os::raw::c_double,
                  outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_willr_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_willr(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wma_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_wma(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
              options: *const std::os::raw::c_double,
              outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_zlema_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_zlema(size: std::os::raw::c_int, inputs: *const *const std::os::raw::c_double,
                options: *const std::os::raw::c_double,
                outputs: *const *mut std::os::raw::c_double) -> std::os::raw::c_int;
}
pub type ti_indicator_start_function
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_double) -> std::os::raw::c_int>;
pub type ti_indicator_function
    =
    Option<unsafe extern "C" fn(_: std::os::raw::c_int,
                                _: *const *const std::os::raw::c_double,
                                _: *const std::os::raw::c_double,
                                _: *const *mut std::os::raw::c_double)
               -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *mut std::os::raw::c_char,
    pub full_name: *mut std::os::raw::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub type_0: std::os::raw::c_int,
    pub inputs: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub input_names: [*mut std::os::raw::c_char; 10],
    pub option_names: [*mut std::os::raw::c_char; 10],
    pub output_names: [*mut std::os::raw::c_char; 10],
}
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2018 Tulip Charts LLC
 * Lewis Van Winkle (LV@tulipcharts.org)
 *
 * This file is part of Tulip Indicators.
 *
 * Tulip Indicators is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * Tulip Indicators is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Tulip Indicators.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
/*
 *
 * Version 0.8.4
 * Header Build 1537377628
 *
 */
/*
 *
 * This file is generated. Do not modify directly.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn ti_version() -> *const std::os::raw::c_char {
    return b"0.8.4\x00" as *const u8 as *const std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ti_build() -> std::os::raw::c_long {
    return 1537377628 as std::os::raw::c_int as std::os::raw::c_long;
}
#[no_mangle]
pub static mut ti_indicators: [ti_indicator_info; 105] =
    unsafe {
        [{
             let mut init =
                 ti_indicator_info{name:
                                       b"abs\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Absolute Value\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_abs_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_abs as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"abs\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"acos\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Arccosine\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_acos_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_acos as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"acos\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ad\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Accumulation/Distribution Line\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ad_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ad as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ad\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"add\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Addition\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_add_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_add as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"add\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"adosc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Accumulation/Distribution Oscillator\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_adosc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_adosc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"adosc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"adx\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Average Directional Movement Index\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_adx_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_adx as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"dx\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"adxr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Average Directional Movement Rating\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_adxr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_adxr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"dx\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ao\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Awesome Oscillator\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ao_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ao as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ao\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"apo\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Absolute Price Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_apo_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_apo as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"apo\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"aroon\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Aroon\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_aroon_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_aroon as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"aroon_down\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"aroon_up\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"aroonosc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Aroon Oscillator\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_aroonosc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_aroonosc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"aroonosc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"asin\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Arcsine\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_asin_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_asin as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"asin\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"atan\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Arctangent\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_atan_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_atan as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"atan\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"atr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Average True Range\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_atr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_atr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"atr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"avgprice\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Average Price\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_avgprice_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_avgprice as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"open\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"avgprice\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"bbands\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Bollinger Bands\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_bbands_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_bbands as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 3 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"stddev\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"bbands_lower\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"bbands_middle\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"bbands_upper\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"bop\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Balance of Power\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_bop_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_bop as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"open\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"bop\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"cci\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Commodity Channel Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_cci_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_cci as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"cci\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ceil\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Ceiling\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ceil_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ceil as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ceil\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"cmo\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Chande Momentum Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_cmo_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_cmo as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"cmo\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"cos\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Cosine\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_cos_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_cos as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"cos\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"cosh\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Hyperbolic Cosine\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_cosh_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_cosh as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"cosh\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"crossany\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Crossany\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_crossany_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_crossany as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"crossany\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"crossover\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Crossover\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_crossover_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_crossover as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"crossover\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"cvi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Chaikins Volatility\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_cvi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_cvi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"cvi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"decay\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Linear Decay\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_decay_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_decay as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"decay\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"dema\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Double Exponential Moving Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_dema_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_dema as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"dema\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"di\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Directional Indicator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_di_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_di as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"plus_di\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"minus_di\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"div\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Division\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_div_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_div as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"div\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"dm\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Directional Movement\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_dm_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_dm as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"plus_dm\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"minus_dm\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"dpo\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Detrended Price Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_dpo_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_dpo as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"dpo\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"dx\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Directional Movement Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_dx_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_dx as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"dx\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"edecay\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Exponential Decay\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_edecay_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_edecay as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"edecay\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ema\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Exponential Moving Average\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ema_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ema as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ema\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"emv\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Ease of Movement\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_emv_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_emv as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"emv\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"exp\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Exponential\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_exp_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_exp as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"exp\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"fisher\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Fisher Transform\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_fisher_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_fisher as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"fisher\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"fisher_signal\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"floor\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Floor\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_floor_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_floor as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"floor\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"fosc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Forecast Oscillator\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_fosc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_fosc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"fosc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"hma\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Hull Moving Average\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_hma_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_hma as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"hma\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"kama\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Kaufman Adaptive Moving Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_kama_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_kama as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"kama\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"kvo\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Klinger Volume Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_kvo_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_kvo as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"kvo\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"lag\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Lag\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_lag_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_lag as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"lag\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"linreg\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Linear Regression\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_linreg_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_linreg as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"linreg\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"linregintercept\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Linear Regression Intercept\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_linregintercept_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_linregintercept as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"linregintercept\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"linregslope\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Linear Regression Slope\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_linregslope_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_linregslope as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"linregslope\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ln\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Natural Log\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ln_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ln as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ln\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"log10\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Base-10 Log\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_log10_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_log10 as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"log10\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"macd\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Moving Average Convergence/Divergence\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_macd_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_macd as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 3 as std::os::raw::c_int,
                                   outputs: 3 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"signal period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"macd\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"macd_signal\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"macd_histogram\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"marketfi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Market Facilitation Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_marketfi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_marketfi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"marketfi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"mass\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Mass Index\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_mass_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_mass as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"mass\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"max\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Maximum In Period\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_max_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_max as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"max\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"md\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Mean Deviation Over Period\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_md_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_md as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"md\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"medprice\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Median Price\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_medprice_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_medprice as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"medprice\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"mfi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Money Flow Index\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_mfi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_mfi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 4 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"mfi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"min\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Minimum In Period\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_min_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_min as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"min\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"mom\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Momentum\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_mom_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_mom as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"mom\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"msw\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Mesa Sine Wave\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_msw_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_msw as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"msw_sine\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"msw_lead\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"mul\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Multiplication\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_mul_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_mul as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"mul\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"natr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Normalized Average True Range\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_natr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_natr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"natr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"nvi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Negative Volume Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_nvi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_nvi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"nvi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"obv\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"On Balance Volume\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_obv_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_obv as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"obv\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ppo\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Percentage Price Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ppo_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ppo as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ppo\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"psar\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Parabolic SAR\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_psar_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_psar as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"acceleration factor step\x00" as
                                            *const u8 as *const std::os::raw::c_char
                                            as *mut std::os::raw::c_char,
                                        b"acceleration factor maximum\x00" as
                                            *const u8 as *const std::os::raw::c_char
                                            as *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"psar\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"pvi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Positive Volume Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_pvi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_pvi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"pvi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"qstick\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Qstick\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_qstick_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_qstick as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"open\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"qstick\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"roc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Rate of Change\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_roc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_roc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"roc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"rocr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Rate of Change Ratio\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_rocr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_rocr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"rocr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"round\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Round\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_round_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_round as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"round\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"rsi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Relative Strength Index\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_rsi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_rsi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"rsi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sin\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Sine\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sin_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sin as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sin\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sinh\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Hyperbolic Sine\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sinh_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sinh as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sinh\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sma\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Simple Moving Average\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sma_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sma as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sma\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sqrt\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Square Root\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sqrt_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sqrt as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sqrt\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"stddev\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Standard Deviation Over Period\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_stddev_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_stddev as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"stddev\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"stderr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Standard Error Over Period\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_stderr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_stderr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"stderr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"stoch\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Stochastic Oscillator\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_stoch_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_stoch as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 3 as std::os::raw::c_int,
                                   outputs: 2 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"%k period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"%k slowing period\x00" as *const u8
                                            as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"%d period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"stoch_k\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"stoch_d\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"stochrsi\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Stochastic RSI\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_stochrsi_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_stochrsi as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"stochrsi\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sub\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Subtraction\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sub_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sub as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sub\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"sum\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Sum Over Period\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_sum_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_sum as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"sum\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"tan\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Tangent\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_tan_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_tan as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"tan\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"tanh\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Hyperbolic Tangent\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_tanh_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_tanh as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"tanh\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"tema\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Triple Exponential Moving Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_tema_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_tema as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"tema\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"todeg\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Degree Conversion\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_todeg_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_todeg as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"degrees\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"torad\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Radian Conversion\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_torad_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_torad as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"radians\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"tr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"True Range\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_tr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_tr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"tr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"trima\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Triangular Moving Average\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_trima_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_trima as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"trima\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"trix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Trix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_trix_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_trix as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"trix\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"trunc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vector Truncate\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_trunc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_trunc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 4 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"trunc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"tsf\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Time Series Forecast\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_tsf_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_tsf as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"tsf\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"typprice\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Typical Price\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_typprice_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_typprice as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"typprice\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"ultosc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Ultimate Oscillator\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_ultosc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_ultosc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 3 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"medium period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"ultosc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"var\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Variance Over Period\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_var_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_var as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 3 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"var\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"vhf\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Vertical Horizontal Filter\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_vhf_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_vhf as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"vhf\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"vidya\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Variable Index Dynamic Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_vidya_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_vidya as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 3 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"alpha\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"vidya\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"volatility\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Annualized Historical Volatility\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_volatility_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_volatility as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"volatility\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"vosc\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Volume Oscillator\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_vosc_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_vosc as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 2 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"short period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"long period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"vosc\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"vwma\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Volume Weighted Moving Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_vwma_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_vwma as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 2 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"volume\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"vwma\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"wad\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Williams Accumulation/Distribution\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_wad_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_wad as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"wad\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"wcprice\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Weighted Close Price\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_wcprice_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_wcprice as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"wcprice\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"wilders\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Wilders Smoothing\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_wilders_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_wilders as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"wilders\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"willr\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Williams %R\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_willr_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_willr as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 2 as std::os::raw::c_int,
                                   inputs: 3 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"high\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"low\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        b"close\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"willr\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"wma\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Weighted Moving Average\x00" as
                                           *const u8 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_wma_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_wma as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"wma\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       b"zlema\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       b"Zero-Lag Exponential Moving Average\x00"
                                           as *const u8 as *const std::os::raw::c_char
                                           as *mut std::os::raw::c_char,
                                   start:
                                       Some(ti_zlema_start as
                                                unsafe extern "C" fn(_:
                                                                         *const std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   indicator:
                                       Some(ti_zlema as
                                                unsafe extern "C" fn(_:
                                                                         std::os::raw::c_int,
                                                                     _:
                                                                         *const *const std::os::raw::c_double,
                                                                     _:
                                                                         *const std::os::raw::c_double,
                                                                     _:
                                                                         *const *mut std::os::raw::c_double)
                                                    -> std::os::raw::c_int),
                                   type_0: 1 as std::os::raw::c_int,
                                   inputs: 1 as std::os::raw::c_int,
                                   options: 1 as std::os::raw::c_int,
                                   outputs: 1 as std::os::raw::c_int,
                                   input_names:
                                       [b"real\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [b"period\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [b"zlema\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         },
         {
             let mut init =
                 ti_indicator_info{name:
                                       0 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   full_name:
                                       0 as *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char,
                                   start: None,
                                   indicator: None,
                                   type_0: 0 as std::os::raw::c_int,
                                   inputs: 0 as std::os::raw::c_int,
                                   options: 0 as std::os::raw::c_int,
                                   outputs: 0 as std::os::raw::c_int,
                                   input_names:
                                       [0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   option_names:
                                       [0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],
                                   output_names:
                                       [0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char,
                                        0 as *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char],};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn ti_find_indicator(mut name: *const std::os::raw::c_char)
 -> *const ti_indicator_info {
    let mut imin: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut imax: std::os::raw::c_int =
        (::std::mem::size_of::<[ti_indicator_info; 105]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<ti_indicator_info>()
                                             as
                                             std::os::raw::c_ulong).wrapping_sub(2 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong)
            as std::os::raw::c_int;
    /*Binary search.*/
    while imax >= imin {
        let i: std::os::raw::c_int = imin + (imax - imin) / 2 as std::os::raw::c_int;
        let c: std::os::raw::c_int = strcmp(name, ti_indicators[i as usize].name);
        if c == 0 as std::os::raw::c_int {
            return ti_indicators.as_mut_ptr().offset(i as isize)
        } else {
            if c > 0 as std::os::raw::c_int {
                imin = i + 1 as std::os::raw::c_int
            } else { imax = i - 1 as std::os::raw::c_int }
        }
    }
    return 0 as *const ti_indicator_info;
}
