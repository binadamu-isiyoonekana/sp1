use p3_field::AbstractField;
use sp1_recursion_compiler::{asm::AsmBuilder, prelude::*};
use sp1_recursion_core::runtime::Runtime;
use sp1_stark::{baby_bear_poseidon2::BabyBearPoseidon2, StarkGenericConfig};

#[test]
fn test_compiler_less_than() {
    type SC = BabyBearPoseidon2;
    type F = <SC as StarkGenericConfig>::Val;
    type EF = <SC as StarkGenericConfig>::Challenge;
    let mut builder = AsmBuilder::<F, EF>::default();

    let a: Var<_> = builder.constant(F::from_canonical_u32(10));
    let b: Var<_> = builder.constant(F::from_canonical_u32(20));
    let c = builder.lt(a, b);
    builder.assert_var_eq(c, F::one());

    let a: Var<_> = builder.constant(F::from_canonical_u32(20));
    let b: Var<_> = builder.constant(F::from_canonical_u32(10));
    let c = builder.lt(a, b);
    builder.assert_var_eq(c, F::zero());

    let program = builder.compile_program();

    let config = SC::default();
    let mut runtime = Runtime::<F, EF, _>::new(&program, config.perm.clone());
    runtime.run().unwrap();
}
