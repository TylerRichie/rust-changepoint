error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    // Without the `Result` wrapper:
    //
    // types {
    //     Error, ErrorKind, ResultExt;
    // }

    // Automatic conversions between this error chain and other
    // error chains. In this case, it will e.g. generate an
    // `ErrorKind` variant called `Another` which in turn contains
    // the `other_error::ErrorKind`, with conversions from
    // `other_error::Error`.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    links {
        // Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in the first case, the
    // `ErrorKind::Fmt` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    foreign_links {
        // Fmt(::std::fmt::Error);
        // Io(::std::io::Error) #[cfg(unix)];
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
        NaNOrInfiniteFloat(v: String) {
            description("Floating point value passed was either infinite or NaN")
            display("{} is not a finite floating point number", v)
        }
        NotEnoughValues(collection_len: usize, delta: usize) {
            description("Collection is too small -- it must have a length at least twice the value of delta")
            display(
                "The collection has {} elements, but it needs to have at least {} elements to be used with the given value of {} for delta",
                collection_len,
                delta * 2,
                delta)
        }
        // InvalidToolchainName(t: String) {
        //     description("invalid toolchain name")
        //     display("invalid toolchain name: '{}'", t)
        // }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        // UnknownToolchainVersion(v: String) {
        //     description("unknown toolchain version"), // note the ,
        //     display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        // }
    }
}
