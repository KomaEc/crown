use std::ops::Range;

use crate::access_path::{EncodedProjections, KLimited};

impl KLimited<EncodedProjections> {
    /// This matcher only allows matching betweens projections with the same
    /// k_limit. We can develop a (slightly more sophisticated) algorithm to
    /// enable efficient matching between projections with different k_limits,
    /// but it is not quite necessay. We can always encode places with specific
    /// k_limits so that the two access paths to be matched have the same k_limit. 
    pub fn r#match(self, other: Self) -> impl Iterator<Item = (usize, usize)> {
        assert_eq!(self.k_limit, other.k_limit);
        assert_eq!(self.data.size(), other.data.size());
        Range::from(self.data).zip(Range::from(other.data))
    }
}

