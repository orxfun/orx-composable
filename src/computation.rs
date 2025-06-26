/// A generic computation which might or might not use closed over data.
///
/// It might be considered as a closure with a single input parameter.
/// Note that multiple argument functions will be handled by tuples.
///
/// # Examples
///
/// The following example demonstrates an average age computation with
/// a single argument that is a collection of people.
///
/// ```
/// use orx_composable::*;
///
/// struct Person {
///     age: u32,
/// }
///
/// struct AverageAge;
///
/// impl Computation for AverageAge {
///     type In<'i> = &'i [Person];
///
///     type Out = u32;
///
///     fn compute(&self, people: Self::In<'_>) -> Self::Out {
///         match people.len() {
///             0 => 0,
///             n => people.iter().map(|p| p.age).sum::<u32>() / n as u32,
///         }
///     }
/// }
///
/// let average_age_computer = AverageAge;
///
/// let people = [1, 26, 42, 99].map(|age| Person { age });
/// assert_eq!(average_age_computer.compute(&people), 42);
///
/// let people = [].map(|age| Person { age });
/// assert_eq!(average_age_computer.compute(&people), 0);
/// ```
///
/// As demonstrated below, the computation can capture and use internal immutable
/// data, `age_threshold` in this case.
///
/// ```
/// use orx_composable::*;
///
/// struct Person {
///     age: u32,
/// }
///
/// struct AverageAgeOfAdults {
///     age_threshold: u32,
/// }
///
/// impl Computation for AverageAgeOfAdults {
///     type In<'i> = &'i [Person];
///
///     type Out = u32;
///
///     fn compute(&self, people: Self::In<'_>) -> Self::Out {
///         let (count, sum) = people.iter().fold((0, 0), |(count, sum), person| {
///             match person.age > self.age_threshold {
///                 true => (count + 1, sum + person.age),
///                 false => (count, sum),
///             }
///         });
///         match count {
///             0 => 0,
///             n => sum / n as u32,
///         }
///     }
/// }
///
/// let average_age_computer = AverageAgeOfAdults { age_threshold: 30 };
///
/// let people = [1, 26, 42, 99].map(|age| Person { age });
/// assert_eq!(average_age_computer.compute(&people), 70);
///
/// let people = [].map(|age| Person { age });
/// assert_eq!(average_age_computer.compute(&people), 0);
/// ```
///
/// Finally, the following example defines `Append` as a computation
/// with two arguments a string slice and a character to append to it.
///
/// ```
/// use orx_composable::*;
///
/// struct Append;
///
/// impl Computation for Append {
///     type In<'i> = (&'i str, char);
///
///     type Out = String;
///
///     fn compute(&self, (str, c): Self::In<'_>) -> Self::Out {
///         let mut str = str.to_string();
///         str.push(c);
///         str
///     }
/// }
///
/// let append = Append;
/// assert_eq!(append.compute((&"4", '2')), 42.to_string());
/// ```
pub trait Computation {
    /// Input of the computation.
    type In<'i>;

    /// Output of the computation.
    type Out;

    /// Computes output for the given `input`.
    ///
    /// See [`Computation`] for examples.
    fn compute(&self, input: Self::In<'_>) -> Self::Out;
}
