macro_rules! to_angles {
    {@impl [$($out:tt)*],} => {
        $($out)*
    };

    {@impl [$($out:tt)*], [<[$($head:tt)*]>] $($tail:tt)*} => {
        to_angles!{
            @impl
            [$($out)* <$($head)*>],
            $($tail)*
        }
    };

    {@impl [$($out:tt)*], [ $($head:tt)* ] $($tail:tt)*} => {
        to_angles!{
            @impl
            [$($out)* [$($head)*]],
            $($tail)*
        }
    };

    {@impl [$($out:tt)*], ( $($head:tt)* ) $($tail:tt)*} => {
        to_angles!{
            @impl
            [$($out)* ($($head)*)],
            $($tail)*
        }
    };

    {@impl [$($out:tt)*], { $($head:tt)* } $($tail:tt)*} => {
        to_angles!{
            @impl
            [$($out)* {$($head)*}],
            $($tail)*
        }
    };

    {@impl [$($out:tt)*], $head:tt $($tail:tt)*} => {
        to_angles!{
            @impl
            [$($out)* $head],
            $($tail)*
        }
    };

    {$($e:tt)*} => {
        to_angles!{@impl [], $($e)*}
    };
}

to_angles! {
    struct Token [<[T]>] {
        data: T,
    }
}

pub fn use_macro() {
    let x: to_angles!(Vec[<[i32]>]) = vec![];
}