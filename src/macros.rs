#[macro_export]
macro_rules! parse_number {
    ($table: expr) => ({
        use ::dynamic::SPEC_PRIORITY;
        use ::dynamic::SPEC_PRIORITY_NAME;
        if let Some(ref value) = $table.get(SPEC_PRIORITY_NAME) {
            value.as_integer()
        } else {
            Some(SPEC_PRIORITY)
        }
    });
}

#[macro_export]
macro_rules! format_subneko {
    ($slf: expr, $msg: expr) => ({
        <T as io::Write>::write(&mut $slf.shell, b"\n");
        <T as io::Write>::write(&mut $slf.shell, $msg);
    });
    ($slf: expr, $arg: expr, $verb: expr, $command: expr) => ({
        use std::error::Error;
        let output: String = match $command {
            Ok(_) => format!("{}ed with success.", $verb),
            Err(why) => format!("Can't {} \"{}\" because: {}.", $verb, $arg, why.description()),
        };
        format_subneko!($slf, output.as_ref());
    });
}

#[macro_export]
macro_rules! format_subneko_err {
    ($slf: expr, $arg: expr, $verb: expr, $why: expr) => ({
        use std::error::Error;
        let output: String =
            format!("Can't {} \"{}\" because: {}.", $verb, $arg, $why.description());
        format_subneko!($slf, output.as_ref());
    });
}

#[macro_export]
macro_rules! only_rep {
    ($sub: expr) => ({
        let lib: &OsStr = $sub.as_ref();
        unsafe {
            let name: &str = lib.to_str().unwrap_or_default();

            name.slice_unchecked(
                name.find('@').and_then(|index|
                                        Some(index+1)
                ).unwrap_or_default(),
                name.len()
            )
        }
    });
}

#[macro_export]
macro_rules! account_at_rep {
    ($start: expr) => ({
        use std::ops::BitOr;
        if let (Some(middle), true) = (
            $start.rfind('/'),
            $start.ends_with(".git")
        ) {
            let right = $start.len()-4;
            $start.split_at(middle).0.rfind(|c: char| c.eq(&':').bitor(c.eq(&'/'))
            ).and_then(|left| unsafe {
                let all = $start.slice_unchecked(left+1, right)
                                .to_lowercase()
                                .replace("/", "@");
                let left = $start.slice_unchecked(left+1, middle).to_lowercase();
                let right = $start.slice_unchecked(middle+1, right).to_lowercase();
                if left.is_empty().bitor(right.is_empty()) {
                    None
                } else {
                    Some(all)
                }
            })
        } else {
            None
        }
    });
}
