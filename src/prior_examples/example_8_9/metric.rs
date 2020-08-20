use std::ops::{Mul, Sub};
use crate::example_8_9::Error;
use std::fmt::{Debug};

// note: Metrics vs. Distances

trait Metric {
    fn is_single_real(&self) -> bool;
    fn has_upper_bound(&self) -> bool;
    fn is_triangular(&self) -> bool;
    fn has_path_connectivity(&self) -> bool;
    fn is_symmetric(&self) -> bool;
}

#[derive(PartialEq, Clone)]
pub enum DataMetric {
    DistFloat(DistFloat),
    AddRemove(AddRemove),
    And(AndMetric),
}

#[derive(PartialEq, Clone, Debug)]
pub enum PrivacyMeasure {
    PureDP(PureDP),
    ApproxDP(ApproxDP),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DistFloat;

#[derive(Clone, Debug, PartialEq)]
pub struct AddRemove;

#[derive(PartialEq)]
pub struct AndMetric(Box<DataMetric>, Box<DataMetric>);
impl Clone for AndMetric {
    fn clone(&self) -> Self {
        Self(Box::new(*self.0.clone()), Box::new(*self.1.clone()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PureDP;

#[derive(Clone, Debug, PartialEq)]
pub struct ApproxDP;

impl Metric for DistFloat {
    fn is_single_real(&self) -> bool {
        true
    }

    fn has_upper_bound(&self) -> bool {
        unimplemented!()
    }

    fn is_triangular(&self) -> bool {
        unimplemented!()
    }

    fn has_path_connectivity(&self) -> bool {
        unimplemented!()
    }

    fn is_symmetric(&self) -> bool {
        unimplemented!()
    }
}


#[derive(Clone, PartialOrd, PartialEq)]
pub enum DataDistance {
    DistFloat(f64),
    AddRemove(u16),
    And(Box<DataDistance>, Box<DataDistance>),
}
// impl DataDistance {
//     fn get_metric(self) -> DataMetric {
//         match self {
//
//         }
//     }
// }

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PrivacyDistance {
    PureDP(f64),
    ApproxDP(f64, f64),
}

impl Mul<i64> for DataDistance {
    type Output = DataDistance;

    fn mul(self, rhs: i64) -> Self::Output {
        match self {
            DataDistance::DistFloat(d) => DataDistance::DistFloat(d * rhs as f64),
            _ => unimplemented!()
        }
    }
}

impl Sub<&PrivacyDistance> for &PrivacyDistance {
    type Output = Result<PrivacyDistance, Error>;

    fn sub(self, rhs: &PrivacyDistance) -> Self::Output {
        Ok(match (self, rhs) {
            (PrivacyDistance::ApproxDP(eps_l, del_l), PrivacyDistance::ApproxDP(eps_r, del_r)) =>
                PrivacyDistance::ApproxDP(eps_l + eps_r, del_l + del_r),
            (PrivacyDistance::PureDP(eps_l), PrivacyDistance::PureDP(eps_r)) =>
                PrivacyDistance::PureDP(eps_l + eps_r),
            _ => return Err("privacy units must match")
        })
    }
}