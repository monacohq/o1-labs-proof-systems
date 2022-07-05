use ark_ec::{short_weierstrass_jacobian::GroupAffine, ModelParameters};
use commitment_dlog::{commitment::CommitmentCurve, srs::endos};
use mina_curves::pasta::{pallas::PallasParameters, vesta::VestaParameters};
use once_cell::sync::Lazy;
use oracle::poseidon::ArithmeticSpongeParams;

///Representing additional information that a curve needs to be used with Kimchi
pub trait KimchiCurve: CommitmentCurve {
    type OtherCurve: KimchiCurve;

    fn sponge_params() -> &'static ArithmeticSpongeParams<Self::ScalarField>;

    fn endos() -> &'static (Self::BaseField, Self::ScalarField);
}

impl KimchiCurve for GroupAffine<VestaParameters> {
    type OtherCurve = GroupAffine<PallasParameters>;

    fn sponge_params() -> &'static ArithmeticSpongeParams<Self::ScalarField> {
        oracle::pasta::fp_kimchi::static_params()
    }

    fn endos() -> &'static (Self::BaseField, Self::ScalarField) {
        static VESTA_ENDOS: Lazy<(
            <VestaParameters as ModelParameters>::BaseField,
            <VestaParameters as ModelParameters>::ScalarField,
        )> = Lazy::new(endos::<GroupAffine<VestaParameters>>);
        &VESTA_ENDOS
    }
}

impl KimchiCurve for GroupAffine<PallasParameters> {
    type OtherCurve = GroupAffine<VestaParameters>;

    fn sponge_params() -> &'static ArithmeticSpongeParams<Self::ScalarField> {
        oracle::pasta::fq_kimchi::static_params()
    }

    fn endos() -> &'static (Self::BaseField, Self::ScalarField) {
        static PALLAS_ENDOS: Lazy<(
            <PallasParameters as ModelParameters>::BaseField,
            <PallasParameters as ModelParameters>::ScalarField,
        )> = Lazy::new(endos::<GroupAffine<PallasParameters>>);
        &PALLAS_ENDOS
    }
}
