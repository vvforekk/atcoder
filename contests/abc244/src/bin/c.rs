use std::{
    any::type_name,
    fmt::Debug,
    io::{self, BufRead, BufReader, Stdin},
    iter::Peekable,
    marker::PhantomData,
    str::{FromStr, SplitWhitespace},
    sync::Mutex,
};

use lazy_static::lazy_static;

/// Source reading stream line by line.
///
/// It is a wrapper for `BufRead`.  You can create `LineSource` from any type implementing
/// `BufRead`.
pub struct LineSource<R: BufRead> {
    // FIXME: This is actually not 'static but it is treated as 'static for the
    // same reason with crate::source::once::Source.  Also there is no way to
    // separate context and tokens since they are private field, this is safe.
    tokens: Peekable<SplitWhitespace<'static>>,

    // context `tokens` reffering to
    current_context: Box<str>,

    reader: R,
}

impl<R: BufRead> LineSource<R> {
    /// Creates a `LineSource` by specified `BufRead`.
    pub fn new(reader: R) -> LineSource<R> {
        // dummy values.
        LineSource {
            current_context: "".to_string().into_boxed_str(),
            tokens: "".split_whitespace().peekable(),
            reader,
        }
    }

    fn prepare(&mut self) {
        while self.tokens.peek().is_none() {
            let mut line = String::new();
            let num_bytes = self
                .reader
                .read_line(&mut line)
                .expect("failed to get linel maybe an IO error.");

            if num_bytes == 0 {
                // reached EOF
                return;
            }

            self.current_context = line.into_boxed_str();
            self.tokens = unsafe { std::mem::transmute::<_, &'static str>(&*self.current_context) }
                .split_whitespace()
                .peekable();
        }
    }
}

impl<R: BufRead> Source<R> for LineSource<R> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str> {
        // while tokens are empty, reads a new line.
        self.prepare();
        self.tokens.next()
    }

    /// Check if tokens are empty
    fn is_empty(&mut self) -> bool {
        self.prepare();
        self.tokens.peek().is_none()
    }
}

/// You can create `LineSource` from `&str`.  Since `&[u8]` is a `Read`, `BufRead` can be easily
/// created by wrapping using `BufReader`.
impl<'a> From<&'a str> for LineSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> LineSource<BufReader<&'a [u8]>> {
        LineSource::new(BufReader::new(s.as_bytes()))
    }
}

/// Source reading entire content for the first time.
///
/// It is a wrapper for `BufRead`.  You can create `OnceSource` from any type implementing
/// `BufRead`.
pub struct OnceSource<R: BufRead> {
    // Of course this is not 'static actually, but it is always valid reference
    // while entire `Source` is alive.  The actual lifetime is the context's
    // inner lifetime, and it is essentially the lifetime of self.  Also note
    // that there is no way to separate context and tokens since they are both
    // private field.
    //
    // FIXME: find nicer way.
    tokens: Peekable<SplitWhitespace<'static>>,

    // context `tokens` is reffering to
    context: Box<str>,

    // to consume `R`.  Actually `OnceSource` is not need to have `R`, since reading is done in its
    // constructor.  This is for the consistency with `LineSource` (To use smoothly through `AutoSource`).
    _read: PhantomData<R>,
}

impl<R: BufRead> OnceSource<R> {
    /// Creates `Source` using specified reader of `BufRead`.
    pub fn new(mut source: R) -> OnceSource<R> {
        let mut context = String::new();
        source
            .read_to_string(&mut context)
            .expect("failed to read from source; maybe an IO error.");

        // Boxed str is no need to check to pin.
        let context = context.into_boxed_str();

        // We can create tokens first.  But doing so causes "unused variable
        // `context`" warning (here `context` is Source::context, a member of
        // Source`). To avoid the warning at first tokens are dummy and replace
        // it using Source's context.
        let mut res = OnceSource {
            context,
            tokens: "".split_whitespace().peekable(),
            _read: PhantomData,
        };

        use std::mem;
        let context: &'static str = unsafe { mem::transmute(&*res.context) };
        res.tokens = context.split_whitespace().peekable();

        res
    }
}

impl<R: BufRead> Source<R> for OnceSource<R> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
    }

    /// Check if tokens are empty
    fn is_empty(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
}

/// You can create `OnceSource` from `&str`.  Since `&[u8]` is a `Read`, `BufRead` can be easily
/// created by wrapping using `BufReader`.
impl<'a> From<&'a str> for OnceSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> OnceSource<BufReader<&'a [u8]>> {
        OnceSource::new(BufReader::new(s.as_bytes()))
    }
}

pub enum AutoSource<R: BufRead> {
    Line(LineSource<R>),
    Once(OnceSource<R>),
}

impl<R: BufRead> AutoSource<R> {
    pub fn new(reader: R, interactive: bool) -> Self {
        if interactive {
            Self::Line(LineSource::new(reader))
        } else {
            Self::Once(OnceSource::new(reader))
        }
    }
}

impl<R: BufRead> Source<R> for AutoSource<R> {
    fn next_token(&mut self) -> Option<&str> {
        match self {
            AutoSource::Line(s) => s.next_token(),
            AutoSource::Once(s) => s.next_token(),
        }
    }

    fn is_empty(&mut self) -> bool {
        match self {
            AutoSource::Line(s) => s.is_empty(),
            AutoSource::Once(s) => s.is_empty(),
        }
    }
}

impl<'a> From<&'a str> for AutoSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> AutoSource<BufReader<&'a [u8]>> {
        AutoSource::new(BufReader::new(s.as_bytes()), false)
    }
}

/// The main trait. Types implementing this trait can be used for source of `input!` macro.
pub trait Source<R: BufRead> {
    /// Gets a whitespace-splitted next token.
    fn next_token(&mut self) -> Option<&str>;

    /// Check if tokens are empty
    #[allow(clippy::wrong_self_convention)]
    fn is_empty(&mut self) -> bool;

    /// Force gets a whitespace-splitted next token.
    fn next_token_unwrap(&mut self) -> &str {
        self.next_token().expect(concat!(
            "failed to get the next token; ",
            "maybe reader reached an end of input. ",
            "ensure that arguments for `input!` macro is correctly ",
            "specified to match the problem input."
        ))
    }
}

// &mut S where S: Source is also source.
impl<R: BufRead, S: Source<R>> Source<R> for &'_ mut S {
    fn next_token(&mut self) -> Option<&str> {
        (*self).next_token()
    }

    fn is_empty(&mut self) -> bool {
        (*self).is_empty()
    }
}

/// A trait representing which type can be read from `Source`.
///
/// If you want to read your own type using `input!`, you can implement this trait for your type.
/// Alternatively, you can add `#[derive_readable]` if you put `use
/// proconio_derive::derive_readable` in your source.  It automatically implements `Readable` if
/// all members of your type are `Readable`.
pub trait Readable {
    type Output;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output;
}

// implementations of Readable for any `FromStr` types including primitives.
impl<T: FromStr> Readable for T
where
    T::Err: Debug,
{
    type Output = T;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> T {
        let token = source.next_token_unwrap();
        match token.parse() {
            Ok(v) => v,
            Err(e) => panic!(
                concat!(
                    "failed to parse the input `{input}` ",
                    "to the value of type `{ty}`: {err:?}; ",
                    "ensure that the input format is collectly specified ",
                    "and that the input value must handle specified type.",
                ),
                input = token,
                ty = type_name::<T>(),
                err = e,
            ),
        }
    }
}

pub static mut __INTERACTIVE: bool = false;

lazy_static! {
    #[doc(hidden)]
    pub static ref STDIN_SOURCE: Mutex<AutoSource<BufReader<Stdin>>> =
    if cfg!(debug_assertion) {
        Mutex::new(AutoSource::Line(LineSource::new(BufReader::new(io::stdin()))))
    } else {
        Mutex::new(AutoSource::new(BufReader::new(io::stdin()), unsafe {__INTERACTIVE}))
    };
}

/// read input from stdin.
///
/// basic syntax is:
/// ```text
/// input! {
///     from source,          // optional: if you omitted, stdin is used by default.
///     (mut) variable: type, // mut is optional: mut makes the variable mutable.
///     ...
/// }
/// ```
/// the trailing comma is optional.  `source` can be anything implementing `Source`.  This macro
/// moves out the specified source.  If you want to prevent moving, you can use `&mut source` since
/// `&mut S` where `S: Source` also implements `Source`.
#[macro_export]
macro_rules! input {
    // terminator
    (@from [$source:expr] @rest) => {};

    // parse mutability
    (@from [$source:expr] @rest mut $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [mut]
            @rest $($rest)*
        }
    };
    (@from [$source:expr] @rest $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut []
            @rest $($rest)*
        }
    };

    // parse variable pattern
    (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [$($mut)*]
            @var $var
            @kind []
            @rest $($rest)*
        }
    };

    // parse kind (type)
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
        let $($mut)* $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)*] @rest);
        $crate::input!(@from [$source] @rest $($rest)*);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
    };

    (from $source:expr, $($rest:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut s = $source;
        $crate::input! {
            @from [&mut s]
            @rest $($rest)*
        }
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().expect(concat!(
            "failed to lock the stdin; please re-run this program.  ",
            "If this issue repeatedly occur, this is a bug in `proconio`.  ",
            "Please report this issue from ",
            "<https://github.com/statiolake/proconio-rs/issues>."
        ));
        $crate::input! {
            @from [&mut *locked_stdin]
            @rest $($rest)*
        }
        drop(locked_stdin); // release the lock
    };
}

/// read input from stdin interactively.
///
/// this macro is alias of:
/// ```text
/// let source = procontio::source::line::LineSource::new(BufReader::new(std::io::stdin()))
/// input! {
///     from &mut source,
///     (mut) variable: type,
///     ...
/// }
/// ```
/// read the documet of [input!](input) for further information.
#[macro_export]
macro_rules! input_interactive {
    ($($rest:tt)*) => {
        unsafe { $crate::__INTERACTIVE = true; }
        $crate::input! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    // array and variable length array
    (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
        $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
        let len = <usize as $crate::__Readable>::read($source);
        $crate::read_value!(@source [$source] @kind [[$($kind)*; len]])
    }};
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest ; $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)*] @len [$($rest)*])
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)* $tt] @rest $($rest)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
        let len = $($len)*;
        (0..len)
            .map(|_| $crate::read_value!(@source [$source] @kind [$($kind)*]))
            .collect::<Vec<_>>()
    }};

    // tuple
    (@source [$source:expr] @kind [($($kinds:tt)*)]) => {
        $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
    };
    (@tuple @source [$source:expr] @kinds [$([$($kind:tt)*])*] @current [] @rest) => {
        (
            $($crate::read_value!(@source [$source] @kind [$($kind)*]),)*
        )
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest, $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest $($rest)*)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)*] @current [$($curr)* $tt] @rest $($rest)*)
    };

    // unreachable
    (@source [$source:expr] @kind []) => {
        compile_error!(concat!("Reached unreachable statement while parsing macro input.  ", "This is a bug in `proconio`.  ", "Please report this issue from ", "<https://github.com/statiolake/proconio-rs/issues>."));
    };

    // normal other
    (@source [$source:expr] @kind [$kind:ty]) => {
        <$kind as $crate::Readable>::read($source)
    }
}

/// Checks if some of tokens are left on stdin.
///
/// This is useful when the number of test cases is not specified like ICPC problems.
///
/// ```text
/// loop {
///     if is_stdin_empty() {
///         break;
///     }
///
///     // do the normal logic
///     input! { ... }
/// }
/// ```
pub fn is_stdin_empty() -> bool {
    let mut lock = STDIN_SOURCE.lock().expect(concat!(
        "failed to lock the stdin; please re-run this program.  ",
        "If this issue repeatedly occur, this is a bug in `proconio`.  ",
        "Please report this issue from ",
        "<https://github.com/statiolake/proconio-rs/issues>."
    ));
    lock.is_empty()
}

// #[fastout]
fn main() {
    input_interactive! {
        n: usize,
    }

    let mut used = vec![false; 2 * n + 1];
    let mut cur = 0usize;

    loop {
        while used[cur] {
            cur += 1;
        }
        used[cur] = true;
        println!("{}", cur + 1);
        input_interactive! {
            x: usize,
        }
        if x == 0 {
            break;
        }
        used[x - 1] = true;
    }
}
