use crate::{Clipper, ClipperError, FillRule, Paths, PointScaler};

/// This function intersects closed subject paths with clip paths.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path_a: Paths = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)].into();
/// let path_b: Paths = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)].into();
///
/// let output: Vec<Vec<(f64, f64)>> = intersect(path_a, path_b, FillRule::default())
///     .expect("Failed to run boolean operation").into();
///
/// dbg!(output);
/// ```
/// ![Image displaying the result of the intersect example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/intersect.png)
///
/// For more details see the original [intersect](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Intersect.htm) docs.
pub fn intersect<P: PointScaler>(
    subject: impl Into<Paths<P>>,
    clip: impl Into<Paths<P>>,
    fill_rule: FillRule,
) -> Result<Paths<P>, ClipperError> {
    Clipper::new()
        .add_subject(subject)
        .add_clip(clip)
        .intersect(fill_rule)
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_intersect() {
        let path1 = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let path2 = vec![(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)];
        let expected_output = vec![vec![(1.0, 1.0), (0.5, 1.0), (0.5, 0.5), (1.0, 0.5)]];

        let output: Vec<Vec<(f64, f64)>> = intersect::<Centi>(path1, path2, FillRule::default())
            .unwrap()
            .into();
        assert_eq!(output, expected_output);
    }
}
