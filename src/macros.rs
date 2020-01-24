macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(unix, feature = "_docs"))]
            #[cfg_attr(feature = "_docs", doc(cfg(unix)))]
            $item
        )*
    }
}
