use crate::{Transformation, Measurement};
use crate::metric::{DataDistance, PrivacyDistance};
use crate::base::Data;

pub fn make_tt_chain(
    trans_2: Transformation, trans: Transformation,
    hint: Option<Box<dyn Fn(&DataDistance, &DataDistance) -> DataDistance>>,
    // hint_hint: Option<Box<dyn Fn(&>>
) -> Result<Transformation, crate::Error> {
    if trans_2.input_domain != trans.output_domain {
        return Err(crate::Error::DomainMismatch)
    }

    let Transformation {
        input_domain: trans_input_domain,
        stability_relation: trans_stability_relation,
        function: trans_function,
        ..
    } = trans;

    let Transformation {
        output_domain: trans_2_output_domain,
        stability_relation: trans_2_stability_relation,
        function: trans_2_function,
        ..
    } = trans_2;

    Ok(Transformation {
        input_domain: trans_input_domain,
        output_domain: trans_2_output_domain,
        stability_relation: Box::new(move |d_in: &DataDistance, d_out: &DataDistance| {
            let d_mid = (hint.as_ref().unwrap())(d_in, d_out);
            (trans_2_stability_relation)(&d_mid, d_out) && (trans_stability_relation)(d_in, &d_mid)
        }),
        function: Box::new(move |data: Data| (trans_2_function)((trans_function)(data)?)),
        hint: None,
    })
}


pub fn make_mt_chain(
    meas: Measurement, trans: Transformation,
    hint: Option<Box<dyn Fn(&DataDistance, &PrivacyDistance) -> DataDistance>>,
    // hint_hint: Option<Box<dyn Fn(&>>
) -> Result<Measurement, crate::Error> {
    if meas.input_domain != trans.output_domain {
        return Err(crate::Error::DomainMismatch)
    }

    let Transformation {
        input_domain: trans_input_domain,
        stability_relation: trans_stability_relation,
        function: trans_function,
        ..
    } = trans;

    let Measurement {
        privacy_relation: meas_privacy_relation,
        function: meas_function,
        ..
    } = meas;

    Ok(Measurement {
        input_domain: trans_input_domain,
        privacy_relation: Box::new(move |d_in: &DataDistance, d_out: &PrivacyDistance| {
            let d_mid = (hint.as_ref().unwrap())(d_in, d_out);
            (meas_privacy_relation)(&d_mid, d_out) && (trans_stability_relation)(d_in, &d_mid)
        }),
        function: Box::new(move |data: Data| (meas_function)((trans_function)(data)?)),
    })
}
