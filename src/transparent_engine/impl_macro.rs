macro_rules! transparent_engine_impl {
    (
        $engine:ident,
        $fr:ty
    ) => {
        #[derive(Clone)]
        pub struct $engine;

        impl pairing::ff::ScalarEngine for $engine {
            type Fr = $fr;
        }

        impl pairing::Engine for $engine {
            type G1 = $fr;
            type G1Affine = $fr;
            type G2 = $fr;
            type G2Affine = $fr;
            type Fq = $fr;
            type Fqe = $fr;
            type Fqk = $fr;

            fn miller_loop<'a, I>(_i: I) -> Self::Fqk
                where I: IntoIterator<Item=&'a (
                                            &'a <Self::G1Affine as pairing::CurveAffine>::Prepared,
                                            &'a <Self::G2Affine as pairing::CurveAffine>::Prepared
                                    )>
            {
                <$fr as pairing::ff::Field>::zero()
            }

            /// Perform final exponentiation of the result of a miller loop.
            fn final_exponentiation(this: &Self::Fqk) -> Option<Self::Fqk>
            {
                Some(*this)
            }
        }


        impl pairing::CurveProjective for $fr {
            type Affine = $fr;
            type Base = $fr;
            type Scalar = $fr;
            type Engine = $engine;

            fn zero() -> Self {
                <$fr as pairing::ff::Field>::zero()
            }

            fn one() -> Self {
                <$fr as pairing::ff::Field>::one()
            }

            fn is_zero(self: &Self) -> bool {
                <$fr as pairing::ff::Field>::is_zero(self)
            }

            fn batch_normalization(_: &mut [Self]) {
                
            }

            fn is_normalized(&self) -> bool {
                true
            }

            fn double(self: &mut Self) {
                <$fr as pairing::ff::Field>::double(self);
            }

            fn add_assign(self: &mut Self, other: &Self) {
                <$fr as pairing::ff::Field>::add_assign(self, other);
            }

            fn add_assign_mixed(&mut self, other: &Self) {
                <$fr as pairing::ff::Field>::add_assign(self, other);
            }

            fn negate(self: &mut Self) {
                <$fr as pairing::ff::Field>::negate(self);
            }

            fn mul_assign<S: Into<<Self::Scalar as pairing::ff::PrimeField>::Repr>>(self: &mut Self, other: S)
            {
                let tmp = <$fr as pairing::ff::PrimeField>::from_repr(other.into()).unwrap();

                <$fr as pairing::ff::Field>::mul_assign(self, &tmp);
            }

            fn into_affine(&self) -> $fr {
                *self
            }

            fn recommended_wnaf_for_scalar(_: <Self::Scalar as pairing::ff::PrimeField>::Repr) -> usize {
                3
            }

            fn recommended_wnaf_for_num_scalars(_: usize) -> usize {
                3
            }
        }

        #[derive(Copy, Clone)]
        pub struct FakePoint;

        impl AsMut<[u8]> for FakePoint {
            fn as_mut(&mut self) -> &mut [u8] {
                unimplemented!()
            }
        }

        impl AsRef<[u8]> for FakePoint {
            fn as_ref(&self) -> &[u8] {
                unimplemented!()
            }
        }

        impl pairing::EncodedPoint for FakePoint {
            type Affine = $fr;

            fn empty() -> Self {
                unimplemented!()
            }

            fn size() -> usize {
                unimplemented!()
            }

            fn into_affine(&self) -> Result<Self::Affine, pairing::GroupDecodingError> {
                unimplemented!()
            }

            fn into_affine_unchecked(&self) -> Result<Self::Affine, pairing::GroupDecodingError> {
                unimplemented!()
            }

            fn from_affine(_: Self::Affine) -> Self {
                unimplemented!()
            }
        }

        impl pairing::CurveAffine for $fr {
            type Pair = $fr;
            type PairingResult = $fr;
            type Compressed = FakePoint;
            type Uncompressed = FakePoint;
            type Prepared = $fr;
            type Projective = $fr;
            type Base = $fr;
            type Scalar = $fr;
            type Engine = $engine;

            fn zero() -> Self {
                <$fr as pairing::ff::Field>::zero()
            }

            fn one() -> Self {
                <$fr as pairing::ff::Field>::one()
            }

            fn is_zero(&self) -> bool {
                <$fr as pairing::ff::Field>::is_zero(self)
            }

            fn negate(&mut self) {
                <$fr as pairing::ff::Field>::negate(self);
            }

            fn mul<S: Into<<Self::Scalar as pairing::ff::PrimeField>::Repr>>(&self, other: S) -> Self::Projective
            {
                let mut res = *self;
                let tmp = <$fr as pairing::ff::PrimeField>::from_repr(other.into()).unwrap();

                <$fr as pairing::ff::Field>::mul_assign(&mut res, &tmp);

                res
            }

            fn prepare(&self) -> Self::Prepared {
                *self
            }

            fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
                self.mul(*other)
            }

            fn into_projective(&self) -> Self::Projective {
                *self
            }

            fn as_xy(&self) -> (&Self::Base, &Self::Base) {
                (&self, &self)
            }

            fn into_xy_unchecked(&self) -> (Self::Base, Self::Base) {
                (<$fr as pairing::ff::Field>::zero(), <$fr as pairing::ff::Field>::zero())
            }

            fn from_xy_unchecked(_x: Self::Base, _y: Self::Base) -> Self {
                <$fr as pairing::ff::Field>::zero()
            }

            fn from_xy_checked(_x: Self::Base, _y: Self::Base) -> Result<Self, pairing::GroupDecodingError> {
                Ok(<$fr as pairing::ff::Field>::zero())
            }
        }

        impl pairing::RawEncodable for $fr {
            fn into_raw_uncompressed_le(&self) -> Self::Uncompressed {
                use pairing::EncodedPoint;
                Self::Uncompressed::empty()
            }

            fn from_raw_uncompressed_le_unchecked(
                    _encoded: &Self::Uncompressed, 
                    _infinity: bool
            ) -> Result<Self, pairing::GroupDecodingError> {
                Ok(<Self as pairing::ff::Field>::zero())
            }

            fn from_raw_uncompressed_le(encoded: &Self::Uncompressed, _infinity: bool) -> Result<Self, pairing::GroupDecodingError> {
                Self::from_raw_uncompressed_le_unchecked(&encoded, _infinity)
            }
        }
    }
}