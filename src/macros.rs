macro_rules! cfg_futures {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "futures")]
            #[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
            $item
        )*
    }
}

macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(unix, docsrs))]
            #[cfg_attr(docsrs, doc(cfg(unix)))]
            $item
        )*
    }
}
